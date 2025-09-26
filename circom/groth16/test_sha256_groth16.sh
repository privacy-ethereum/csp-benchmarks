#!/bin/bash

if [ $# -ne 2 ]; then
  echo "input_size and tau_rank required"
  exit 1
fi

set -e
SCRIPT=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
CIRCUIT_DIR=${SCRIPT_DIR}"/../circuits/sha256_test"

# Use /usr/bin/time -l (BSD time)
TIME=(/usr/bin/time -l)

# Global vars (will be filled in by run_with_time / avg_time)
LAST_MEM=""
LAST_TIME=""

parse_time_output() {
    awk '
        /maximum resident set size/ { mem = $1 }
        /^[ \t]*[0-9.]+ real/ { time = $1 }
        END {
            if (mem != "") printf("%s %s\n", mem, time);
        }'
}

function run_with_time() {
    # usage: run_with_time cmd args...
    result=$("${TIME[@]}" "$@" 2>&1 >/dev/null | parse_time_output)
    LAST_MEM=$(echo "$result" | awk '{print $1}')
    LAST_TIME=$(echo "$result" | awk '{print $2}')
}

bytes_to_human() {
  local bytes=$1
  local kib=$((1024))
  local mib=$((1024 * kib))
  local gib=$((1024 * mib))

  if (( bytes >= gib )); then
    printf "%.2f GiB" "$(echo "$bytes / $gib" | bc -l)"
  elif (( bytes >= mib )); then
    printf "%.2f MiB" "$(echo "$bytes / $mib" | bc -l)"
  elif (( bytes >= kib )); then
    printf "%.2f KiB" "$(echo "$bytes / $kib" | bc -l)"
  else
    printf "%d B" "$bytes"
  fi
}

INPUT_SIZE=$1
TAU_RANK=$2
echo "input size $INPUT_SIZE"
echo "tau rank $TAU_RANK"
TAU_DIR=${SCRIPT_DIR}"/../setup/tau"
TAU_FILE="${TAU_DIR}/powersOfTau28_hez_final_${TAU_RANK}.ptau"

export NODE_OPTIONS=--max_old_space_size=327680
# sysctl -w vm.max_map_count=655300

PREPROCESSING_SIZE=0
PROOF_SIZE=0
PROVE_MEM=0
PROVE_TIME=0
VERIFY_TIME=0

function renderCircom() {
  cd "$CIRCUIT_DIR"
  echo sed -i '' "s/Main([0-9]*)/Main($INPUT_SIZE)/" sha256_test.circom
  sed -i '' "s/Main([0-9]*)/Main($INPUT_SIZE)/" sha256_test.circom
  cd ..
}

function compile() {
  cd "$CIRCUIT_DIR"
  echo circom sha256_test.circom --r1cs --sym --wasm
  run_with_time circom sha256_test.circom --r1cs --sym --wasm
  echo "Compile time: $LAST_TIME seconds"
  echo "Compile memory: $(bytes_to_human $LAST_MEM)"
  cd ..
}

function setup() {
  echo "$SCRIPT_DIR"/trusted_setup.sh "$TAU_RANK"
  run_with_time "$SCRIPT_DIR"/trusted_setup.sh "$TAU_RANK"
  echo "Trusted Setup time: $LAST_TIME seconds"
  echo "Trusted Setup memory: $(bytes_to_human $LAST_MEM)"
  prove_key_size=$(ls -lh "$CIRCUIT_DIR"/sha256_test_0001.zkey | awk '{print $5}')
  verify_key_size=$(ls -lh "$CIRCUIT_DIR"/verification_key.json | awk '{print $5}')
  echo "Prove key size: $prove_key_size"
  echo "Verify key size: $verify_key_size"
}

function generateWtns() {
  cd "$CIRCUIT_DIR"
  echo node sha256_test_js/generate_witness.js sha256_test_js/sha256_test.wasm input_${INPUT_SIZE}.json witness.wtns
  run_with_time node sha256_test_js/generate_witness.js sha256_test_js/sha256_test.wasm input_${INPUT_SIZE}.json witness.wtns
  echo "Generate witness time: $LAST_TIME seconds"
  echo "Generate witness memory: $(bytes_to_human $LAST_MEM)"
  cd ..
}

avg_time() {
    #
    # usage: avg_time n command ...
    #
    n=$1; shift
    (($# > 0)) || return
    echo "$@"
    result=$(
      for ((i = 0; i < n; i++)); do
          "${TIME[@]}" "$@" 2>&1 >/dev/null
      done | awk '
          /maximum resident set size/ { mem += $1; nm++ }
          /^[ \t]*[0-9.]+ real/ { time += $1; nt++ }
          END {
              if (nm > 0) printf("%f %f\n", mem/nm, time/nt);
          }'
    )
    LAST_MEM=$(echo "$result" | awk '{print $1}')
    LAST_TIME=$(echo "$result" | awk '{print $2}')
}


function normalProve() {
  cd "$CIRCUIT_DIR"
  avg_time 10 snarkjs groth16 prove sha256_test_0001.zkey witness.wtns proof.json public.json
  echo "Prove time: $LAST_TIME seconds"
  echo "Prove memory: $(bytes_to_human $LAST_MEM)"
  proof_size=$(ls -lh proof.json | awk '{print $5}')
  echo "Proof size: $proof_size"
  preprocessing_size = $(du -ck sha256_test_0001.zkey witness.wtns proof.json public.json | grep total | awk '{print $1}')
  echo "Preprocessing size: $preprocessing_size"
  cd ..
}

function verify() {
  cd "$CIRCUIT_DIR"
  avg_time 10 snarkjs groth16 verify verification_key.json public.json proof.json
  echo "Verify time: $LAST_TIME seconds"
  echo "Verify memory: $(bytes_to_human $LAST_MEM)"
  cd ..
}

echo "========== Step0: render circom  =========="
renderCircom

echo "========== Step1: compile circom  =========="
compile

echo "========== Step2: setup =========="
setup

echo "========== Step3: generate witness  =========="
generateWtns

echo "========== Step4: prove  =========="
normalProve

echo "========== Step5: verify  =========="
verify
