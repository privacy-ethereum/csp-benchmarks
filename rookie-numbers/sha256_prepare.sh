#!/usr/bin/env bash
# Prepare SHA256 Stwo prover state for csp-benchmarks
set -euo pipefail

# Required env vars:
# - INPUT_SIZE: input size in bytes
# - STATE_JSON: output JSON file path

: "${INPUT_SIZE:?INPUT_SIZE is required}"
: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$SCRIPT_DIR/repo"
BINARY="$REPO_DIR/target/release/sha256_prover"

# Build if not present
if [[ ! -f "$BINARY" ]]; then
    echo "Building sha256_prover..."
    cargo build --release -p sha256 --bin sha256_prover --manifest-path "$REPO_DIR/Cargo.toml"
fi

exec "$BINARY" prepare --input-size "$INPUT_SIZE" --state-json "$STATE_JSON"
