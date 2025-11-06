#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments

: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON")
CIRCUIT_PATH=$(jq -r '."circuit-path"' "$STATE_JSON")
TOML_PATH=$(jq -r '."toml-path"' "$STATE_JSON")
INPUT_SIZE=$(jq -r '."input-size"' "$STATE_JSON")

# Change to workspace root
cd "$WORKSPACE_ROOT_PATH"

#### Step 1: Witness generation ####
WITNESS_FILE="sha256_${INPUT_SIZE}.gz"
nargo execute --prover-name $TOML_PATH --package "sha256" $WITNESS_FILE

#### Step 2: bb prove ####
bb prove -b "$CIRCUIT_PATH" -w "$WORKSPACE_ROOT_PATH/target/$WITNESS_FILE" -o "$WORKSPACE_ROOT_PATH/target/"

cd ../..