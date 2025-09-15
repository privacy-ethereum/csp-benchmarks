#!/usr/bin/env bash
set -euo pipefail

# ================================
# Ligetron macOS end-to-end setup
# ================================
# - Installs Homebrew deps (cmake, gmp, mpfr, libomp, llvm, boost, nlohmann-json)
# - Builds Dawn (WebGPU)
# - Builds WABT
# - Installs Emscripten (emsdk)
# - Clones and builds Ligetron SDK (emscripten)
# - Builds Ligetron native
# - Runs test prover & verifier
#
# Re-runnable: clones are skipped if folders exist.

### Helpers
step() { printf "\n\033[1;34m==> %s\033[0m\n" "$*"; }
ok()   { printf "\033[1;32m✓ %s\033[0m\n" "$*"; }
warn() { printf "\033[1;33m! %s\033[0m\n" "$*"; }

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script is for macOS (Darwin) only."; exit 1
fi

ROOT_DIR="$(pwd)"
TP_DIR="${ROOT_DIR}/third_party"
mkdir -p "${TP_DIR}"

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
brew install cmake gmp mpfr libomp llvm boost nlohmann-json
ok "Homebrew deps installed"

# Prefer Xcode clang; if you want brew llvm, uncomment exports below.
export CC="${CC:-clang}"
export CXX="${CXX:-clang++}"
# If you hit toolchain issues, you can force brew LLVM like this:
# export CC="$(brew --prefix llvm)/bin/clang"
# export CXX="$(brew --prefix llvm)/bin/clang++"
# export LDFLAGS="-L$(brew --prefix llvm)/lib ${LDFLAGS:-}"
# export CPPFLAGS="-I$(brew --prefix llvm)/include ${CPPFLAGS:-}"
# export PATH="$(brew --prefix llvm)/bin:${PATH}"

# -----------------------
# Build Dawn (WebGPU)
# -----------------------
DAWN_DIR="${TP_DIR}/dawn"
DAWN_COMMIT="41d631c0cbcd46ddc723222fc80890f4305dbc65"

if [[ ! -d "${DAWN_DIR}" ]]; then
  step "Cloning Dawn"
  git clone https://dawn.googlesource.com/dawn "${DAWN_DIR}"
fi

step "Building & installing Dawn @ ${DAWN_COMMIT}"
pushd "${DAWN_DIR}" >/dev/null
git fetch --all
git checkout "${DAWN_COMMIT}"
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
# Build WABT
# -----------------------
WABT_DIR="${TP_DIR}/wabt"
if [[ ! -d "${WABT_DIR}" ]]; then
  step "Cloning WABT"
  git clone https://github.com/WebAssembly/wabt.git "${WABT_DIR}"
fi

step "Building & installing WABT (clang++)"
pushd "${WABT_DIR}" >/dev/null
git submodule update --init
mkdir -p build
pushd build >/dev/null
cmake -DCMAKE_CXX_COMPILER="${CXX}" ..
cmake --build . -j
sudo cmake --install .
popd >/dev/null
popd >/dev/null
ok "WABT installed"

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
# Clone Ligetron
# -----------------------
LIGETRON_DIR="${TP_DIR}/ligetron"
if [[ ! -d "${LIGETRON_DIR}" ]]; then
  step "Cloning Ligetron"
  git clone https://github.com/ligeroinc/ligero-prover.git "${LIGETRON_DIR}"
else
  step "Ligetron already present; pulling latest"
  pushd "${LIGETRON_DIR}" >/dev/null
  git pull || true
  popd >/dev/null
fi

# -----------------------
# Build Ligetron SDK (Web)
# -----------------------
step "Building Ligetron SDK with emscripten"
pushd "${LIGETRON_DIR}/sdk" >/dev/null
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
bash "$(dirname "$0")/benchmark.sh" --ligetron-dir "${LIGETRON_DIR}"

ok "All done!"
echo
echo "Binaries: ${LIGETRON_DIR}/build"
echo "SDK wasm: ${LIGETRON_DIR}/sdk/build/examples/edit.wasm"
echo "If you need the Web build later, set CMAKE_PREFIX_PATH to your dependency prefixes and build in ${LIGETRON_DIR}/build-web."
