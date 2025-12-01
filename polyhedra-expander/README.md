# Polyhedra Expander benchmark

## Prerequisites

Polyhedra Expander depends on MPI and the workspace nightly toolchain. Reproduce the CI environment (`.github/actions/install-ompi` plus the Rust workflow) before running benches:

```bash
brew install open-mpi
mpirun --version

rustup toolchain install nightly-2025-08-18-aarch64-apple-darwin \
  --component llvm-tools rustc-dev
rustup override set nightly-2025-08-18-aarch64-apple-darwin
```

## How to run

```bash
cd polyhedra-expander
cargo bench
```
