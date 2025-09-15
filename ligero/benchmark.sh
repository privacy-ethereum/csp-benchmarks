#!/usr/bin/env bash
set -euo pipefail

# Run Ligetron demo prover & verifier (Edit Distance)
# Usage: benchmark.sh [--ligetron-dir <path>]

LIGETRON_DIR="${LIGETRON_DIR:-}"

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
  SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
  LIGETRON_DIR="${SCRIPT_DIR}/third_party/ligetron"
fi

if [[ ! -d "${LIGETRON_DIR}" ]]; then
  echo "LIGETRON_DIR does not exist: ${LIGETRON_DIR}" >&2
  exit 1
fi

step() { printf "\n\033[1;34m==> %s\033[0m\n" "$*"; }
ok()   { printf "\033[1;32m✓ %s\033[0m\n" "$*"; }
warn() { printf "\033[1;33m! %s\033[0m\n" "$*"; }

step "Running demo prover & verifier (Edit Distance)"

pushd "${LIGETRON_DIR}/build" >/dev/null || { echo "Missing build directory: ${LIGETRON_DIR}/build" >&2; exit 1; }

PROVER_JSON='{"program":"../sdk/build/examples/edit.wasm","shader-path":"../shader","packing":8192,"private-indices":[1],"args":[{"str":"abcdeabcdeabcde"},{"str":"bcdefabcdeabcde"},{"i64":15},{"i64":15}]}'
VERIFIER_JSON='{"program":"../sdk/build/examples/edit.wasm","shader-path":"../shader","packing":8192,"private-indices":[1],"args":[{"str":"xxxxxxxxxxxxxxx"},{"str":"bcdefabcdeabcde"},{"i64":15},{"i64":15}]}'

if [[ ! -x "./webgpu_prover" || ! -x "./webgpu_verifier" ]]; then
  warn "webgpu_prover/webgpu_verifier not found or not executable. Did the native build succeed?"
fi

step "Prover:"
./webgpu_prover "${PROVER_JSON}" || { warn "Prover returned non-zero"; }

step "Verifier:"
./webgpu_verifier "${VERIFIER_JSON}" || { warn "Verifier returned non-zero"; }

popd >/dev/null

ok "Benchmark complete"

