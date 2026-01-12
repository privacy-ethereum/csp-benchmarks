#!/usr/bin/env bash
set -euo pipefail

# Generates proof and verification key for verifier benchmarking.
: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WRITE_VK=1 STATE_JSON="$STATE_JSON" bash "$SCRIPT_DIR/prove.sh"
