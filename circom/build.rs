use std::{
    fs,
    path::{Path, PathBuf},
};

fn emit_rpath(path: &Path) {
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path.display());
}

fn find_rapidsnark_dirs(out_dir: &str) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let out_path = Path::new(out_dir);
    let build_root = out_path.parent().and_then(|p| p.parent());
    let target = std::env::var("TARGET").ok();

    if let Some(build_root) = build_root
        && let Ok(entries) = fs::read_dir(build_root)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !name.starts_with("rust-rapidsnark-") {
                continue;
            }
            let base = path.join("out").join("rapidsnark");
            let mut candidates = Vec::new();
            if let Some(ref triple) = target {
                candidates.push(base.join(triple));
            }
            candidates.push(base.join("aarch64-apple-darwin"));
            candidates.push(base.join("x86_64-apple-darwin"));
            for candidate in candidates {
                if candidate.join("librapidsnark.dylib").exists() {
                    dirs.push(candidate);
                }
            }
        }
    }

    dirs
}

const CIRCUIT_DIR: &str = "./circuits/ecdsa";

fn circuit_sources() -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(CIRCUIT_DIR) else {
        return Vec::new();
    };
    entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "circom"))
        .collect()
}

/// The generated files are not tracked, so their mtime is what says whether they
/// still describe the circuit. CI never relies on this: its cache is keyed on a
/// hash of the same sources, so an edited circuit misses the cache and lands
/// here with nothing to skip.
fn is_current(cpp: &Path, dat: &Path, sources: &[PathBuf]) -> bool {
    if !dat.exists() || sources.is_empty() {
        return false;
    }
    let Ok(built) = cpp.metadata().and_then(|m| m.modified()) else {
        return false;
    };
    sources.iter().all(|source| {
        source
            .metadata()
            .and_then(|m| m.modified())
            .is_ok_and(|changed| changed <= built)
    })
}

/// The ECDSA witness generator is the one artifact not tracked in the repo: at
/// 57 MiB of generated C++ it is roughly three times the largest file otherwise
/// stored here, because the width-12 comb table is inlined in the circuit. It is
/// compiled from `ecdsa_32.circom` on demand instead, which needs `circom` on
/// PATH. Every other circuit still ships its `.cpp` and `.dat` in tree.
fn generate_ecdsa_witness_generator() {
    const CIRCUIT: &str = "ecdsa_32.circom";

    // Every circuit in the directory, not just `ecdsa_32.circom`: that file is a
    // `main` component over the includes that hold the circuit itself. The
    // directory is not watched as a whole because the generated files live in it.
    let sources = circuit_sources();
    for source in &sources {
        println!("cargo:rerun-if-changed={}", source.display());
    }

    let dest = Path::new(CIRCUIT_DIR).join("ecdsa_32");
    let cpp = dest.join("ecdsa_32.cpp");
    let dat = dest.join("ecdsa_32.dat");
    if is_current(&cpp, &dat, &sources) {
        return;
    }

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set for build scripts");
    let staging = Path::new(&out_dir).join("ecdsa_32_circom");
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging).expect("cannot create the circom output directory");

    // `--c` alone: the `.r1cs` is not needed to build the witness generator, and
    // writing it costs over 100 MB. circom emits into `<out>/ecdsa_32_cpp/`.
    let status = std::process::Command::new("circom")
        .current_dir(CIRCUIT_DIR)
        .arg(CIRCUIT)
        .arg("--c")
        .arg("-o")
        .arg(&staging)
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "could not run `circom`, which builds the ECDSA witness generator \
                 (the only circuit artifact not stored in the repo): {e}. \
                 Install circom 2.2.3 and put it on PATH."
            )
        });
    assert!(status.success(), "circom failed to compile {CIRCUIT}");

    let emitted = staging.join("ecdsa_32_cpp");
    fs::create_dir_all(&dest).expect("cannot create the circuit directory");
    for file in ["ecdsa_32.cpp", "ecdsa_32.dat"] {
        fs::copy(emitted.join(file), dest.join(file))
            .unwrap_or_else(|e| panic!("circom did not emit {file}: {e}"));
    }
}

fn main() {
    generate_ecdsa_witness_generator();

    // SHA256 circuits
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_128");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_256");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_512");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_1024");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_2048");

    // ECDSA circuit (secp256k1, fake-GLV + width-12 comb)
    witnesscalc_adapter::build_and_link("./circuits/ecdsa/ecdsa_32");

    // Keccak circuits
    witnesscalc_adapter::build_and_link("./circuits/keccak/keccak_128");
    witnesscalc_adapter::build_and_link("./circuits/keccak/keccak_256");
    witnesscalc_adapter::build_and_link("./circuits/keccak/keccak_512");
    witnesscalc_adapter::build_and_link("./circuits/keccak/keccak_1024");
    witnesscalc_adapter::build_and_link("./circuits/keccak/keccak_2048");

    // Poseidon circuits
    witnesscalc_adapter::build_and_link("./circuits/poseidon/poseidon_2");
    witnesscalc_adapter::build_and_link("./circuits/poseidon/poseidon_4");
    witnesscalc_adapter::build_and_link("./circuits/poseidon/poseidon_8");
    witnesscalc_adapter::build_and_link("./circuits/poseidon/poseidon_12");
    witnesscalc_adapter::build_and_link("./circuits/poseidon/poseidon_16");

    if let Ok(out_dir) = std::env::var("OUT_DIR") {
        let lib_dir = Path::new(&out_dir)
            .join("witnesscalc")
            .join("package")
            .join("lib");
        if lib_dir.exists() {
            emit_rpath(&lib_dir);
        }

        let mut rapidsnark_dirs = if let Ok(paths) = std::env::var("DEP_RUST_RAPIDSNARK_NATIVE") {
            std::env::split_paths(std::ffi::OsStr::new(&paths))
                .filter(|p| p.exists())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        if rapidsnark_dirs.is_empty() {
            rapidsnark_dirs.extend(find_rapidsnark_dirs(&out_dir));
        }

        for dir in rapidsnark_dirs {
            emit_rpath(&dir);
        }
    }
}
