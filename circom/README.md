# Circom SHA256 benchmarks

This benchmark code is from: https://github.com/brevis-network/zk-benchmark/tree/main/circom

# Run SHA256 benches
1. Install the `circom`, `snarkjs`, `wget`.

2. Copy the `input_2048.json.example` file.  

```bash
cd circom
cp ./circuits/sha256_test/input_2048.json.example ./circuits/sha256_test/input_2048.json
```

3. Run the bench script.  

```bash
chmod +x ./groth16/multi_test_sha256_groth16.sh
chmod +x ./groth16/test_sha256_groth16.sh
chmod +x ./groth16/trusted_setup.sh
./groth16/multi_test_sha256_groth16.sh
```
