use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let zolt_dir = PathBuf::from(
        env::var("ZOLT_DIR").unwrap_or_else(|_| {
            // Default: sibling directory to csp-benchmarks
            let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            manifest
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("zolt")
                .to_string_lossy()
                .into_owned()
        }),
    );

    // Build zolt C-API static library
    let status = Command::new("zig")
        .args(["build", "-Doptimize=ReleaseFast"])
        .current_dir(&zolt_dir)
        .status()
        .expect("Failed to run `zig build`. Is Zig installed?");
    assert!(status.success(), "zig build failed");

    let lib_dir = zolt_dir.join("zig-out").join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=zolt_capi");

    // macOS system frameworks required by Zig's Metal backend + ObjC runtime
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=objc");
    }

    // Emit examples directory path for ELF lookup at runtime
    let examples_dir = zolt_dir.join("examples");
    println!(
        "cargo:rustc-env=ZOLT_EXAMPLES_DIR={}",
        examples_dir.display()
    );

    // Rerun if zolt sources change
    println!("cargo:rerun-if-changed={}/src", zolt_dir.display());
    println!("cargo:rerun-if-changed={}/build.zig", zolt_dir.display());
    println!("cargo:rerun-if-env-changed=ZOLT_DIR");
}
