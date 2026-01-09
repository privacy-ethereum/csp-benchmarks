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

CIRCUIT_PATH="${WORKSPACE_ROOT_PATH}/target/ecdsa.json"

####   Generate input(Prover.toml)   ####
GEN="$("$UTILS_BIN" ecdsa)"
DIGEST="$(printf "%s\n" "$GEN" | sed -n '1p')"
PUB_KEY_X="$(printf "%s\n" "$GEN" | sed -n '2p')"
PUB_KEY_Y="$(printf "%s\n" "$GEN" | sed -n '3p')"
SIGNATURE="$(printf "%s\n" "$GEN" | sed -n '4p')"

if [[ -z "$DIGEST" || -z "$PUB_KEY_X" || -z "$PUB_KEY_Y" || -z "$SIGNATURE" ]]; then
  echo "prepare.sh: generator output malformed" >&2
  exit 2
fi

# Function: Convert hex string to array of decimal bytes
hex_to_bytes_string() {
  local hex_input="$1"
  local hex_clean byte_vals expected_bytes parsed_len

  # Clean: remove whitespace/newlines
  hex_clean="${hex_input//$'\n'/}"
  hex_clean="${hex_clean//$'\r'/}"
  hex_clean="${hex_clean//[[:space:]]/}"

  # Check even length
  if (( ${#hex_clean} % 2 != 0 )); then
    echo "Error: cleaned hex string has odd length (${#hex_clean})" >&2
    return 1
  fi

  expected_bytes=$(( ${#hex_clean} / 2 ))

  # Use xxd if available
  if command -v xxd >/dev/null 2>&1; then
    mapfile -t byte_vals < <(
      echo -n "$hex_clean" \
        | xxd -r -p \
        | od -An -vt u1 \
        | tr -s ' ' '\n' \
        | sed '/^\s*$/d'
    )
  else
    # Fallback
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
  if (( parsed_len != expected_bytes )); then
    echo "Warning: parsed length ($parsed_len) != expected ($expected_bytes)" >&2
  fi

  # Return values: print joined by space (can be captured)
  echo "${byte_vals[@]}"
}

DIGEST_BYTES="$(hex_to_bytes_string "$DIGEST")"
PUB_KEY_X_BYTES="$(hex_to_bytes_string "$PUB_KEY_X")"
PUB_KEY_Y_BYTES="$(hex_to_bytes_string "$PUB_KEY_Y")"
SIGNATURE_BYTES="$(hex_to_bytes_string "$SIGNATURE")"

# Build TOML: input array and input_len

# Determine path
CIRCUIT_MEMBER_DIR="${WORKSPACE_ROOT_PATH}/ecdsa"
TOML_PATH="${CIRCUIT_MEMBER_DIR}/Prover.toml"

# Function: print array string "v0 v1 v2 ..." as "label = [v0, v1, v2, …]" 
print_array_from_string() {
  local label="$1"
  local num_string="$2"
  local arr i v

  # Split the space-separated string into an array
  # (default IFS includes space/tab/newline) :contentReference[oaicite:0]{index=0}
  arr=( $num_string )

  printf "%s = [" "$label"
  for i in "${!arr[@]}"; do
    v="${arr[i]}"
    if (( i == 0 )); then
      printf "%s" "$v"
    else
      printf ", %s" "$v"
    fi
  done
  printf "]\n"
}

# Usage
{
  print_array_from_string "public_key_x"    "$PUB_KEY_X_BYTES"
  print_array_from_string "public_key_y"    "$PUB_KEY_Y_BYTES"
  print_array_from_string "signature"       "$SIGNATURE_BYTES"
  print_array_from_string "message_hash"    "$DIGEST_BYTES"
} > "$TOML_PATH"

####    Create STATE JSON    ####
JQ_PROG='{"workspace-root-path":$workspace, "circuit-path":$circuit, "toml-path":$toml, "benchmark-name":$bench}'

jq -nc \
  --arg workspace "$WORKSPACE_ROOT_PATH" \
  --arg circuit "$CIRCUIT_PATH" \
  --arg toml "$TOML_PATH" \
  --arg bench "ecdsa" \
  "$JQ_PROG" > "$STATE_JSON"
