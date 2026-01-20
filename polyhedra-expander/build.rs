use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// `build.rs` is used to generate the circuits for different input sizes.
/// This is necessary because Expander is using a macro to build the circuit for a fixed input size.
/// `src/metadata.rs` is a symlink to `utils/src/metadata.rs` and is used by `build.rs`
/// to generate the circuits for different input sizes.
/// `build.rs` can only track changes inside the crate, so symlink is necessary to avoid rebuilding on any code change.
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Read the metadata at build-time
    let utils_metadata = root.join("src/metadata.rs");
    let contents = fs::read_to_string(&utils_metadata).expect("read src/metadata.rs");

    generate_circuit(&root, &out_dir, &contents, "BYTE_INPUTS_FULL", "sha256");
    generate_circuit(
        &root,
        &out_dir,
        &contents,
        "FIELD_ELEMENT_INPUTS_FULL",
        "poseidon",
    );

    println!("cargo:rerun-if-env-changed=BENCH_INPUT_PROFILE");
    println!("cargo:rerun-if-changed=templates/sha256_sizes.rs.tpl");
    println!("cargo:rerun-if-changed=templates/poseidon_sizes.rs.tpl");
    println!("cargo:rerun-if-changed={}", utils_metadata.display());
}

fn generate_circuit(root: &Path, out_dir: &Path, contents: &str, const_name: &str, name: &str) {
    // Parse input sizes from const
    let mut sizes: Vec<String> = Vec::new();
    if let Some(id_start) = contents.find(const_name) {
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
    assert!(!sizes.is_empty(), "Failed to parse {}", const_name);

    // Load and process template
    let template_path = root.join(format!("templates/{name}_sizes.rs.tpl"));
    let template = fs::read_to_string(&template_path).expect("read template");

    let decl_begin_tag = "// BEGIN_DECL";
    let decl_end_tag = "// END_DECL";
    let decl_start = template.find(decl_begin_tag).expect("BEGIN_DECL not found");
    let decl_end = template.find(decl_end_tag).expect("END_DECL not found");
    let decl_snippet = &template[decl_start + decl_begin_tag.len()..decl_end];

    let match_arm_begin_tag = "// BEGIN_MATCH_ARM";
    let match_arm_end_tag = "// END_MATCH_ARM";
    let match_arm_start = template
        .find(match_arm_begin_tag)
        .expect("BEGIN_MATCH_ARM not found");
    let match_arm_end = template
        .find(match_arm_end_tag)
        .expect("END_MATCH_ARM not found");
    let match_arm_snippet = &template[match_arm_start + match_arm_begin_tag.len()..match_arm_end];

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

    let mut wrapper = String::new();
    wrapper.push_str(&template[..decl_start]);
    wrapper.push_str(&template[decl_end + decl_end_tag.len()..match_arm_start]);
    wrapper.push_str(&template[match_arm_end + match_arm_end_tag.len()..]);

    let final_out = wrapper
        .replace("{{CIRCUIT_DECLS}}", &decls_rendered)
        .replace("{{MATCH_ARMS}}", &arms_rendered);

    fs::write(out_dir.join(format!("{name}_sizes.rs")), final_out).expect("write generated file");
}
