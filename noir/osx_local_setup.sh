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

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Detect the shell
current_shell=$(basename "$SHELL")

# -----------------------
# Install Nargo
# -----------------------

curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash
if [ "$current_shell" = "zsh" ]; then
    [ -f ~/.zshrc ] && source ~/.zshrc
elif [ "$current_shell" = "bash" ]; then
    # On macOS, interactive bash sessions might use .bash_profile or .bashrc
    [ -f ~/.bashrc ] && source ~/.bashrc
    [ -f ~/.bash_profile ] && source ~/.bash_profile
else
    echo "Unsupported shell: $current_shell" >&2
fi
noirup --version 1.0.0-beta.9

# -----------------------
# Install Barretenberg(bbup)
# -----------------------

curl -L https://raw.githubusercontent.com/AztecProtocol/aztec-packages/refs/heads/next/barretenberg/bbup/install | bash
if [ "$current_shell" = "zsh" ]; then
    [ -f ~/.zshrc ] && source ~/.zshrc
elif [ "$current_shell" = "bash" ]; then
    # On macOS, interactive bash sessions might use .bash_profile or .bashrc
    [ -f ~/.bashrc ] && source ~/.bashrc
    [ -f ~/.bash_profile ] && source ~/.bash_profile
else
    echo "Unsupported shell: $current_shell" >&2
fi
bbup

# -----------------------
# Run demo prover & verifier
# -----------------------
bash ../benchmark.sh --system-dir "${SCRIPT_DIR}"

ok "All done!"
