//! Module tree generation.
//!
//! Builds a coherent Rust module tree from a Pulumi schema, mapping token module
//! paths to a directory structure with lib.rs, mod.rs files, and per-module
//! subdirectories for resources, types, and functions.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use crate::enums;
use crate::functions;
use crate::naming::{self, NamingContext};
use crate::provider;
use crate::resources;
use crate::schema::PulumiSchema;

/// A single generated file in the output tree.
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    /// Relative path from the crate root (e.g., "src/storage/mod.rs").
    pub path: String,
    /// The Rust source code.
    pub content: String,
}

/// The complete module tree for a generated provider crate.
#[derive(Debug)]
pub struct ModuleTree {
    /// All generated files.
    pub files: Vec<GeneratedFile>,
    /// The Cargo.toml content.
    pub cargo_toml: String,
}

/// Internal node in the module tree.
#[derive(Debug, Default)]
struct ModuleNode {
    /// Resource code snippets (filename → code).
    resources: BTreeMap<String, String>,
    /// Function code snippets (filename → code).
    functions: BTreeMap<String, String>,
    /// Type/enum code snippets (filename → code).
    types: BTreeMap<String, String>,
    /// Child modules.
    children: BTreeMap<String, ModuleNode>,
}

/// Generate the full module tree from a schema.
pub fn generate_module_tree(schema: &PulumiSchema) -> ModuleTree {
    let naming = NamingContext::new(&schema.name);
    let mut root = ModuleNode::default();

    // Process resources
    for (token, spec) in &schema.resources {
        let module_path = naming.token_to_module_path(token);
        let type_name = naming.token_to_type_name(token);
        let file_name = naming::camel_to_snake(&type_name);

        let gen = resources::generate_resource(token, spec, &naming);

        let node = ensure_module(&mut root, &module_path);
        node.resources.insert(file_name, gen.code);
    }

    // Process functions
    for (token, spec) in &schema.functions {
        let module_path = naming.token_to_module_path(token);
        let type_name = naming.token_to_type_name(token);
        let file_name = naming::camel_to_snake(&type_name);

        let gen = functions::generate_function(token, spec, &naming);

        let node = ensure_module(&mut root, &module_path);
        node.functions.insert(file_name, gen.code);
    }

    // Process types (enums and object types)
    for (token, spec) in &schema.types {
        let module_path = naming.token_to_module_path(token);
        let type_name = naming.token_to_type_name(token);
        let file_name = naming::camel_to_snake(&type_name);

        if let Some(gen) = enums::generate_enum(token, spec, &naming) {
            let node = ensure_module(&mut root, &module_path);
            node.types.insert(file_name, gen.code);
        }
        // TODO: Object type structs could also go here
    }

    // Generate files
    let mut files = Vec::new();

    // Provider resource
    let gen_provider = provider::generate_provider(schema, &naming);
    files.push(GeneratedFile {
        path: "src/provider.rs".to_string(),
        content: gen_provider.code,
    });

    // Recursively emit files for each module
    emit_module_files(&root, &[], &mut files);

    // Generate lib.rs
    let lib_rs = generate_lib_rs(&root);
    files.push(GeneratedFile {
        path: "src/lib.rs".to_string(),
        content: lib_rs,
    });

    // Generate Cargo.toml
    let cargo_toml = generate_cargo_toml(schema, &root);

    ModuleTree { files, cargo_toml }
}

/// Walk the path segments and get-or-create each module node.
fn ensure_module<'a>(root: &'a mut ModuleNode, path: &[String]) -> &'a mut ModuleNode {
    let mut current = root;
    for segment in path {
        current = current
            .children
            .entry(segment.clone())
            .or_default();
    }
    current
}

