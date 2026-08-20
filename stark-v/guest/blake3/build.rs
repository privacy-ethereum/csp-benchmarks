fn main() {
    let linker = std::env::var("DEP_GUEST_BIN_LINKER_SCRIPT")
        .expect("DEP_GUEST_BIN_LINKER_SCRIPT not set — requires guest-bin >= 4538cbab");
    println!("cargo:rustc-link-arg=-T{linker}");
}
