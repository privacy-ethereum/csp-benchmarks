#!/usr/bin/env bash
set -u

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

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# -----------------------
# Install Nargo
# -----------------------

curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash
# Safe check for ZSH_VERSION or BASH_VERSION
if [ -n "${ZSH_VERSION-}" ]; then
    # We are in zsh
    [ -f "${HOME}/.zshrc" ] && source "${HOME}/.zshrc"
elif [ -n "${BASH_VERSION-}" ]; then
    # We are in bash
    # On macOS, .bash_profile is often used first, .bashrc sometimes sourced from it
    [ -f "${HOME}/.bashrc" ] && source "${HOME}/.bashrc"
    [ -f "${HOME}/.bash_profile" ] && source "${HOME}/.bash_profile"
else
    # Unknown shell: you can fallback or warn
    echo "Warning: Unknown shell, cannot source rc file automatically" >&2
fi
noirup --version 1.0.0-beta.13

# -----------------------
# Install Barretenberg(bbup)
# -----------------------

curl -L https://raw.githubusercontent.com/AztecProtocol/aztec-packages/refs/heads/next/barretenberg/bbup/install | bash
# Safe check for ZSH_VERSION or BASH_VERSION
if [ -n "${ZSH_VERSION-}" ]; then
    # We are in zsh
    [ -f "${HOME}/.zshrc" ] && source "${HOME}/.zshrc"
elif [ -n "${BASH_VERSION-}" ]; then
    # We are in bash
    # On macOS, .bash_profile is often used first, .bashrc sometimes sourced from it
    [ -f "${HOME}/.bashrc" ] && source "${HOME}/.bashrc"
    [ -f "${HOME}/.bash_profile" ] && source "${HOME}/.bash_profile"
else
    # Unknown shell: you can fallback or warn
    echo "Warning: Unknown shell, cannot source rc file automatically" >&2
fi
bbup -v 0.87.0

# -----------------------
# Run demo prover & verifier
# -----------------------
bash ../benchmark.sh --system-dir "${SCRIPT_DIR}"

ok "All done!"
