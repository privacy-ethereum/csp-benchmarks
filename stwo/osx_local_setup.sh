#!/usr/bin/env bash
set -u

# ================================
# STWO macOS end-to-end setup
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

echo "Setting up virtual environment at .venv with Python 3.9"

# -----------------------
# Install uv
# -----------------------
if ! command -v uv >/dev/null; then
    curl -LsSf https://astral.sh/uv/install.sh | sh
fi


# ---------------------------
# Create virtual environment
# ---------------------------
if ! [ -d ".venv" ]; then
    echo "Creating virtual environment..."
    uv python install 3.9
    uv venv --python 3.9
    echo "Virtual environment created."
fi
echo "Virtual environment exists."

source .venv/bin/activate
echo "Virtual environment activated."

# -----------------------
# Update dependencies
# -----------------------
echo "Installing dependencies"
python -m ensurepip --upgrade
python -m pip install cairo-lang || {
    echo "Failed to install cairo-lang."
    exit 1
}

# -----------------------
# Run demo prover & verifier
# -----------------------
bash ../benchmark.sh --system-dir "${SCRIPT_DIR}"

ok "All done!"
