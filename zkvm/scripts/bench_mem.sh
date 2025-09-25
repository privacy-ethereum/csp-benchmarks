#!/usr/bin/env bash

# --- Settings ---
BENCH_BIN="./target/release/zkvm-csp-benchmarks"
REPORTS_DIR="${REPORTS_DIR:-reports}"
OUT="$REPORTS_DIR/memory.json"
VMS=(sp1 jolt miden risc0)
PROGRAM="sha256"
CONFIG="2048"
# ---------------

mkdir -p "$REPORTS_DIR"
[ -x "$BENCH_BIN" ] || { echo "Missing or not executable: $BENCH_BIN" >&2; exit 1; }

# Start JSON
printf '{' > "$OUT"
comma=""

for vm in "${VMS[@]}"; do
  echo "Running $vm (program=$PROGRAM, size=$CONFIG)..." >&2

  bytes=$({ /usr/bin/time -l "$BENCH_BIN" prove "$vm" "$PROGRAM" "$CONFIG" >/dev/null; } 2>&1 \
          | awk '/maximum resident set size/ {print $1; exit}')

  [ -n "$bytes" ] || { echo "Could not read memory for $vm" >&2; exit 1; }

  printf '%s"%s":%s' "$comma" "$vm" "$bytes" >> "$OUT"
  comma=","
done

# Close JSON
printf '}\n' >> "$OUT"

echo "Wrote $OUT" >&2
