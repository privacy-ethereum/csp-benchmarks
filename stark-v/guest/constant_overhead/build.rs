fn main() {
    let linker = std::env::var("DEP_GUEST_BIN_LINKER_SCRIPT")
        .expect("guest-bin did not provide a linker script");
    println!("cargo:rustc-link-arg=-T{linker}");
}
