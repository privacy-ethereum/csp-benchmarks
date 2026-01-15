use std::env;
use std::fs;
use std::path::PathBuf;

/// `build.rs` is used to generate the circuits for different input sizes.
/// This is necessary because Expander is using a macro to build the circuit for a fixed input size.
/// `src/metadata.rs` is a symlink to `utils/src/metadata.rs` and is used by `build.rs`
/// to generate the circuits for different input sizes.
/// `build.rs` can only track changes inside the crate, so symlink is necessary to avoid rebuilding on any code change.
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Read the metadata::BYTE_INPUTS at build-time by including the file and evaluating it here
    let workspace_root = root.parent().unwrap();

    let utils_metadata = workspace_root.join("polyhedra-expander/src/metadata.rs");
    let contents = fs::read_to_string(&utils_metadata).expect("read src/metadata.rs");

    // Always parse the full set of input sizes from const BYTE_INPUTS_FULL: [usize; N] = [a, b, c];
    let mut sizes: Vec<String> = Vec::new();
    if let Some(id_start) = contents.find("BYTE_INPUTS_FULL") {
        let after_id = &contents[id_start..];
        if let Some(eq_rel) = after_id.find('=') {
            let after_eq = &after_id[eq_rel + 1..];
            if let Some(lb_rel) = after_eq.find('[') {
                let after_lb = &after_eq[lb_rel + 1..];
                if let Some(rb_rel) = after_lb.find(']') {
                    let inner = &after_lb[..rb_rel];
                    sizes = inner
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .filter_map(|s| s.parse::<usize>().ok())
                        .map(|n| n.to_string())
                        .collect();
                }
            }
        }
    }

    assert!(
        !sizes.is_empty(),
        "Failed to parse sizes from utils/src/metadata.rs: BYTE_INPUTS is empty"
    );

    // Single template file approach
    let template_path = root.join("templates/sha256_sizes.rs.tpl");
    let template = fs::read_to_string(&template_path).expect("read templates/sha256_sizes.rs.tpl");

    // Extract declaration snippet between markers
    let decl_begin_tag = "// BEGIN_DECL";
    let decl_end_tag = "// END_DECL";
    let decl_start = template
        .find("// BEGIN_DECL")
        .expect("BEGIN_DECL not found");
    let decl_end = template.find("// END_DECL").expect("END_DECL not found");
    let decl_snippet = &template[decl_start + decl_begin_tag.len()..decl_end];

    // Extract match arm snippet between markers
    let match_arm_begin_tag = "// BEGIN_MATCH_ARM";
    let match_arm_end_tag = "// END_MATCH_ARM";
    let match_arm_start = template
        .find("// BEGIN_MATCH_ARM")
        .expect("BEGIN_MATCH_ARM not found");
    let match_arm_end = template
        .find("// END_MATCH_ARM")
        .expect("END_MATCH_ARM not found");
    let match_arm_snippet = &template[match_arm_start + match_arm_begin_tag.len()..match_arm_end];

    // Render repeated sections
    let mut decls_rendered = String::new();
    for size in &sizes {
        let rendered = decl_snippet.replace("{{LEN}}", size);
        decls_rendered.push_str(rendered.trim_start_matches('\n'));
        decls_rendered.push('\n');
    }

    let mut arms_rendered = String::new();
    for size in &sizes {
        let rendered = match_arm_snippet.replace("{{LEN}}", size);
        arms_rendered.push_str(rendered.trim_start_matches('\n'));
    }

    // Remove snippet blocks and keep existing placeholders from template
    let mut wrapper = String::new();
    wrapper.push_str(&template[..decl_start]);
    wrapper.push_str(&template[decl_end + decl_end_tag.len()..match_arm_start]);
    wrapper.push_str(&template[match_arm_end + match_arm_end_tag.len()..]);

    // Replace placeholders in the wrapper
    let final_out = wrapper
        .replace("{{CIRCUIT_DECLS}}", &decls_rendered)
        .replace("{{MATCH_ARMS}}", &arms_rendered);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("sha256_sizes.rs");
    fs::write(&out_file, final_out).expect("write generated sha256_sizes.rs");

    println!("cargo:rerun-if-env-changed=BENCH_INPUT_PROFILE");
    println!("cargo:rerun-if-changed={}", template_path.display());
    println!("cargo:rerun-if-changed={}", utils_metadata.display());
}
