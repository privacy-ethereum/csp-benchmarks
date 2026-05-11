# ProveKit-Groth16 benchmarks

Groth16+BSB22 sibling of [`provekit/`](../provekit), pinned to
[`worldfnd/ProveKit@622c276f`](https://github.com/worldfnd/ProveKit/commit/622c276fdea28c3c020705e2df71d20870f415ce).

## Prerequisites

Same Noir + Rust toolchain as the WHIR `provekit/` crate. See [`../provekit/README.md`](../provekit/README.md).

## Benchmarking

```bash
cargo bench -p provekit-groth16-bench
```

## Trusted setup

A fresh trusted setup is sampled per `prepare` call from the OS RNG; the
toxic-waste struct is wiped via `ZeroizeOnDrop`. Suitable for benchmarking
only — production deployments need a proper MPC ceremony.
