#!/usr/bin/env bash
set -euo pipefail

: "${STATE_JSON:?STATE_JSON is required}"

WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON")
CIRCUIT_PATH=$(jq -r '."circuit-path"' "$STATE_JSON")
TOML_PATH=$(jq -r '."toml-path"' "$STATE_JSON")
INPUT_SIZE=$(jq -r '."input-size"' "$STATE_JSON")

cd "$WORKSPACE_ROOT_PATH"

WITNESS_FILE="poseidon2_${INPUT_SIZE}.gz"
nargo execute --prover-name "$(basename "$TOML_PATH" .toml)" --package "poseidon2" "$WITNESS_FILE"

bb prove -b "$CIRCUIT_PATH" -w "$WORKSPACE_ROOT_PATH/target/$WITNESS_FILE" -o "$WORKSPACE_ROOT_PATH/target/"

cd ../..
