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

fn main() {
    // SHA256 circuits
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_128");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_256");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_512");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_1024");
    witnesscalc_adapter::build_and_link("./circuits/sha256/sha256_2048");

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
