#!/usr/bin/env bash
set -euo pipefail

# Run Ligetron SHA-256 benchmark
# Usage: benchmark.sh [--ligetron-dir <path>]

LIGETRON_DIR="${LIGETRON_DIR:-}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MEASURE_RAM_SCRIPT="${SCRIPT_DIR}/../measure_mem_avg.sh"

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

step "Running Ligetron SHA-256 benchmark"

pushd "${LIGETRON_DIR}/build" >/dev/null || { echo "Missing build directory: ${LIGETRON_DIR}/build" >&2; exit 1; }

# SHA-256 circuit expects: [1] input str, [2] input length (i64), [3] expected digest (hex)
PROVER_JSON='{"program":"../sdk/build/examples/sha256.wasm","shader-path":"../shader","packing":8192,"private-indices":[1],"args":[{"str":"abcdeabcdeabcde"},{"i64":15},{"hex":"0xfb3d5042fc80a5df2b0dc63d70b93b9e226f327f16e36591865a320daa458d7b"}]}'
VERIFIER_JSON='{"program":"../sdk/build/examples/sha256.wasm","shader-path":"../shader","packing":8192,"private-indices":[1],"args":[{"str":"xxxxxxxxxxxxxxx"},{"i64":15},{"hex":"0xfb3d5042fc80a5df2b0dc63d70b93b9e226f327f16e36591865a320daa458d7b"}]}'

if [[ ! -x "./webgpu_prover" || ! -x "./webgpu_verifier" ]]; then
  warn "webgpu_prover/webgpu_verifier not found or not executable. Did the native build succeed?"
fi

step "Prover:"
./webgpu_prover "${PROVER_JSON}" || { warn "Prover returned non-zero"; }

step "Verifier:"
./webgpu_verifier "${VERIFIER_JSON}" || { warn "Verifier returned non-zero"; }

step "RAM measurement"
if [[ -f "${MEASURE_RAM_SCRIPT}" ]]; then
  MEM_JSON="${SCRIPT_DIR}/sha256_ligetron_mem_report.json"
  bash "${MEASURE_RAM_SCRIPT}" -o "${MEM_JSON}" -- ./webgpu_prover "${PROVER_JSON}" || warn "Memory measurement failed"
  ok "Memory report: ${MEM_JSON}"
else
  warn "measure_mem_avg.sh not found: ${MEASURE_RAM_SCRIPT}"
fi

popd >/dev/null

ok "Benchmark complete"

