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

cd "$WORKSPACE_ROOT_PATH"
nargo compile --workspace --silence-warnings --skip-brillig-constraints-check
cd ../..

CIRCUIT_PATH="${WORKSPACE_ROOT_PATH}/target/sha256.json"

####   Generate input(Prover.toml)   ####
GEN="$("$UTILS_BIN" sha256 -n 2048)"
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
CIRCUIT_MEMBER_DIR="${WORKSPACE_ROOT_PATH}/hash/sha256"
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
  printf "input_len = %d\n" "$INPUT_SIZE"
} > "$TOML_PATH"

####    Create STATE JSON    ####
JQ_PROG='{"workspace-root-path":$workspace, "circuit-path":$circuit, "toml-path":$toml, "input-size":$len}'

jq -nc \
  --arg workspace "$WORKSPACE_ROOT_PATH" \
  --arg circuit "$CIRCUIT_PATH" \
  --arg toml "$TOML_PATH" \
  --argjson len "$INPUT_SIZE" \
  "$JQ_PROG" > "$STATE_JSON"
