SHELL := /usr/bin/env bash

BENCH_BIN := ./target/release/zkvm-csp-benchmarks
BENCH_MEM_SCRIPT := scripts/bench_mem.sh

.PHONY: build inputs bench bench-time bench-mem clean

bench: build inputs clean bench-time bench-mem
	@echo "============================================================"
	@echo "Successfully completed benchmarks."
	@echo "============================================================"

bench-time:
	@echo "--- Running time benchmarks ---"
	@$(BENCH_BIN) bench
	@echo "Time benchmarks complete."

bench-mem:
	@echo "--- Running memory benchmarks ---"
	@bash $(BENCH_MEM_SCRIPT)
	@echo "Memory benchmarks complete."

build:
	@echo "--- Building benchmark runner ---"
	@cargo build --release

inputs:
	@echo "--- Generating benchmark inputs ---"
	@$(BENCH_BIN) gen-inputs

clean:
	@rm -rf reports
