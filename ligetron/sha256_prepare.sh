#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - UTILS_BIN: path to utils binary
# - INPUT_SIZE: input size in bytes
# - STATE_JSON: output JSON file path

: "${UTILS_BIN:?UTILS_BIN is required}"
: "${INPUT_SIZE:?INPUT_SIZE is required}"
: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROGRAM_PATH="${SCRIPT_DIR}/ligero-prover/sdk/build/examples/sha256.wasm"
SHADER_PATH="${SCRIPT_DIR}/ligero-prover/shader"

GEN="$("$UTILS_BIN" sha256 -n "$INPUT_SIZE")"
MSG="$(printf "%s\n" "$GEN" | sed -n '1p')"
HEX_NO_PREFIX="$(printf "%s\n" "$GEN" | sed -n '2p')"

if [[ -z "$MSG" || -z "$HEX_NO_PREFIX" ]]; then
  echo "prepare.sh: generator output malformed" >&2
  exit 2
fi

JQ_PROG='{program:$prog, "shader-path":$shader, packing:8192, "private-indices":[1], args:[{hex:$msg},{i64:$len},{hex:$dig}]}'

jq -nc \
  --arg prog "$PROGRAM_PATH" \
  --arg shader "$SHADER_PATH" \
  --arg msg "$MSG" \
  --arg dig "0x$HEX_NO_PREFIX" \
  --argjson len "$INPUT_SIZE" \
  "$JQ_PROG" > "$STATE_JSON"