/// Recursively emit files for a module node.
fn emit_module_files(node: &ModuleNode, prefix: &[String], files: &mut Vec<GeneratedFile>) {
    let dir = if prefix.is_empty() {
        "src".to_string()
    } else {
        format!("src/{}", prefix.join("/"))
    };

    // Emit resource files
    for (name, code) in &node.resources {
        files.push(GeneratedFile {
            path: format!("{dir}/{name}.rs"),
            content: code.clone(),
        });
    }

    // Emit function files into functions/ subdirectory
    if !node.functions.is_empty() {
        let mut fn_mod = String::new();
        for name in node.functions.keys() {
            writeln!(fn_mod, "pub mod {name};").ok();
        }
        files.push(GeneratedFile {
            path: format!("{dir}/functions/mod.rs"),
            content: fn_mod,
        });

        for (name, code) in &node.functions {
            files.push(GeneratedFile {
                path: format!("{dir}/functions/{name}.rs"),
                content: code.clone(),
            });
        }
    }

    // Emit type files into types/ subdirectory
    if !node.types.is_empty() {
        let mut type_mod = String::new();
        for name in node.types.keys() {
            writeln!(type_mod, "pub mod {name};").ok();
        }
        files.push(GeneratedFile {
            path: format!("{dir}/types/mod.rs"),
            content: type_mod,
        });

        for (name, code) in &node.types {
            files.push(GeneratedFile {
                path: format!("{dir}/types/{name}.rs"),
                content: code.clone(),
            });
        }
    }

    // Emit mod.rs for this module (if it has child modules, resources, etc.)
    if !prefix.is_empty() {
        let mod_rs = generate_mod_rs(node);
        files.push(GeneratedFile {
            path: format!("{dir}/mod.rs"),
            content: mod_rs,
        });
    }

    // Recurse into children
    for (child_name, child_node) in &node.children {
        let mut child_prefix = prefix.to_vec();
        child_prefix.push(child_name.clone());
        emit_module_files(child_node, &child_prefix, files);
    }
}

/// Generate mod.rs for a module.
fn generate_mod_rs(node: &ModuleNode) -> String {
    let mut code = String::new();

    // Declare child modules
    for child in node.children.keys() {
        writeln!(code, "pub mod {child};").ok();
    }

    // Declare resource modules
    for name in node.resources.keys() {
        writeln!(code, "pub mod {name};").ok();
    }

    // Declare functions subdirectory
    if !node.functions.is_empty() {
        writeln!(code, "pub mod functions;").ok();
    }

    // Declare types subdirectory
    if !node.types.is_empty() {
        writeln!(code, "pub mod types;").ok();
    }

    code
}

/// Generate the top-level lib.rs.
fn generate_lib_rs(root: &ModuleNode) -> String {
    let mut code = String::new();
    writeln!(code, "//! Auto-generated Pulumi provider crate.").ok();
    writeln!(code).ok();
    writeln!(code, "pub mod provider;").ok();

    // Top-level child modules
    for child in root.children.keys() {
        writeln!(code, "pub mod {child};").ok();
    }

    // Top-level resource files
    for name in root.resources.keys() {
        writeln!(code, "pub mod {name};").ok();
    }

    // Top-level functions
    if !root.functions.is_empty() {
        writeln!(code, "pub mod functions;").ok();
    }

    // Top-level types
    if !root.types.is_empty() {
        writeln!(code, "pub mod types;").ok();
    }

    writeln!(code).ok();
    writeln!(code, "pub use provider::Provider;").ok();
    writeln!(code, "pub use provider::ProviderArgs;").ok();

    code
}

