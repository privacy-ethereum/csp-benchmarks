#!/usr/bin/env bash
set -euo pipefail

# Requires a BLAKE3 proof prepared through blake3_prepare.sh and prove.sh.
: "${STATE_JSON:?STATE_JSON is required}"

WORKSPACE_ROOT_PATH=$(jq -r '."workspace-root-path"' "$STATE_JSON")
PROOF_PATH="${WORKSPACE_ROOT_PATH}/target/proof"
VK_PATH="${WORKSPACE_ROOT_PATH}/target/vk"
PUBLIC_INPUTS_PATH="${WORKSPACE_ROOT_PATH}/target/public_inputs"
MUTATED_PUBLIC_INPUTS=$(mktemp "${TMPDIR:-/tmp}/barretenberg-blake3-public-inputs.XXXXXX")
trap 'rm -f "$MUTATED_PUBLIC_INPUTS"' EXIT

bb verify -p "$PROOF_PATH" -vk "$VK_PATH" -i "$PUBLIC_INPUTS_PATH"

python3 - "$PUBLIC_INPUTS_PATH" "$MUTATED_PUBLIC_INPUTS" <<'PY'
from pathlib import Path
import sys

source, destination = map(Path, sys.argv[1:])
public_inputs = bytearray(source.read_bytes())
if len(public_inputs) < 32:
    raise SystemExit("public input file is too short")

# Each public u8 is encoded as a 32-byte field element. Mutate the low byte of
# the first public digest element while keeping it a canonical small value.
public_inputs[31] ^= 1
destination.write_bytes(public_inputs)
PY

if bb verify -p "$PROOF_PATH" -vk "$VK_PATH" -i "$MUTATED_PUBLIC_INPUTS"; then
  echo "mutated BLAKE3 public digest was accepted" >&2
  exit 1
fi

echo "mutated BLAKE3 public digest rejected"
