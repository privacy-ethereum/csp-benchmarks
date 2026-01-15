#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments for proving
# - SIZES_JSON: output JSON path (defaults to <dir>/${TARGET}_<INPUT_SIZE>_sizes.json when run via benchmark.sh)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUT_JSON="${SIZES_JSON:-}"

: "${STATE_JSON:?STATE_JSON is required}"
: "${SIZES_JSON:?SIZES_JSON is required}"

# Run one proving cycle to generate artifacts for measurement
"$SCRIPT_DIR/sha256_prove.sh" >/dev/null 2>&1 || true

# In Ligetron, the prover writes a proof file named proof.data in the current working directory.
proof_path="${PWD}/proof.data"

if [[ ! -f "$proof_path" ]]; then
  echo "proof.data not found for Ligero size measurement" >&2
  exit 1
fi

proof_size_bytes=$(stat -f %z "$proof_path" 2>/dev/null || stat -c %s "$proof_path")

# Preprocessing artifacts: WASM used by the prover and the prover binary itself
WASM_PATH="${SCRIPT_DIR}/ligero-prover/sdk/build/examples/sha256.wasm"
wasm_size=$(stat -f %z "$WASM_PATH" 2>/dev/null || stat -c %s "$WASM_PATH")
preprocessing_size_bytes=$(( wasm_size ))

json_output=$(jq -n \
  --argjson proof_size "$proof_size_bytes" \
  --argjson preprocessing_size "$preprocessing_size_bytes" \
  '{proof_size: $proof_size, preprocessing_size: $preprocessing_size}'
)

echo "$json_output" > "$OUT_JSON"
jq . "$OUT_JSON" || true



# === Compute and update circuit_sizes.json (Ligetron) ===
# Perform a dummy proving run to capture constraint counts from Stage 1
PROVER_BIN="${SCRIPT_DIR}/ligero-prover/build/webgpu_prover"
if [[ -x "$PROVER_BIN" ]]; then
  DUMMY_OUTPUT="$($PROVER_BIN "$(cat "$STATE_JSON")" 2>&1 || true)"

  # Extract first-stage linear and quadratic counts and sum them
  CONSTRAINTS_SUM=$(printf "%s\n" "$DUMMY_OUTPUT" | awk '
    /^Start Stage 1/ { in_s1=1; next }
    in_s1 && /Num Linear constraints:/ { if (match($0, /[0-9]+/)) lin=substr($0, RSTART, RLENGTH); next }
    in_s1 && /Num quadratic constraints:/ { if (match($0, /[0-9]+/)) quad=substr($0, RSTART, RLENGTH); print lin+quad; exit }
  ')

  if [[ -n "$CONSTRAINTS_SUM" ]]; then
    # Derive input size label from SIZES_JSON filename: sha256_<SIZE>_sizes.json
    SIZE_LABEL=$(basename "$OUT_JSON" | sed -E 's/^sha256_([^_]+)_sizes\.json$/\1/')
    if [[ -n "$SIZE_LABEL" ]]; then
      SYSTEM_DIR="$(cd "$(dirname "$0")" && pwd)"
      CONSTRAINTS_JSON_PATH="${SYSTEM_DIR}/circuit_sizes.json"

      if [[ -f "$CONSTRAINTS_JSON_PATH" ]]; then
        UPDATED_JSON=$(jq \
          --arg size_key "$SIZE_LABEL" \
          --argjson size_val "$CONSTRAINTS_SUM" \
          '.sha256[$size_key] = $size_val | . // {sha256: {($size_key): $size_val}}' \
          "$CONSTRAINTS_JSON_PATH")
      else
        UPDATED_JSON=$(jq -n \
          --arg size_key "$SIZE_LABEL" \
          --argjson size_val "$CONSTRAINTS_SUM" \
          '{sha256: {($size_key): $size_val}}')
      fi

      printf "%s\n" "$UPDATED_JSON" > "$CONSTRAINTS_JSON_PATH"
    fi
  fi
else
  echo "Warning: Ligetron prover binary not found; skipping constraints generation" >&2
fi