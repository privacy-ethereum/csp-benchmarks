#!/usr/bin/env bash
set -euo pipefail

# ================================
# Noir macOS end-to-end setup
# ================================
# - Installs Nargo
# - Installs Barretenberg(bbup)
# - Runs test prover & verifier
#

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
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

if [[ "${REINSTALL}" == "1" ]]; then
  step "Reinstall mode enabled: cleaning build caches before rebuilding"
fi

# # -----------------------
# # Install Nargo
# # -----------------------

# curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash
# noirup --version 1.0.0-beta.9

# # -----------------------
# # Install Barretenberg(bbup)
# # -----------------------

# curl -L https://raw.githubusercontent.com/AztecProtocol/aztec-packages/refs/heads/next/barretenberg/bbup/install | bash
# bbup

# -----------------------
# Run demo prover & verifier
# -----------------------
bash ../benchmark.sh --system-dir "${SCRIPT_DIR}"

ok "All done!"
