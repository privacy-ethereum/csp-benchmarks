#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - UTILS_BIN: path to utils binary
# - STATE_JSON: output JSON file path
# Note: ECDSA is a fixed-size benchmark, INPUT_SIZE is ignored

: "${UTILS_BIN:?UTILS_BIN is required}"
: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROGRAM_PATH="${SCRIPT_DIR}/ligero-prover/sdk/cpp/build/examples/ecdsa_p256_verify_prehashed.wasm"
SHADER_PATH="${SCRIPT_DIR}/ligero-prover/shader"

# Get ECDSA components: digest, pub_key_x, pub_key_y, signature
ECDSA_OUT="$("$UTILS_BIN" ecdsa)"
DIGEST="$(printf "%s\n" "$ECDSA_OUT" | sed -n '1p')"
PUB_KEY_X="$(printf "%s\n" "$ECDSA_OUT" | sed -n '2p')"
PUB_KEY_Y="$(printf "%s\n" "$ECDSA_OUT" | sed -n '3p')"
SIGNATURE="$(printf "%s\n" "$ECDSA_OUT" | sed -n '4p')"

# Concatenate public key coordinates (x || y)
PUBKEY="${PUB_KEY_X}${PUB_KEY_Y}"

if [[ -z "$DIGEST" || -z "$SIGNATURE" || -z "$PUBKEY" ]]; then
  echo "ecdsa_prepare.sh: generator output malformed" >&2
  exit 2
fi

# Build JSON for Ligetron prover
# Args order matches ecdsa_p256_verify_prehashed.cpp: msg_hash, signature, pubkey
JQ_PROG='{program:$prog, "shader-path":$shader, packing:16384, "private-indices":[1,2,3], args:[{hex:$hash},{hex:$sig},{hex:$pub}]}'

jq -nc \
  --arg prog "$PROGRAM_PATH" \
  --arg shader "$SHADER_PATH" \
  --arg hash "0x$DIGEST" \
  --arg sig "0x$SIGNATURE" \
  --arg pub "0x$PUBKEY" \
  "$JQ_PROG" > "$STATE_JSON"
