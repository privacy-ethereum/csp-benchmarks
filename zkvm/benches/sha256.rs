use ere_jolt::JOLT_TARGET;
use ere_miden::MIDEN_TARGET;
use ere_risc0::RV32_IM_RISC0_ZKVM_ELF;
use ere_sp1::RV32_IM_SUCCINCT_ZKVM_ELF;
use zkvm_csp_benchmarks::programs::{
    JoltBuilder, MidenBuilder, Risc0Builder, Sp1Builder, sha256::sha256_benchmark,
};

fn main() {
    sha256_benchmark(&RV32_IM_RISC0_ZKVM_ELF, &Risc0Builder, "risc0");
    sha256_benchmark(&RV32_IM_SUCCINCT_ZKVM_ELF, &Sp1Builder, "sp1");
    sha256_benchmark(&JOLT_TARGET, &JoltBuilder, "jolt");
    sha256_benchmark(&MIDEN_TARGET, &MidenBuilder, "miden");
}
