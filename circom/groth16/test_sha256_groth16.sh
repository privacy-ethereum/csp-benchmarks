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

parse_time_output() {
    awk '
        /maximum resident set size/ { mem = $1 }
        /^[ \t]*[0-9.]+ real/ { time = $1 }
        END {
            if (mem != "") printf("mem %s\n", mem);
            if (time != "") printf("time %s\n", time);
        }'
}

function run_with_time() {
    # usage: run_with_time cmd args...
    "${TIME[@]}" "$@" 2>&1 >/dev/null | parse_time_output
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
  pushd "$CIRCUIT_DIR"
  echo sed -i '' "s/Main([0-9]*)/Main($INPUT_SIZE)/" sha256_test.circom
  sed -i '' "s/Main([0-9]*)/Main($INPUT_SIZE)/" sha256_test.circom
  popd
}

function compile() {
  pushd "$CIRCUIT_DIR"
  echo circom sha256_test.circom --r1cs --sym --wasm
  circom sha256_test.circom --r1cs --sym --wasm
  popd
}

function setup() {
  echo "$SCRIPT_DIR"/trusted_setup.sh "$TAU_RANK"
  run_with_time "$SCRIPT_DIR"/trusted_setup.sh "$TAU_RANK"
  prove_key_size=$(ls -lh "$CIRCUIT_DIR"/sha256_test_0001.zkey | awk '{print $5}')
  verify_key_size=$(ls -lh "$CIRCUIT_DIR"/verification_key.json | awk '{print $5}')
  echo "Prove key size: $prove_key_size"
  echo "Verify key size: $verify_key_size"
}

function generateWtns() {
  pushd "$CIRCUIT_DIR"
  echo node sha256_test_js/generate_witness.js sha256_test_js/sha256_test.wasm input_${INPUT_SIZE}.json witness.wtns
  run_with_time node sha256_test_js/generate_witness.js sha256_test_js/sha256_test.wasm input_${INPUT_SIZE}.json witness.wtns
  popd
}

avg_time() {
    #
    # usage: avg_time n command ...
    #
    n=$1; shift
    (($# > 0)) || return
    echo "$@"
    for ((i = 0; i < n; i++)); do
        # capture stderr from /usr/bin/time
        "${TIME[@]}" "$@" 2>&1 >/dev/null
    done | awk '
        /maximum resident set size/ { mem += $1; nm++ }
        /^[ \t]*[0-9.]+ real/ { time += $1; nt++ }
        END {
            if (nm > 0) printf("mem %f\n", mem/nm);
            if (nt > 0) printf("time %f\n", time/nt);
        }'
}


function normalProve() {
  pushd "$CIRCUIT_DIR"
  avg_time 10 snarkjs groth16 prove sha256_test_0001.zkey witness.wtns proof.json public.json
  proof_size=$(ls -lh proof.json | awk '{print $5}')
  echo "Proof size: $proof_size"
  popd
}

function verify() {
  pushd "$CIRCUIT_DIR"
  avg_time 10 snarkjs groth16 verify verification_key.json public.json proof.json
  popd
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
