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

# Use shared common helper to write sizes and update constraints
. "$SCRIPT_DIR/_measure_common.sh"
bb_write_sizes_and_constraints "ecdsa" "ecdsa.json" "$STATE_JSON" "$OUT_JSON" "$SCRIPT_DIR"
