#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments

: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$SCRIPT_DIR/ligero-prover/build/webgpu_verifier" "$(cat "$STATE_JSON")"


