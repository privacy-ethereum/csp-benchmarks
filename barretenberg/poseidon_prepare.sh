#!/usr/bin/env bash
set -euo pipefail

: "${UTILS_BIN:?UTILS_BIN is required}"
: "${INPUT_SIZE:?INPUT_SIZE is required}"
: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_ROOT_PATH="${SCRIPT_DIR}/circuits"

# Update circuit to use INPUT_SIZE
CIRCUIT_SOURCE="${WORKSPACE_ROOT_PATH}/hash/poseidon/src/main.nr"
if [[ -f "$CIRCUIT_SOURCE" ]]; then
  # Replace hash_N function name and array size
  sed -E -i.bak \
    -e "s/hash_[0-9]+/hash_${INPUT_SIZE}/g" \
    -e "s/\[Field;[[:space:]]*[0-9]+\]/[Field; ${INPUT_SIZE}]/g" \
    "$CIRCUIT_SOURCE"
  rm -f "${CIRCUIT_SOURCE}.bak"
else
  echo "Error: Circuit source file not found: $CIRCUIT_SOURCE" >&2
  exit 1
fi

cd "$WORKSPACE_ROOT_PATH"
nargo compile --workspace --silence-warnings --skip-brillig-constraints-check
cd ../..

CIRCUIT_PATH="${WORKSPACE_ROOT_PATH}/target/poseidon.json"
CIRCUIT_MEMBER_DIR="${WORKSPACE_ROOT_PATH}/hash/poseidon"
TOML_PATH="${CIRCUIT_MEMBER_DIR}/Prover_${INPUT_SIZE}.toml"

GEN="$("$UTILS_BIN" poseidon -n ${INPUT_SIZE})"
mapfile -t field_elements < <(printf "%s\n" "$GEN")

{
  printf "inputs = ["
  for ((i = 0; i < INPUT_SIZE; i++)); do
    v=${field_elements[i]}
    if (( i == 0 )); then
      printf "\"%s\"" "$v"
    else
      printf ", \"%s\"" "$v"
    fi
  done
  printf "]\n"
} > "$TOML_PATH"

JQ_PROG='{"workspace-root-path":$workspace, "circuit-path":$circuit, "toml-path":$toml, "input-size":$len, "benchmark-name":$bench}'

jq -nc \
  --arg workspace "$WORKSPACE_ROOT_PATH" \
  --arg circuit "$CIRCUIT_PATH" \
  --arg toml "$TOML_PATH" \
  --argjson len "$INPUT_SIZE" \
  --arg bench "poseidon" \
  "$JQ_PROG" > "$STATE_JSON"
