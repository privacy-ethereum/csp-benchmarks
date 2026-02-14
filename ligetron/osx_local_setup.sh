#!/usr/bin/env bash
set -euo pipefail

# ================================
# Ligetron macOS end-to-end setup
# ================================
# - Installs Homebrew deps (cmake, gmp, mpfr, libomp, llvm, boost, nlohmann-json, wabt)
# - Builds Dawn (WebGPU)
# - Installs Emscripten (emsdk)
# - Builds Ligetron SDK (emscripten)
# - Builds Ligetron native
# - Runs test prover & verifier
#
# Re-runnable: clones are skipped if folders exist.
# Use --reinstall to force a clean rebuild of third-party and Ligetron builds.

### Helpers
step() { printf "\n\033[1;34m==> %s\033[0m\n" "$*"; }
ok()   { printf "\033[1;32m✓ %s\033[0m\n" "$*"; }
warn() { printf "\033[1;33m! %s\033[0m\n" "$*"; }

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script is for macOS (Darwin) only."; exit 1
fi

REINSTALL=0
for arg in "$@"; do
  case "$arg" in
    --reinstall|-r)
      REINSTALL=1
      ;;
    -h|--help)
      echo "Usage: $0 [--reinstall]"; exit 0
      ;;
    *)
      warn "Unknown argument: $arg"
      ;;
  esac
done

ROOT_DIR="$(pwd)"
TP_DIR="${ROOT_DIR}/third_party"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
mkdir -p "${TP_DIR}"

if [[ "${REINSTALL}" == "1" ]]; then
  step "Reinstall mode enabled: cleaning build caches before rebuilding"
fi

# -----------------------
# Homebrew + dependencies
# -----------------------
if ! command -v brew >/dev/null 2>&1; then
  step "Installing Homebrew"
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ok "Homebrew installed"
else
  step "Homebrew already present"
fi

step "Installing build dependencies via Homebrew"
brew update
brew install cmake gmp mpfr libomp llvm boost nlohmann-json wabt
ok "Homebrew deps installed"

# Prefer Xcode clang; if you want brew llvm, uncomment exports below.
export CC="${CC:-clang}"
export CXX="${CXX:-clang++}"

# -----------------------
# Build Dawn (WebGPU)
# -----------------------
DAWN_DIR="${TP_DIR}/dawn"
DAWN_COMMIT="cec4482eccee45696a7c0019e750c77f101ced04"

if [[ ! -d "${DAWN_DIR}" ]]; then
  step "Cloning Dawn"
  git clone https://dawn.googlesource.com/dawn "${DAWN_DIR}"
fi

step "Building & installing Dawn @ ${DAWN_COMMIT}"
pushd "${DAWN_DIR}" >/dev/null
git fetch --all
git checkout "${DAWN_COMMIT}"
if [[ "${REINSTALL}" == "1" ]]; then
  # Clean stale build cache to avoid CMake source-dir mismatch
  rm -rf release
fi
mkdir -p release
pushd release >/dev/null
cmake -DDAWN_FETCH_DEPENDENCIES=ON -DDAWN_ENABLE_INSTALL=ON -DCMAKE_BUILD_TYPE=Release ..
cmake --build . -j
# Install to default prefix (/usr/local on Intel; /opt/homebrew on Apple Silicon may need sudo)
sudo cmake --install .
popd >/dev/null
popd >/dev/null
ok "Dawn installed"

# -----------------------
# Install Emscripten SDK
# -----------------------
EMSDK_DIR="${TP_DIR}/emsdk"
if [[ ! -d "${EMSDK_DIR}" ]]; then
  step "Cloning emsdk"
  git clone https://github.com/emscripten-core/emsdk.git "${EMSDK_DIR}"
fi

step "Installing & activating latest emsdk"
pushd "${EMSDK_DIR}" >/dev/null
git pull || true
./emsdk install latest
./emsdk activate latest
# shellcheck disable=SC1091
source ./emsdk_env.sh
popd >/dev/null
ok "emsdk ready (emcmake available)"

# -----------------------
# Ligetron submodule
# -----------------------
LIGETRON_DIR="${SCRIPT_DIR}/ligero-prover"

if [[ ! -d "${LIGETRON_DIR}" ]]; then
  step "Ligetron submodule not found; initializing"
  git -C "${REPO_ROOT}" submodule update --init --recursive ligetron/ligero-prover || {
    warn "Failed to init submodule. Run: git submodule update --init --recursive"; exit 1; }
fi
ok "Ligetron submodule ready"

# -----------------------
# Build Ligetron SDK (Web)
# -----------------------
step "Building Ligetron SDK with emscripten"
pushd "${LIGETRON_DIR}/sdk/cpp" >/dev/null
if [[ "${REINSTALL}" == "1" ]]; then
  # Clean stale build cache
  rm -rf build
fi
mkdir -p build
pushd build >/dev/null
# Ensure emsdk env is live for this subshell
# shellcheck disable=SC1091
source "${EMSDK_DIR}/emsdk_env.sh"
emcmake cmake ..
cmake --build . -j
popd >/dev/null
popd >/dev/null
ok "Ligetron SDK built"

# -----------------------
# Build Ligetron Native
# -----------------------
step "Building Ligetron native (Release)"
pushd "${LIGETRON_DIR}" >/dev/null
if [[ "${REINSTALL}" == "1" ]]; then
  # Clean stale build cache
  rm -rf build
fi
mkdir -p build
pushd build >/dev/null
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . -j
popd >/dev/null
popd >/dev/null
ok "Ligetron native built"

# -----------------------
# Run demo prover & verifier
# -----------------------
bash "${REPO_ROOT}/benchmark.sh" --system-dir "${SCRIPT_DIR}"

ok "All done!"
