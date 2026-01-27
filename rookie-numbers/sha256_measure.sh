#!/usr/bin/env bash
# Report SHA256 Stwo proof sizes for csp-benchmarks
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to state JSON from prepare
# - SIZES_JSON: output path for sizes JSON

: "${STATE_JSON:?STATE_JSON is required}"
: "${SIZES_JSON:?SIZES_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/repo/target/release/sha256_prover"

if [[ ! -f "$BINARY" ]]; then
    echo "Error: Binary not found. Run sha256_prepare.sh first." >&2
    exit 1
fi

# Run prove first to generate the proof (required for measure)
"$BINARY" prove --state-json "$STATE_JSON" >/dev/null 2>&1 || true

# Run measure command
"$BINARY" measure --state-json "$STATE_JSON" --sizes-json "$SIZES_JSON"

# === Update circuit_sizes.json ===
# Extract log_size and input_size from state JSON
LOG_SIZE=$(jq -r '.log_size' "$STATE_JSON")
INPUT_SIZE=$(jq -r '.input_size' "$STATE_JSON")

# For Stwo AIR, circuit size is the trace length: 2^log_size SHA256 instances
# Each instance requires 64 rounds, so total trace rows = 64 * 2^log_size
CIRCUIT_SIZE=$((64 * (1 << LOG_SIZE)))

# Update circuit_sizes.json
CIRCUIT_SIZES_PATH="${SCRIPT_DIR}/circuit_sizes.json"
if [[ -f "$CIRCUIT_SIZES_PATH" ]]; then
    UPDATED_JSON=$(jq \
        --arg size_key "$INPUT_SIZE" \
        --argjson size_val "$CIRCUIT_SIZE" \
        '.sha256[$size_key] = $size_val' \
        "$CIRCUIT_SIZES_PATH")
else
    UPDATED_JSON=$(jq -n \
        --arg size_key "$INPUT_SIZE" \
        --argjson size_val "$CIRCUIT_SIZE" \
        '{sha256: {($size_key): $size_val}}')
fi

printf "%s\n" "$UPDATED_JSON" > "$CIRCUIT_SIZES_PATH"
