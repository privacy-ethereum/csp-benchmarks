#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments

: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

ADAPTED_STWO_BIN=$(jq -r '."adapted-stwo-bin"' "$STATE_JSON")
TARGET_DIR=$(jq -r '."target-dir"' "$STATE_JSON")

PUB_INPUT="${TARGET_DIR}/air_public_inputs.json"
PRIV_INPUT="${TARGET_DIR}/air_private_inputs.json"
PROOF_FILE="${TARGET_DIR}/proof.json"

# Change to workspace root
cd "$SCRIPT_DIR"

# Step 2: Stwo verifying
"$ADAPTED_STWO_BIN" \
  --pub_json $PUB_INPUT \
  --priv_json $PRIV_INPUT \
  --proof_path $PROOF_FILE \
  --verify

cd ..
