#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments containing:
#   - workspace-root-path: path to the workspace
#   - circuit-path: path to the compiled circuit
#   - toml-path: path to the prover TOML file
#   - benchmark-name
#   - input-size (optional)

: "${STATE_JSON:?STATE_JSON is required}"

WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON")
CIRCUIT_PATH=$(jq -r '."circuit-path"' "$STATE_JSON")
TOML_PATH=$(jq -r '."toml-path"' "$STATE_JSON")
BENCHMARK_NAME=$(jq -r '."benchmark-name"' "$STATE_JSON")
INPUT_SIZE=$(jq -r '."input-size" // empty' "$STATE_JSON")

cd "$WORKSPACE_ROOT_PATH"

# Determine witness file name
if [[ -n "$INPUT_SIZE" ]]; then
  WITNESS_FILE="${BENCHMARK_NAME}_${INPUT_SIZE}.gz"
else
  WITNESS_FILE="${BENCHMARK_NAME}.gz"
fi

# Extract prover name from TOML path (filename without .toml extension)
PROVER_NAME="$(basename "$TOML_PATH" .toml)"

# Step 1: Witness generation
nargo execute --prover-name "$PROVER_NAME" --package "$BENCHMARK_NAME" "$WITNESS_FILE"

# Step 2: bb prove (set WRITE_VK=1 to also write verification key)
bb prove -b "$CIRCUIT_PATH" -w "$WORKSPACE_ROOT_PATH/target/$WITNESS_FILE" ${WRITE_VK:+--write_vk} -o "$WORKSPACE_ROOT_PATH/target/"

cd ../..
