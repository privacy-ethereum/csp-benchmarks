#!/usr/bin/env bash
set -euo pipefail

# Required env vars:
# - UTILS_BIN: path to utils binary
# - INPUT_SIZE: input size in bytes
# - STATE_JSON: output JSON file path

: "${UTILS_BIN:?UTILS_BIN is required}"
: "${INPUT_SIZE:?INPUT_SIZE is required}"
: "${STATE_JSON:?STATE_JSON is required}"

####   Compile circuits   ####
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_ROOT_PATH="${SCRIPT_DIR}/circuits"

# Update circuit to use INPUT_SIZE for the input parameter only
CIRCUIT_SOURCE="${WORKSPACE_ROOT_PATH}/hash/keccak256/src/main.nr"
if [[ -f "$CIRCUIT_SOURCE" ]]; then
  # Replace the input array length and the keccak generic arity in `fn main(...)`
  sed -E -i.bak \
    -e "s/(fn[[:space:]]+main\\([[:space:]]*input:[[:space:]]*\\[u8;)[[:space:]]*[0-9]+/\\1 ${INPUT_SIZE}/" \
    -e "s/(keccak256::<)[[:space:]]*[0-9]+/\\1${INPUT_SIZE}/" \
    "$CIRCUIT_SOURCE"
  # Remove backup file if it exists
  rm -f "${CIRCUIT_SOURCE}.bak"
else
  echo "Error: Circuit source file not found: $CIRCUIT_SOURCE" >&2
  exit 1
fi

cd "$WORKSPACE_ROOT_PATH"
nargo compile --workspace --silence-warnings --skip-brillig-constraints-check
cd ../..

CIRCUIT_PATH="${WORKSPACE_ROOT_PATH}/target/keccak.json"

####   Generate input(Prover.toml)   ####
GEN="$("$UTILS_BIN" keccak -n ${INPUT_SIZE})"
MSG="$(printf "%s\n" "$GEN" | sed -n '1p')"
HEX_NO_PREFIX="$(printf "%s\n" "$GEN" | sed -n '2p')"

if [[ -z "$MSG" || -z "$HEX_NO_PREFIX" ]]; then
  echo "prepare.sh: generator output malformed" >&2
  exit 2
fi

# Clean it: remove whitespace/newlines if any
hex_clean="${MSG//$'\n'/}"
hex_clean="${hex_clean//$'\r'/}"
hex_clean="${hex_clean//[[:space:]]/}"

# Check even length
if (( ${#hex_clean} % 2 != 0 )); then
  echo "Error: cleaned hex string has odd length (${#hex_clean})" >&2
  exit 1
fi

expected_bytes=$(( ${#hex_clean} / 2 ))

# We pipe hex → binary → decimal values without capturing raw binary
# Use xxd if available
if command -v xxd >/dev/null 2>&1; then
  # echo hex → xxd reverse to binary → od to decimal bytes
  mapfile -t byte_vals < <(
    echo -n "$hex_clean" \
      | xxd -r -p \
      | od -An -vt u1 \
      | tr -s ' ' '\n' \
      | sed '/^\s*$/d'
  )
else
  # fallback: convert hex to \x escapes then printf, then od
  mapfile -t byte_vals < <(
    echo -n "$hex_clean" \
      | sed 's/../\\x&/g' \
      | xargs printf \
      | od -An -vt u1 \
      | tr -s ' ' '\n' \
      | sed '/^\s*$/d'
  )
fi

parsed_len=${#byte_vals[@]}

# Optionally check lengths match
if (( parsed_len != expected_bytes )); then
  echo "Warning: parsed length ($parsed_len) != expected ($expected_bytes)" >&2
fi

# Build TOML: input array and input_len

# Determine path
CIRCUIT_MEMBER_DIR="${WORKSPACE_ROOT_PATH}/hash/keccak256"
TOML_PATH="${CIRCUIT_MEMBER_DIR}/Prover_${INPUT_SIZE}.toml"

{
  # Print array
  printf "input = ["
  for i in "${!byte_vals[@]}"; do
    v=${byte_vals[i]}
    if (( i == 0 )); then
      printf "%d" "$v"
    else
      printf ", %d" "$v"
    fi
  done
  printf "]\n"

  # Print length
  printf "message_size = %d\n" "$INPUT_SIZE"
} > "$TOML_PATH"

####    Create STATE JSON    ####
JQ_PROG='{"workspace-root-path":$workspace, "circuit-path":$circuit, "toml-path":$toml, "input-size":$len}'

jq -nc \
  --arg workspace "$WORKSPACE_ROOT_PATH" \
  --arg circuit "$CIRCUIT_PATH" \
  --arg toml "$TOML_PATH" \
  --argjson len "$INPUT_SIZE" \
  "$JQ_PROG" > "$STATE_JSON"

