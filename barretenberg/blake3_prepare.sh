#!/usr/bin/env bash
set -euo pipefail

: "${UTILS_BIN:?UTILS_BIN is required}"
: "${INPUT_SIZE:?INPUT_SIZE is required}"
: "${STATE_JSON:?STATE_JSON is required}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_ROOT_PATH="${SCRIPT_DIR}/circuits"
CIRCUIT_SOURCE="${WORKSPACE_ROOT_PATH}/hash/blake3/src/main.nr"

if (( INPUT_SIZE > 1024 )); then
  # TODO: Remove this exit-77 skip after the pinned Barretenberg native BLAKE3
  # black box supports multi-chunk inputs. The current backend accepts at most
  # one 1024-byte BLAKE3 chunk. A complete 2048-byte hash requires two chunk
  # chaining values with chunk counters 0 and 1, followed by a parent
  # compression over those values with the PARENT and ROOT flags. Calling the
  # existing black box independently on each 1024-byte half is not equivalent:
  # it returns two finalized root digests rather than the two chunk chaining
  # values required by the parent node. Once upstream implements the complete
  # chunk tree, update the Barretenberg pin, remove this guard, restore 2048 to
  # this native path, and validate the result against the reference BLAKE3 hash
  # with a full 2048-byte prove-and-verify run. Exit 77 is reserved by
  # benchmark.sh for an unsupported system/target/input-size measurement.
  echo "Skipping BLAKE3 input size ${INPUT_SIZE}: native black box supports at most 1024 bytes" >&2
  exit 77
fi

sed -E -i.bak \
  -e "s/(fn[[:space:]]+main\([[:space:]]*input:[[:space:]]*\[u8;)[[:space:]]*[0-9]+/\1 ${INPUT_SIZE}/" \
  "$CIRCUIT_SOURCE"
rm -f "${CIRCUIT_SOURCE}.bak"

cd "$WORKSPACE_ROOT_PATH"
nargo compile --workspace --silence-warnings --skip-brillig-constraints-check
cd ../..

CIRCUIT_PATH="${WORKSPACE_ROOT_PATH}/target/blake3.json"
bb write_vk -b "$CIRCUIT_PATH" -o "${WORKSPACE_ROOT_PATH}/target/"

GEN="$("$UTILS_BIN" blake3 -n "${INPUT_SIZE}")"
MSG="$(printf "%s\n" "$GEN" | sed -n '1p')"
DIGEST="$(printf "%s\n" "$GEN" | sed -n '2p')"
if [[ -z "$MSG" || -z "$DIGEST" ]]; then
  echo "blake3_prepare.sh: generator output malformed" >&2
  exit 2
fi

mapfile -t byte_vals < <(
  printf "%s" "$MSG" | xxd -r -p | od -An -vt u1 | tr -s ' ' '\n' | sed '/^[[:space:]]*$/d'
)
if (( ${#byte_vals[@]} != INPUT_SIZE )); then
  echo "BLAKE3 input generator returned ${#byte_vals[@]} bytes, expected ${INPUT_SIZE}" >&2
  exit 1
fi

CIRCUIT_MEMBER_DIR="${WORKSPACE_ROOT_PATH}/hash/blake3"
TOML_PATH="${CIRCUIT_MEMBER_DIR}/Prover_${INPUT_SIZE}.toml"
{
  printf "input = ["
  for i in "${!byte_vals[@]}"; do
    if (( i == 0 )); then
      printf "%d" "${byte_vals[i]}"
    else
      printf ", %d" "${byte_vals[i]}"
    fi
  done
  printf "]\n"
} > "$TOML_PATH"

jq -nc \
  --arg workspace "$WORKSPACE_ROOT_PATH" \
  --arg circuit "$CIRCUIT_PATH" \
  --arg toml "$TOML_PATH" \
  --argjson len "$INPUT_SIZE" \
  --arg bench "blake3" \
  '{"workspace-root-path":$workspace, "circuit-path":$circuit, "toml-path":$toml, "input-size":$len, "benchmark-name":$bench}' \
  > "$STATE_JSON"
