#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to JSON arguments for proving
# - SIZES_JSON: output JSON path (defaults to <dir>/${TARGET}_<INPUT_SIZE>_sizes.json when run via benchmark.sh)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OUT_JSON="${SIZES_JSON:-}"

: "${STATE_JSON:?STATE_JSON is required}"
: "${SIZES_JSON:?SIZES_JSON is required}"

# Use shared common helper
. "$SCRIPT_DIR/_measure_common.sh"

# Clear CRS cache
bb_clear_crs

# Run one proving cycle to generate artifacts for measurement
"$SCRIPT_DIR/keccak_prove.sh" >/dev/null 2>&1 || true

# Measure CRS size after proving
CRS_SIZE=$(bb_measure_crs_size)

bb_write_sizes_and_constraints "keccak" "keccak.json" "$STATE_JSON" "$OUT_JSON" "$SCRIPT_DIR" "$CRS_SIZE"
