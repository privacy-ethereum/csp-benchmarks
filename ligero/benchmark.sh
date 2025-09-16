#!/usr/bin/env bash
set -euo pipefail

# Run Ligetron SHA-256 benchmark
# Usage: benchmark.sh [--ligetron-dir <path>]

LIGETRON_DIR="${LIGETRON_DIR:-}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MEASURE_RAM_SCRIPT="${SCRIPT_DIR}/../measure_mem_avg.sh"
UTILS_BIN="${SCRIPT_DIR}/../target/release/utils"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --ligetron-dir)
      LIGETRON_DIR="$2"; shift 2 ;;
    *)
      echo "Unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [[ -z "${LIGETRON_DIR}" ]]; then
  # default to third_party/ligetron next to this script
  LIGETRON_DIR="${SCRIPT_DIR}/third_party/ligetron"
fi

if [[ ! -d "${LIGETRON_DIR}" ]]; then
  echo "LIGETRON_DIR does not exist: ${LIGETRON_DIR}" >&2
  exit 1
fi

step() { printf "\n\033[1;34m==> %s\033[0m\n" "$*"; }
ok()   { printf "\033[1;32m✓ %s\033[0m\n" "$*"; }
warn() { printf "\033[1;33m! %s\033[0m\n" "$*"; }

generate_json_inline() { :; }

step "Running Ligetron SHA-256 benchmark"

pushd "${LIGETRON_DIR}/build" >/dev/null || { echo "Missing build directory: ${LIGETRON_DIR}/build" >&2; exit 1; }

# SHA-256 circuit expects: [1] input str, [2] input length (i64), [3] expected digest (hex)
if [[ ! -x "$UTILS_BIN" ]]; then
  echo "utils binary not found or not executable: $UTILS_BIN" >&2
  exit 1
fi

sizes_len=$("$UTILS_BIN" sizes len)
if [[ -z "$sizes_len" ]]; then
  echo "Failed to obtain sizes length from utils" >&2; exit 1
fi

for (( i=0; i<sizes_len; i++ )); do
  INPUT_SIZE=$("$UTILS_BIN" sizes get --index "$i")
  PROVER_JSON_FILE="prover_${INPUT_SIZE}.json"
  VERIFIER_JSON_FILE="verifier_${INPUT_SIZE}.json"

  if [[ ! -x "./webgpu_prover" || ! -x "./webgpu_verifier" ]]; then
    warn "webgpu_prover/webgpu_verifier not found or not executable. Did the native build succeed?"
  fi

  step "Prover (size ${INPUT_SIZE}):"
  hyperfine --runs 10 \
    --prepare "INPUT_SIZE=$INPUT_SIZE UTILS_BIN=$UTILS_BIN PROVER_JSON_FILE=$PROVER_JSON_FILE bash -lc 'set +o noclobber; GEN=\"\$(\"\$UTILS_BIN\" sha256 -n \"\$INPUT_SIZE\")\"; MSG=\$(printf \"%s\\n\" \"\$GEN\" | sed -n 1p); HEX=\$(printf \"%s\\n\" \"\$GEN\" | sed -n 2p); printf \"{\\\"program\\\":\\\"../sdk/build/examples/sha256.wasm\\\",\\\"shader-path\\\":\\\"../shader\\\",\\\"packing\\\":8192,\\\"private-indices\\\":[1],\\\"args\\\":[{\\\"str\\\":\\\"%s\\\"},{\\\"i64\\\":%d},{\\\"hex\\\":\\\"0x%s\\\"}]}\" \"\$MSG\" \$INPUT_SIZE \"\$HEX\" >| \"\$PROVER_JSON_FILE\"'" \
    "PROVER_JSON_FILE=$PROVER_JSON_FILE bash -lc 'ARG=\$(cat \"\$PROVER_JSON_FILE\"); exec ./webgpu_prover \"\$ARG\"'" \
    --export-json ./sha256_${INPUT_SIZE}_ligetron_prover_metrics.json

  step "Verifier (size ${INPUT_SIZE}):"
  hyperfine --runs 10 \
    --prepare "INPUT_SIZE=$INPUT_SIZE UTILS_BIN=$UTILS_BIN VERIFIER_JSON_FILE=$VERIFIER_JSON_FILE bash -lc 'set +o noclobber; GEN=\"\$(\"\$UTILS_BIN\" sha256 -n \"\$INPUT_SIZE\")\"; MSG=\$(printf \"%s\\n\" \"\$GEN\" | sed -n 1p); HEX=\$(printf \"%s\\n\" \"\$GEN\" | sed -n 2p); printf \"{\\\"program\\\":\\\"../sdk/build/examples/sha256.wasm\\\",\\\"shader-path\\\":\\\"../shader\\\",\\\"packing\\\":8192,\\\"private-indices\\\":[1],\\\"args\\\":[{\\\"str\\\":\\\"%s\\\"},{\\\"i64\\\":%d},{\\\"hex\\\":\\\"0x%s\\\"}]}\" \"\$MSG\" \$INPUT_SIZE \"\$HEX\" >| \"\$VERIFIER_JSON_FILE\"; ARG=\$(cat \"\$VERIFIER_JSON_FILE\"); ./webgpu_prover \"\$ARG\" > /dev/null 2>&1'" \
    "VERIFIER_JSON_FILE=$VERIFIER_JSON_FILE bash -lc 'ARG=\$(cat \"\$VERIFIER_JSON_FILE\"); exec ./webgpu_verifier \"\$ARG\"'" \
    --export-json ./sha256_${INPUT_SIZE}_ligetron_verifier_metrics.json 

  step "RAM measurement (size ${INPUT_SIZE})"
  if [[ -f "${MEASURE_RAM_SCRIPT}" ]]; then
    MEM_JSON="${SCRIPT_DIR}/sha256_ligetron_mem_report_${INPUT_SIZE}.json"
    bash "${MEASURE_RAM_SCRIPT}" -o "${MEM_JSON}" -- ./webgpu_prover "$(cat "$PROVER_JSON_FILE")" || warn "Memory measurement failed"
    ok "Memory report: ${MEM_JSON}"
  else
    warn "measure_mem_avg.sh not found: ${MEASURE_RAM_SCRIPT}"
  fi
done

popd >/dev/null

ok "Benchmark complete"

