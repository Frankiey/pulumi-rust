//! Golden-file integration tests for codegen.
//!
//! These tests run codegen on known schemas and verify the output
//! matches expected patterns (structural checks, not exact string comparison).

use pulumi_codegen::format::format_code;
use pulumi_codegen::modules::generate_module_tree;
use pulumi_codegen::schema::PulumiSchema;
use std::collections::HashMap;

fn load_schema(json: &str) -> PulumiSchema {
    serde_json::from_str(json).expect("failed to parse schema")
}

fn load_test_schema(name: &str) -> PulumiSchema {
    let path = format!("{}/tests/testdata/{name}", env!("CARGO_MANIFEST_DIR"));
    let json =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
    load_schema(&json)
}

#[test]
fn test_mini_random_generates_expected_files() {
    let schema = load_test_schema("mini-random-schema.json");
    let tree = generate_module_tree(&schema);

    let file_map: HashMap<&str, &str> = tree
        .files
        .iter()
        .map(|f| (f.path.as_str(), f.content.as_str()))
        .collect();

    // Must have key files
    assert!(file_map.contains_key("src/lib.rs"), "missing lib.rs");
    assert!(
        file_map.contains_key("src/provider.rs"),
        "missing provider.rs"
    );

    // lib.rs should declare provider module
    let lib_rs = file_map["src/lib.rs"];
    assert!(lib_rs.contains("pub mod provider;"));
    assert!(lib_rs.contains("pub use provider::Provider;"));
}

#[test]
fn test_mini_random_resource_code_is_valid_rust() {
    let schema = load_test_schema("mini-random-schema.json");
    let tree = generate_module_tree(&schema);

    // Find a resource file containing RandomString
    let resource_file = tree
        .files
        .iter()
        .find(|f| f.content.contains("RandomString"))
        .expect("no file contains RandomString");

    // It should be parseable by prettyplease/syn
    let formatted = format_code(&resource_file.content);
    assert!(
        formatted.contains("RandomString"),
        "formatting lost RandomString"
    );
}

#[test]
fn test_mini_random_function_code_is_valid_rust() {
    let schema = load_test_schema("mini-random-schema.json");
    let tree = generate_module_tree(&schema);

    // Find a function file
    let fn_file = tree
        .files
        .iter()
        .find(|f| f.content.contains("GetRandomBytes"))
        .expect("no file contains GetRandomBytes");

    let formatted = format_code(&fn_file.content);
    assert!(
        formatted.contains("GetRandomBytes"),
        "formatting lost GetRandomBytes"
    );
}

#[test]
fn test_mini_random_cargo_toml() {
    let schema = load_test_schema("mini-random-schema.json");
    let tree = generate_module_tree(&schema);

    assert!(tree.cargo_toml.contains("pulumi-mini-random"));
    assert!(tree.cargo_toml.contains("1.0.0"));
    assert!(tree.cargo_toml.contains("pulumi-sdk"));
    assert!(tree.cargo_toml.contains("serde"));
}

#[test]
fn test_mini_random_provider_has_correct_token() {
    let schema = load_test_schema("mini-random-schema.json");
    let tree = generate_module_tree(&schema);

    let provider_file = tree
        .files
        .iter()
        .find(|f| f.path == "src/provider.rs")
        .expect("missing provider.rs");

    assert!(provider_file
        .content
        .contains("pulumi:providers:mini-random"));
}

#[test]
#[ignore]
fn gen_azure_native() {
    let json = std::fs::read_to_string("/tmp/azure-native-schema.json").unwrap();
    let schema: PulumiSchema = serde_json::from_str(&json).unwrap();
    let tree = generate_module_tree(&schema);

    let out = std::path::Path::new("/tmp/azure-native-gen");
    let _ = std::fs::remove_dir_all(out);
    std::fs::create_dir_all(out).unwrap();

    for f in &tree.files {
        let path = out.join(&f.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, &f.content).unwrap();
    }
    std::fs::write(out.join("Cargo.toml"), &tree.cargo_toml).unwrap();
    eprintln!("Generated {} files", tree.files.len());
}
