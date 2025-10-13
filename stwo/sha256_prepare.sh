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

SHA256_CAIRO_CIRCUIT_PATH="${SCRIPT_DIR}/src/main.cairo"
TARGET_DIR="${SCRIPT_DIR}/target"
COMPILED_CIRCUIT_PATH="${TARGET_DIR}/sha256_compiled.json"

cd "$SCRIPT_DIR"

# Create "target" directory
if [ ! -d "$TARGET_DIR" ]; then
  echo "Creating target directory..."
  mkdir -p "$TARGET_DIR"
fi

# Step 0: Activate virtual environment
source .venv/bin/activate

# Step 1: Compile the Cairo program
echo "Compiling sha256 circuit..."
cairo-compile $SHA256_CAIRO_CIRCUIT_PATH --output $COMPILED_CIRCUIT_PATH --proof_mode

# Step 2: Generate input(input.json)
GEN="$("$UTILS_BIN" sha256 -n 2048)"
MSG="$(printf "%s\n" "$GEN" | sed -n '1p')"
HEX_NO_PREFIX="$(printf "%s\n" "$GEN" | sed -n '2p')"

if [[ -z "$MSG" || -z "$HEX_NO_PREFIX" ]]; then
  echo "prepare.sh: generator output malformed" >&2
  exit 2
fi

INPUT_JSON="${TARGET_DIR}/sha256_input_${INPUT_SIZE}.json"
INPUT_CONTENT='{"text":$input}'
jq -nc \
  --arg input "$MSG" \
  "$INPUT_CONTENT" > "$INPUT_JSON"


# Step 3: Prepare stwo-cairo prover & verifier
STWO_REPO="https://github.com/starkware-libs/stwo-cairo.git"
STWO_DIR="${SCRIPT_DIR}/stwo-cairo"

if [ ! -d "$STWO_DIR" ]; then
  echo "Cloning stwo-cairo repository..."
  git clone $STWO_REPO
  git checkout 0cbda4a
fi

PROVER_DIR="${STWO_DIR}/stwo_cairo_prover"
ADAPTED_STWO_BIN="${PROVER_DIR}/target/release/adapted_stwo"
if [ ! -f "$ADAPTED_STWO_BIN" ]; then
  echo "Building stwo-cairo ..."
  cd "$PROVER_DIR"
  RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release --features="std"
  cd -
else
  echo "adapted_stwo binary already exists. Skipping build."
fi

####    Create STATE JSON    ####
JQ_PROG='{"adapted-stwo-bin":$adapted_stwo_bin, "compiled-circuit-path":$compiled, "input-json-path":$input, "target-dir":$target_dir}'

jq -nc \
  --arg adapted_stwo_bin "$ADAPTED_STWO_BIN" \
  --arg compiled "$COMPILED_CIRCUIT_PATH" \
  --arg input "$INPUT_JSON" \
  --arg target_dir "$TARGET_DIR" \
  "$JQ_PROG" > "$STATE_JSON"

cd ..
