#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments

: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

ADAPTED_STWO_BIN=$(jq -r '."adapted-stwo-bin"' "$STATE_JSON")
COMPILED_CIRCUIT_PATH=$(jq -r '."compiled-circuit-path"' "$STATE_JSON")
INPUT_JSON=$(jq -r '."input-json-path"' "$STATE_JSON")
TARGET_DIR=$(jq -r '."target-dir"' "$STATE_JSON")

TRACE_FILE="${TARGET_DIR}/trace.bin"
MEMORY_FILE="${TARGET_DIR}/memory.bin"
PUB_INPUT="${TARGET_DIR}/air_public_inputs.json"
PRIV_INPUT="${TARGET_DIR}/air_private_inputs.json"
PROOF_FILE="${TARGET_DIR}/proof.json"

# Change to workspace root
cd "$SCRIPT_DIR"

# Step 1: Run the program and generate execution trace
echo "Running the Cairo program..."
cairo-run --program=$COMPILED_CIRCUIT_PATH \
          --program_input=$INPUT_JSON \
          --layout=starknet \
          --trace_file=$TRACE_FILE \
          --memory_file=$MEMORY_FILE \
          --air_public_input=$PUB_INPUT \
          --air_private_input=$PRIV_INPUT \
          --proof_mode


# Step 2: Stwo proving
RUST_BACKTRACE=1 "$ADAPTED_STWO_BIN" \
  --pub_json $PUB_INPUT \
  --priv_json $PRIV_INPUT \
  --proof_path $PROOF_FILE

cd ..
