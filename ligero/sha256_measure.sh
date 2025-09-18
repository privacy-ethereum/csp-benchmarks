#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments for proving
# Optional env vars:
# - SIZES_JSON: output JSON path (defaults to <dir>/${TARGET}_<INPUT_SIZE>_sizes.json when run via benchmark.sh)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUT_JSON="${SIZES_JSON:-}"

: "${STATE_JSON:?STATE_JSON is required}"

# Run one proving cycle to generate artifacts for measurement
"$SCRIPT_DIR/sha256_prove.sh" >/dev/null 2>&1 || true

# In Ligetron, the prover writes a proof file named proof.data in the current working directory.
CANDIDATES=(
  "${PWD}/proof.data"
)

proof_path=""
for p in "${CANDIDATES[@]}"; do
  if [[ -f "$p" ]]; then
    proof_path="$p"
    break
  fi
done

if [[ -z "$proof_path" ]]; then
  echo "proof.data not found for Ligero size measurement" >&2
  exit 1
fi

proof_size_bytes=$(stat -f %z "$proof_path" 2>/dev/null || stat -c %s "$proof_path")

# Preprocessing artifacts: WASM used by the prover and the prover binary itself
WASM_PATH="${SCRIPT_DIR}/third_party/ligetron/sdk/build/examples/sha256.wasm"
PROVER_BIN_PATH="${SCRIPT_DIR}/third_party/ligetron/build/webgpu_prover"
wasm_size=$(stat -f %z "$WASM_PATH" 2>/dev/null || stat -c %s "$WASM_PATH")
prover_size=$(stat -f %z "$PROVER_BIN_PATH" 2>/dev/null || stat -c %s "$PROVER_BIN_PATH")
preprocessing_size_bytes=$(( wasm_size + prover_size ))

json_output=$(jq -n \
  --argjson proof_size "$proof_size_bytes" \
  --argjson preprocessing_size "$preprocessing_size_bytes" \
  '{proof_size: $proof_size, preprocessing_size: $preprocessing_size}'
)

if [[ -n "$OUT_JSON" ]]; then
  echo "$json_output" > "$OUT_JSON"
  jq . "$OUT_JSON" || true
else
  echo "$json_output"
fi


