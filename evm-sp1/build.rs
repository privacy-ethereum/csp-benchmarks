fn main() {
    sp1_build::build_program_with_args(
        "guest/private_tx",
        sp1_build::BuildArgs {
            ignore_rust_version: true,
            ..Default::default()
        },
    );
}