/// Generate Cargo.toml with optional feature flags per top-level module.
fn generate_cargo_toml(schema: &PulumiSchema, root: &ModuleNode) -> String {
    let pkg_name = format!("pulumi-{}", schema.name);
    let version = schema.version.as_deref().unwrap_or("0.1.0");
    let description = schema
        .description
        .as_deref()
        .unwrap_or("Generated Pulumi provider crate");

    let mut toml = String::new();
    writeln!(toml, "[package]").ok();
    writeln!(toml, "name = \"{pkg_name}\"").ok();
    writeln!(toml, "version = \"{version}\"").ok();
    writeln!(toml, "edition = \"2021\"").ok();
    writeln!(toml, "description = \"{description}\"").ok();
    writeln!(toml).ok();
    writeln!(toml, "[dependencies]").ok();
    writeln!(toml, "pulumi-sdk = {{ path = \"../pulumi-sdk\" }}").ok();
    writeln!(toml, "serde = {{ version = \"1\", features = [\"derive\"] }}").ok();
    writeln!(toml, "serde_json = \"1\"").ok();

    // Feature flags for top-level modules (useful for large providers)
    let top_level_modules: BTreeSet<&String> = root.children.keys().collect();
    if top_level_modules.len() > 1 {
        writeln!(toml).ok();
        writeln!(toml, "[features]").ok();
        let all: Vec<_> = top_level_modules.iter().map(|m| format!("\"{m}\"")).collect();
        writeln!(toml, "default = [{}]", all.join(", ")).ok();
        for module in &top_level_modules {
            writeln!(toml, "{module} = []").ok();
        }
    }

    toml
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::*;
    use std::collections::HashMap;

    fn default_prop(typ: &str) -> PropertySpec {
        PropertySpec {
            description: None,
            property_type: Some(typ.to_string()),
            ref_type: None,
            items: None,
            additional_properties: None,
            one_of: None,
            default: None,
            deprecation_message: None,
            secret: None,
            replace_on_changes: None,
            const_value: None,
            will_replace_on_changes: None,
            language: None,
            plain: None,
        }
    }

    fn test_schema() -> PulumiSchema {
        let mut resources = HashMap::new();
        let mut resource_props = HashMap::new();
        resource_props.insert("length".to_string(), default_prop("integer"));

        resources.insert(
            "random:index:RandomString".to_string(),
            ResourceSpec {
                description: Some("A random string.".to_string()),
                input_properties: Some(resource_props.clone()),
                properties: Some(resource_props),
                required_inputs: Some(vec!["length".to_string()]),
                required: Some(vec!["length".to_string()]),
                aliases: None,
                deprecation_message: None,
                is_component: None,
                state_inputs: None, methods: None,
            },
        );

        PulumiSchema {
            name: "random".to_string(),
            version: Some("4.0.0".to_string()),
            description: Some("A random provider.".to_string()),
            display_name: None,
            homepage: None,
            repository: None,
            publisher: None,
            keywords: None,
            license: None,
            logo_url: None,
            plugin_download_url: None,
            config: None,
            provider: None,
            resources,
            functions: Default::default(),
            types: Default::default(),
            language: None,
            meta: None,
        }
    }

    #[test]
    fn test_module_tree_contains_expected_files() {
        let schema = test_schema();
        let tree = generate_module_tree(&schema);

        let paths: Vec<&str> = tree.files.iter().map(|f| f.path.as_str()).collect();

        // Should have lib.rs, provider.rs, and the resource module
        assert!(paths.contains(&"src/lib.rs"), "missing lib.rs");
        assert!(paths.contains(&"src/provider.rs"), "missing provider.rs");
    }

    #[test]
    fn test_lib_rs_declares_modules() {
        let schema = test_schema();
        let tree = generate_module_tree(&schema);

        let lib_rs = tree
            .files
            .iter()
            .find(|f| f.path == "src/lib.rs")
            .expect("lib.rs not found");

        assert!(lib_rs.content.contains("pub mod provider;"));
        assert!(lib_rs.content.contains("pub use provider::Provider;"));
    }

    #[test]
    fn test_cargo_toml_generated() {
        let schema = test_schema();
        let tree = generate_module_tree(&schema);

        assert!(tree.cargo_toml.contains("pulumi-random"));
        assert!(tree.cargo_toml.contains("4.0.0"));
        assert!(tree.cargo_toml.contains("pulumi-sdk"));
    }

    #[test]
    fn test_multi_module_generates_features() {
        let mut resources = HashMap::new();

        let props = {
            let mut p = HashMap::new();
            p.insert("name".to_string(), default_prop("string"));
            p
        };

        resources.insert(
            "azure-native:resources:ResourceGroup".to_string(),
            ResourceSpec {
                description: None,
                input_properties: Some(props.clone()),
                properties: Some(props.clone()),
                required_inputs: None,
                required: None,
                aliases: None,
                deprecation_message: None,
                is_component: None,
                state_inputs: None, methods: None,
            },
        );

        resources.insert(
            "azure-native:network:VirtualNetwork".to_string(),
            ResourceSpec {
                description: None,
                input_properties: Some(props.clone()),
                properties: Some(props),
                required_inputs: None,
                required: None,
                aliases: None,
                deprecation_message: None,
                is_component: None,
                state_inputs: None, methods: None,
            },
        );

        let schema = PulumiSchema {
            name: "azure-native".to_string(),
            version: Some("2.0.0".to_string()),
            description: Some("Azure Native.".to_string()),
            display_name: None,
            homepage: None,
            repository: None,
            publisher: None,
            keywords: None,
            license: None,
            logo_url: None,
            plugin_download_url: None,
            config: None,
            provider: None,
            resources,
            functions: Default::default(),
            types: Default::default(),
            language: None,
            meta: None,
        };

        let tree = generate_module_tree(&schema);

        // Should have feature flags for two modules
        assert!(tree.cargo_toml.contains("[features]"));
        assert!(tree.cargo_toml.contains("network"));
        assert!(tree.cargo_toml.contains("resources"));
    }
}
