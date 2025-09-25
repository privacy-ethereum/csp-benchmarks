//! CLI for zkVM benchmarking.

use crate::cli::storage::Storage;
use crate::programs::{
    JoltBuilder, MidenBuilder, Risc0Builder, Sp1Builder, SupportedPrograms,
    sha256::{
        Sha256Config, Sha256Generator, SupportedConfigs as Sha256Cfg,
        build_input as build_sha_input,
    },
};
use crate::traits::DataGenerator;
use crate::zkvm::{SupportedVms, ZkVMInstance};
use clap::{Parser, Subcommand};
use ere_jolt::{EreJolt, JOLT_TARGET};
use ere_miden::{EreMiden, MIDEN_TARGET};
use ere_risc0::{EreRisc0, RV32_IM_RISC0_ZKVM_ELF};
use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};
use std::{
    fs,
    path::{Path, PathBuf},
};
use zkvm_interface::Input;

pub mod storage;

/// Root directory.
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Supported VMs.
const VMS: &[SupportedVms] = &[
    SupportedVms::Risc0,
    SupportedVms::Sp1,
    SupportedVms::Jolt,
    SupportedVms::Miden,
];

/// Supported programs.
const PROGRAMS: &[SupportedPrograms] = &[SupportedPrograms::Sha256];

/// Supported SHA256 configurations.
const SHA256_CONFIGS: &[Sha256Cfg] = &[Sha256Cfg::Size2048];

#[derive(Parser, Debug)]
#[command(name = "zkvm-bench", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Runs time benchmarks for all VMs and programs for all configurations.
    Bench,
    /// Generate inputs for every VM and every configuration of each program.
    GenInputs,
    /// Runs proving isolated for a given VM and program for a specific configuration.
    Prove {
        /// VM
        vm: String,
        /// Program
        program: String,
        /// Program configuration
        config: String,
    },
}

/// Main CLI runner.
pub fn run_cli() {
    match Cli::parse().cmd {
        Command::GenInputs => gen_inputs(),
        Command::Prove {
            vm,
            program,
            config,
        } => prove(&vm, &program, &config),
        Command::Bench => bench_all(),
    }
}

/// Generates inputs for every VM and every configuration of each program.
pub fn gen_inputs() {
    for prog in PROGRAMS {
        match prog {
            SupportedPrograms::Sha256 => {
                for cfg in SHA256_CONFIGS {
                    let sha_cfg = Sha256Config::from(*cfg);
                    let (data, _) = Sha256Generator.generate(&sha_cfg);

                    for vm in VMS {
                        let path = input_path(
                            Path::new(ROOT),
                            &vm.to_string(),
                            &prog.to_string(),
                            &cfg.to_string(),
                        );

                        ensure_parent_dir(&path);

                        let input = build_sha_input(vm, &data);
                        input.save(&path).expect("failed to save input");
                    }
                }
            }
        }
    }
}

/// Runs time benchmarks for all VMs and programs for all configurations.
fn bench_all() {
    let bench_name = SupportedPrograms::Sha256.to_string();
    let cfg = Sha256Config::from(Sha256Cfg::Size2048);
    let (data, _) = Sha256Generator.generate(&cfg);

    for vm in VMS {
        let vm_name = vm.to_string();
        let input = build_sha_input(vm, &data);

        match *vm {
            SupportedVms::Risc0 => {
                let zkvm = ZkVMInstance::<RV32_IM_RISC0_ZKVM_ELF, EreRisc0>::new(
                    &RV32_IM_RISC0_ZKVM_ELF,
                    &vm_name,
                    &bench_name,
                    &Risc0Builder,
                )
                .expect("failed to init Risc0 instance");
                zkvm.bench(&input).expect("Risc0 bench failed");
            }
            SupportedVms::Sp1 => {
                let zkvm = ZkVMInstance::<RV32_IM_SUCCINCT_ZKVM_ELF, EreSP1>::new(
                    &RV32_IM_SUCCINCT_ZKVM_ELF,
                    &vm_name,
                    &bench_name,
                    &Sp1Builder,
                )
                .expect("failed to init SP1 instance");
                zkvm.bench(&input).expect("SP1 bench failed");
            }
            SupportedVms::Jolt => {
                let zkvm = ZkVMInstance::<JOLT_TARGET, EreJolt>::new(
                    &JOLT_TARGET,
                    &vm_name,
                    &bench_name,
                    &JoltBuilder,
                )
                .expect("failed to init Jolt instance");
                zkvm.bench(&input).expect("Jolt bench failed");
            }
            SupportedVms::Miden => {
                let zkvm = ZkVMInstance::<MIDEN_TARGET, EreMiden>::new(
                    &MIDEN_TARGET,
                    &vm_name,
                    &bench_name,
                    &MidenBuilder,
                )
                .expect("failed to init Miden instance");
                zkvm.bench(&input).expect("Miden bench failed");
            }
        }
    }
}

/// Runs proving isolated for a given VM and program for a specific configuration.
pub fn prove(vm: &str, program: &str, config: &str) {
    let root = PathBuf::from(ROOT);
    let vm_enum = SupportedVms::from(vm);
    let prog_enum = SupportedPrograms::from(program);

    let cfg_str = match prog_enum {
        SupportedPrograms::Sha256 => Sha256Cfg::try_from(config).expect("invalid Sha256 config"),
    };

    let input = <Input as Storage<Input>>::load(input_path(
        &root,
        &vm_enum.to_string(),
        &prog_enum.to_string(),
        &cfg_str.to_string(),
    ))
    .expect("failed to load input");

    match vm_enum {
        SupportedVms::Risc0 => {
            let zkvm = ZkVMInstance::<RV32_IM_RISC0_ZKVM_ELF, EreRisc0>::new(
                &RV32_IM_RISC0_ZKVM_ELF,
                vm,
                program,
                &Risc0Builder,
            )
            .expect("failed to init Risc0 instance");
            zkvm.prove_only(&input).expect("Risc0 prove failed");
        }
        SupportedVms::Sp1 => {
            let zkvm = ZkVMInstance::<RV32_IM_SUCCINCT_ZKVM_ELF, EreSP1>::new(
                &RV32_IM_SUCCINCT_ZKVM_ELF,
                vm,
                program,
                &Sp1Builder,
            )
            .expect("failed to init SP1 instance");
            zkvm.prove_only(&input).expect("SP1 prove failed");
        }
        SupportedVms::Jolt => {
            let zkvm =
                ZkVMInstance::<JOLT_TARGET, EreJolt>::new(&JOLT_TARGET, vm, program, &JoltBuilder)
                    .expect("failed to init Jolt instance");
            zkvm.prove_only(&input).expect("Jolt prove failed");
        }
        SupportedVms::Miden => {
            let zkvm = ZkVMInstance::<MIDEN_TARGET, EreMiden>::new(
                &MIDEN_TARGET,
                vm,
                program,
                &MidenBuilder,
            )
            .expect("failed to init Miden instance");
            zkvm.prove_only(&input).expect("Miden prove failed");
        }
    }
}

/// Input path builder.
pub fn input_path(root: &Path, vm: &str, program: &str, config: &str) -> PathBuf {
    root.join("inputs")
        .join(vm)
        .join(format!("{program}-{config}.bin"))
}

/// Ensure the parent directory exists.
fn ensure_parent_dir(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create input directory");
    }
}
