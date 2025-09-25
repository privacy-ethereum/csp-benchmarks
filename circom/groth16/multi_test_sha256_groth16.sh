#!/bin/bash

SCRIPT=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
TEST_SCRIPT="test_sha256_groth16.sh"

"$SCRIPT_DIR"/"$TEST_SCRIPT" 2048 21
