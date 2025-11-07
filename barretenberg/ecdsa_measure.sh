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
"$SCRIPT_DIR/ecdsa_prove.sh" >/dev/null 2>&1 || true

# In Noir + Barretenberg(bb), the prover writes a proof file named proof in the "target" directory.
WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON")
proof_path="${WORKSPACE_ROOT_PATH}/target/proof"

if [[ ! -f "$proof_path" ]]; then
  echo "proof not found for Noir size measurement" >&2
  exit 1
fi

proof_size_bytes=$(stat -f %z "$proof_path" 2>/dev/null || stat -c %s "$proof_path")

# Preprocessing artifacts: circuit.json file
CIRCUIT_PATH=$(jq -r '."circuit-path"' "$STATE_JSON")
circuit_size=$(stat -f %z "$CIRCUIT_PATH" 2>/dev/null || stat -c %s "$CIRCUIT_PATH")
preprocessing_size_bytes=$(( circuit_size ))

json_output=$(jq -n \
  --argjson proof_size "$proof_size_bytes" \
  --argjson preprocessing_size "$preprocessing_size_bytes" \
  '{proof_size: $proof_size, preprocessing_size: $preprocessing_size}'
)

echo "$json_output" > "$OUT_JSON"
jq . "$OUT_JSON" || true

# === Compute and update circuit_sizes.json with Circuit size (Barretenberg ECDSA) ===
if command -v noir-profiler >/dev/null 2>&1; then
  WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON")

  # Derive INPUT_SIZE label from the SIZES_JSON filename: e.g., ecdsa_<SIZE>_sizes.json
  SIZE_LABEL=$(basename "$OUT_JSON" | sed -E 's/^ecdsa_([^_]+)_sizes\.json$/\1/')

  CIRCUIT_SIZE=$(
    cd "$WORKSPACE_ROOT_PATH" && \
    { noir-profiler gates \
        --artifact-path ./target/ecdsa.json \
        --backend-path bb \
        --output ./target \
        -- --include_gates_per_opcode 2>&1 || true; } \
      | sed -n 's/.*Circuit size: \([0-9][0-9]*\).*/\1/p' \
      | head -n1
  )

  if [[ -n "$CIRCUIT_SIZE" && -n "$SIZE_LABEL" ]]; then
    SYSTEM_DIR="$(cd "$(dirname "$0")" && pwd)"
    CONSTRAINTS_JSON_PATH="${SYSTEM_DIR}/circuit_sizes.json"

    if [[ -f "$CONSTRAINTS_JSON_PATH" ]]; then
      UPDATED_JSON=$(jq \
        --arg size_key "$SIZE_LABEL" \
        --argjson size_val "$CIRCUIT_SIZE" \
        '.ecdsa[$size_key] = $size_val | . // {ecdsa: {($size_key): $size_val}}' \
        "$CONSTRAINTS_JSON_PATH")
    else
      UPDATED_JSON=$(jq -n \
        --arg size_key "$SIZE_LABEL" \
        --argjson size_val "$CIRCUIT_SIZE" \
        '{ecdsa: {($size_key): $size_val}}')
    fi

    printf "%s\n" "$UPDATED_JSON" > "$CONSTRAINTS_JSON_PATH"
  fi
else
  echo "Warning: noir-profiler not found; skipping ECDSA circuit size generation" >&2
fi
