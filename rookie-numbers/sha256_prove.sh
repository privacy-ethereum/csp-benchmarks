#!/usr/bin/env bash
# Run SHA256 Stwo prover for csp-benchmarks
set -euo pipefail

# Required env vars:
# - STATE_JSON: path to state JSON from prepare

: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/repo/target/release/sha256_prover"

if [[ ! -f "$BINARY" ]]; then
    echo "Error: Binary not found. Run sha256_prepare.sh first." >&2
    exit 1
fi

exec "$BINARY" prove --state-json "$STATE_JSON"
