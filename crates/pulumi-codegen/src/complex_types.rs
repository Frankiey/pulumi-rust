//! Complex object type code generation.
//!
//! Generates Rust structs for each object type in a Pulumi schema's `types` section
//! (types that are not enums). These are used as nested property types in resource
//! Args and Output structs.

use std::fmt::Write;

use crate::naming::NamingContext;
use crate::schema::ComplexTypeSpec;
use crate::types::{self, TypePosition};

/// Generated code for a single complex object type.
#[derive(Debug)]
pub struct GeneratedComplexType {
    /// The type name (e.g., "ContainerConfigurationArgs").
    pub name: String,
    /// The Pulumi type token.
    pub token: String,
    /// The generated Rust source code.
    pub code: String,
}

/// Generate code for a single complex object type.
///
/// Returns `None` if the type has no properties (nothing to generate)
/// or if it is an enum (handled by `enums.rs`).
pub fn generate_complex_type(
    token: &str,
    spec: &ComplexTypeSpec,
    naming: &NamingContext,
) -> Option<GeneratedComplexType> {
    if spec.is_enum() {
        return None;
    }
    let properties = spec.properties.as_ref()?;
    if properties.is_empty() {
        return None;
    }

    let type_name = naming.token_to_type_name(token);

    let required_set: Vec<&str> = spec
        .required
        .as_ref()
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();

    // Determine if this is an "Args" (input) type based on the token suffix.
    // Types whose names already end with "Args" are input types.
    let is_args = type_name.ends_with("Args");

    let mut code = String::new();
    let mut body = String::new();

    // Build struct body first, then determine needed imports

    // Struct definition
    if let Some(desc) = &spec.description {
        for line in desc.lines() {
            writeln!(body, "/// {line}").ok();
        }
    }
    writeln!(body, "#[derive(Debug, Clone, Serialize, Deserialize)]").ok();
    writeln!(body, "pub struct {type_name} {{").ok();

    let mut sorted_props: Vec<_> = properties.iter().collect();
    sorted_props.sort_by_key(|(name, _)| *name);

    for (prop_name, prop_spec) in &sorted_props {
        let field_name = naming.property_to_field_name(prop_name);
        let is_required = required_set.contains(&prop_name.as_str());

        // Use Plain position — these structs are already wrapped in Input<> at usage sites
        let rust_type =
            types::property_to_rust_type(prop_spec, TypePosition::Plain, naming, is_required);
        let type_str = rust_type.to_type_string();

        if let Some(desc) = &prop_spec.description {
            let first_line = desc.lines().next().unwrap_or("");
            if !first_line.is_empty() {
                writeln!(body, "    /// {first_line}").ok();
            }
        }
        writeln!(body, "    #[serde(rename = \"{prop_name}\")]").ok();
        if !is_required {
            writeln!(
                body,
                "    #[serde(skip_serializing_if = \"Option::is_none\")]"
            )
            .ok();
        }
        writeln!(body, "    pub {field_name}: {type_str},").ok();
    }

    writeln!(body, "}}").ok();

    // Generate a Default impl if all fields are optional
    let all_optional = sorted_props
        .iter()
        .all(|(name, _)| !required_set.contains(&name.as_str()));
    if all_optional {
        writeln!(body).ok();
        writeln!(body, "impl Default for {type_name} {{").ok();
        writeln!(body, "    fn default() -> Self {{").ok();
        writeln!(body, "        Self {{").ok();
        for (prop_name, _) in &sorted_props {
            let field_name = naming.property_to_field_name(prop_name);
            writeln!(body, "            {field_name}: None,").ok();
        }
        writeln!(body, "        }}").ok();
        writeln!(body, "    }}").ok();
        writeln!(body, "}}").ok();
    }

    // Assemble final code with imports
    writeln!(code, "use serde::{{Serialize, Deserialize}};").ok();
    if body.contains("HashMap") {
        writeln!(code, "use std::collections::HashMap;").ok();
    }
    writeln!(code).ok();
    code.push_str(&body);

    // Also export the name from parent module (re-export)
    let _ = is_args; // suppress unused warning

    Some(GeneratedComplexType {
        name: type_name,
        token: token.to_string(),
        code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ComplexTypeSpec, PropertySpec};
    use std::collections::HashMap;

    fn naming() -> NamingContext {
        NamingContext::new("test-pkg")
    }

    #[test]
    fn test_generate_simple_object_type() {
        let mut properties = HashMap::new();
        properties.insert(
            "name".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..Default::default()
            },
        );
        properties.insert(
            "count".to_string(),
            PropertySpec {
                property_type: Some("integer".to_string()),
                ..Default::default()
            },
        );
        let spec = ComplexTypeSpec {
            properties: Some(properties),
            required: Some(vec!["name".to_string()]),
            description: Some("A test type.".to_string()),
            enum_type: None,
            enum_values: None,
        };
        let gen = generate_complex_type("test-pkg:network:SubnetArgs", &spec, &naming());
        let gen = gen.expect("should generate");
        assert_eq!(gen.name, "SubnetArgs");
        assert!(gen.code.contains("pub struct SubnetArgs"));
        assert!(gen.code.contains("pub name: String,"));
        assert!(gen.code.contains("pub count: Option<i64>,"));
        assert!(gen.code.contains("#[serde(rename = \"name\")]"));
    }

    #[test]
    fn test_skip_enum_type() {
        use crate::schema::EnumValueSpec;
        let spec = ComplexTypeSpec {
            properties: None,
            required: None,
            description: None,
            enum_type: Some("string".to_string()),
            enum_values: Some(vec![EnumValueSpec {
                name: None,
                value: serde_json::Value::String("foo".to_string()),
                description: None,
                deprecation_message: None,
            }]),
        };
        assert!(generate_complex_type("test-pkg:mod:Foo", &spec, &naming()).is_none());
    }

    #[test]
    fn test_all_optional_generates_default() {
        let mut properties = HashMap::new();
        properties.insert(
            "label".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..Default::default()
            },
        );
        let spec = ComplexTypeSpec {
            properties: Some(properties),
            required: None,
            description: None,
            enum_type: None,
            enum_values: None,
        };
        let gen = generate_complex_type("test-pkg:mod:Config", &spec, &naming()).unwrap();
        assert!(gen.code.contains("impl Default for Config"));
    }
}
