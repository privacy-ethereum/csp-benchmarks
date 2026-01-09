#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments

: "${STATE_JSON:?STATE_JSON is required}"

WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON")

cd "$WORKSPACE_ROOT_PATH"

PROOF_PATH="${WORKSPACE_ROOT_PATH}/target/proof"
VK_PATH="${WORKSPACE_ROOT_PATH}/target/vk"
bb verify -p "$PROOF_PATH" -vk "$VK_PATH"

cd ../..
