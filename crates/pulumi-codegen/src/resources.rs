//! Resource code generation.
//!
//! Generates Rust source code for each resource in a Pulumi schema:
//! - A `FooArgs` input struct with `Input<T>` fields
//! - A `Foo` output struct with `Output<T>` fields
//! - An async `new()` constructor

use std::fmt::Write;

use crate::naming::NamingContext;
use crate::schema::ResourceSpec;
use crate::types::{self, TypePosition};

/// Generated code for a single resource.
#[derive(Debug)]
pub struct GeneratedResource {
    /// The resource type name (e.g., "ResourceGroup").
    pub name: String,
    /// The args struct name (e.g., "ResourceGroupArgs").
    pub args_name: String,
    /// The Pulumi type token (e.g., "azure-native:resources:ResourceGroup").
    pub type_token: String,
    /// The generated Rust source code.
    pub code: String,
}

/// Generate code for a single resource.
pub fn generate_resource(
    token: &str,
    spec: &ResourceSpec,
    naming: &NamingContext,
) -> GeneratedResource {
    let type_name = naming.token_to_type_name(token);
    let args_name = format!("{type_name}Args");

    let mut code = String::new();

    // Imports
    writeln!(code, "use pulumi_sdk::{{Context, Input, Output, ResourceOptions, Result}};").ok();
    writeln!(code, "use std::collections::HashMap;").ok();
    writeln!(code).ok();

    // Generate Args struct
    generate_args_struct(&mut code, &args_name, spec, naming);
    writeln!(code).ok();

    // Generate output struct
    generate_output_struct(&mut code, &type_name, spec, naming);
    writeln!(code).ok();

    // Generate impl with new() constructor
    generate_impl(&mut code, &type_name, &args_name, token, spec, naming);

    GeneratedResource {
        name: type_name,
        args_name,
        type_token: token.to_string(),
        code,
    }
}

fn generate_args_struct(
    code: &mut String,
    args_name: &str,
    spec: &ResourceSpec,
    naming: &NamingContext,
) {
    // Doc comment
    if let Some(ref desc) = spec.description {
        writeln!(code, "/// Input arguments for creating this resource.").ok();
        writeln!(code, "///").ok();
        for line in desc.lines() {
            writeln!(code, "/// {line}").ok();
        }
    } else {
        writeln!(code, "/// Input arguments for creating this resource.").ok();
    }

    // Only derive Default if there are no required input properties
    let has_required = spec
        .required_inputs
        .as_ref()
        .is_some_and(|v| !v.is_empty());
    if !has_required {
        writeln!(code, "#[derive(Default)]").ok();
    }
    writeln!(code, "pub struct {args_name} {{").ok();

    if let Some(ref input_props) = spec.input_properties {
        let required_set: Vec<&str> = spec
            .required_inputs
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();

        // Sort properties for deterministic output
        let mut props: Vec<(&String, &crate::schema::PropertySpec)> = input_props.iter().collect();
        props.sort_by_key(|(name, _)| *name);

        for (prop_name, prop_spec) in props {
            let is_required = required_set.contains(&prop_name.as_str());
            let field_name = naming.property_to_field_name(prop_name);
            let rust_type = types::property_to_rust_type(prop_spec, TypePosition::Input, naming, is_required);

            // Doc comment for field
            if let Some(ref desc) = prop_spec.description {
                let first_line = desc.lines().next().unwrap_or("");
                writeln!(code, "    /// {first_line}").ok();
            }

            writeln!(code, "    pub {field_name}: {},", rust_type.to_type_string()).ok();
        }
    }

    writeln!(code, "}}").ok();
}

fn generate_output_struct(
    code: &mut String,
    type_name: &str,
    spec: &ResourceSpec,
    naming: &NamingContext,
) {
    if let Some(ref desc) = spec.description {
        for line in desc.lines() {
            writeln!(code, "/// {line}").ok();
        }
    }

    writeln!(code, "pub struct {type_name} {{").ok();
    writeln!(code, "    /// The URN of this resource.").ok();
    writeln!(code, "    pub urn: String,").ok();
    writeln!(code, "    /// The provider-assigned unique ID.").ok();
    writeln!(code, "    pub id: Output<serde_json::Value>,").ok();

    if let Some(ref output_props) = spec.properties {
        let mut props: Vec<(&String, &crate::schema::PropertySpec)> = output_props.iter().collect();
        props.sort_by_key(|(name, _)| *name);

        for (prop_name, prop_spec) in props {
            // Skip id — we always emit it above
            if prop_name == "id" {
                continue;
            }

            let field_name = naming.property_to_field_name(prop_name);

            if let Some(ref desc) = prop_spec.description {
                let first_line = desc.lines().next().unwrap_or("");
                writeln!(code, "    /// {first_line}").ok();
            }

            writeln!(code, "    pub {field_name}: Output<serde_json::Value>,").ok();
        }
    }

    writeln!(code, "}}").ok();
}

fn generate_impl(
    code: &mut String,
    type_name: &str,
    args_name: &str,
    token: &str,
    spec: &ResourceSpec,
    naming: &NamingContext,
) {
    writeln!(code, "impl {type_name} {{").ok();
    writeln!(code, "    const TYPE_TOKEN: &'static str = \"{token}\";").ok();
    writeln!(code).ok();
    writeln!(code, "    /// Create a new {type_name} resource.").ok();
    writeln!(code, "    pub async fn new(").ok();
    writeln!(code, "        ctx: &Context,").ok();
    writeln!(code, "        name: &str,").ok();
    writeln!(code, "        args: {args_name},").ok();
    writeln!(code, "        opts: Option<ResourceOptions>,").ok();
    writeln!(code, "    ) -> Result<Self> {{").ok();
    writeln!(code, "        let opts = opts.unwrap_or_default();").ok();
    writeln!(code, "        let mut inputs = HashMap::new();").ok();
    writeln!(code, "        let mut deps: Vec<String> = Vec::new();").ok();
    writeln!(code, "        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();").ok();
    writeln!(code).ok();

    // Generate resolve_input calls for each input property
    if let Some(ref input_props) = spec.input_properties {
        let required_set: Vec<&str> = spec
            .required_inputs
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();

        let mut props: Vec<(&String, &crate::schema::PropertySpec)> = input_props.iter().collect();
        props.sort_by_key(|(name, _)| *name);

        for (prop_name, prop_spec) in props {
            let is_required = required_set.contains(&prop_name.as_str());
            let field_name = naming.property_to_field_name(prop_name);
            let is_map = prop_spec.property_type.as_deref() == Some("object")
                && prop_spec.additional_properties.is_some();
            let is_vec = prop_spec.property_type.as_deref() == Some("array");

            let resolve_fn = if is_map {
                "pulumi_sdk::resolve_input_map"
            } else if is_vec {
                "pulumi_sdk::resolve_input_vec"
            } else {
                "pulumi_sdk::resolve_input"
            };

            if is_required {
                writeln!(
                    code,
                    "        {resolve_fn}(\"{prop_name}\", args.{field_name}, &mut inputs, &mut deps, &mut prop_deps).await;"
                )
                .ok();
            } else {
                writeln!(code, "        if let Some(v) = args.{field_name} {{").ok();
                writeln!(
                    code,
                    "            {resolve_fn}(\"{prop_name}\", v, &mut inputs, &mut deps, &mut prop_deps).await;"
                )
                .ok();
                writeln!(code, "        }}").ok();
            }
        }
    }

    writeln!(code).ok();
    writeln!(code, "        let registered = ctx.register_resource(").ok();
    writeln!(code, "            Self::TYPE_TOKEN,").ok();
    writeln!(code, "            name,").ok();
    writeln!(code, "            inputs,").ok();
    writeln!(code, "            prop_deps,").ok();
    writeln!(code, "            &opts,").ok();
    writeln!(code, "        ).await?;").ok();
    writeln!(code).ok();

    // Construct the output struct
    writeln!(code, "        Ok(Self {{").ok();
    writeln!(code, "            urn: registered.urn.clone(),").ok();
    writeln!(code, "            id: registered.outputs.get(\"id\")").ok();
    writeln!(code, "                .cloned()").ok();
    writeln!(
        code,
        "                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),"
    )
    .ok();

    if let Some(ref output_props) = spec.properties {
        let mut props: Vec<(&String, &crate::schema::PropertySpec)> = output_props.iter().collect();
        props.sort_by_key(|(name, _)| *name);

        for (prop_name, _) in props {
            if prop_name == "id" {
                continue;
            }
            let field_name = naming.property_to_field_name(prop_name);
            writeln!(
                code,
                "            {field_name}: registered.outputs.get(\"{prop_name}\")"
            )
            .ok();
            writeln!(code, "                .cloned()").ok();
            writeln!(
                code,
                "                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),"
            )
            .ok();
        }
    }

    writeln!(code, "        }})").ok();
    writeln!(code, "    }}").ok();
    writeln!(code, "}}").ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{PropertySpec, ResourceSpec};
    use std::collections::HashMap;

    fn default_prop() -> PropertySpec {
        PropertySpec {
            description: None,
            property_type: None,
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

    #[test]
    fn test_generate_simple_resource() {
        let mut input_props = HashMap::new();
        input_props.insert(
            "length".to_string(),
            PropertySpec {
                property_type: Some("integer".to_string()),
                description: Some("The length of the string.".to_string()),
                ..default_prop()
            },
        );

        let mut output_props = HashMap::new();
        output_props.insert(
            "id".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..default_prop()
            },
        );
        output_props.insert(
            "result".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                description: Some("The generated string.".to_string()),
                ..default_prop()
            },
        );

        let spec = ResourceSpec {
            description: Some("Generates a random string.".to_string()),
            input_properties: Some(input_props),
            properties: Some(output_props),
            required_inputs: Some(vec!["length".to_string()]),
            required: Some(vec!["id".to_string(), "result".to_string()]),
            aliases: None,
            deprecation_message: None,
            is_component: None,
            state_inputs: None,
            methods: None,
        };

        let naming = NamingContext::new("random");
        let gen = generate_resource("random:index:RandomString", &spec, &naming);

        assert_eq!(gen.name, "RandomString");
        assert_eq!(gen.args_name, "RandomStringArgs");
        assert_eq!(gen.type_token, "random:index:RandomString");

        // Verify key parts of generated code
        assert!(gen.code.contains("pub struct RandomStringArgs"));
        assert!(gen.code.contains("pub length: Input<i64>"));
        assert!(gen.code.contains("pub struct RandomString"));
        assert!(gen.code.contains("pub urn: String"));
        assert!(gen.code.contains("pub result: Output<serde_json::Value>"));
        assert!(gen.code.contains("TYPE_TOKEN"));
        assert!(gen.code.contains("ctx.register_resource("));
        assert!(gen.code.contains("resolve_input(\"length\""));
    }

    #[test]
    fn test_optional_input_generates_if_let() {
        let mut input_props = HashMap::new();
        input_props.insert(
            "name".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..default_prop()
            },
        );
        input_props.insert(
            "special".to_string(),
            PropertySpec {
                property_type: Some("boolean".to_string()),
                ..default_prop()
            },
        );

        let spec = ResourceSpec {
            description: None,
            input_properties: Some(input_props),
            properties: None,
            required_inputs: Some(vec!["name".to_string()]),
            required: None,
            aliases: None,
            deprecation_message: None,
            is_component: None,
            state_inputs: None,
            methods: None,
        };

        let naming = NamingContext::new("random");
        let gen = generate_resource("random:index:Foo", &spec, &naming);

        // "name" is required → direct resolve
        assert!(gen.code.contains("resolve_input(\"name\", args.name,"));
        // "special" is optional → if let Some(v)
        assert!(gen.code.contains("if let Some(v) = args.special"));
    }

    #[test]
    fn test_map_property_uses_resolve_input_map() {
        let mut input_props = HashMap::new();
        input_props.insert(
            "tags".to_string(),
            PropertySpec {
                property_type: Some("object".to_string()),
                additional_properties: Some(Box::new(PropertySpec {
                    property_type: Some("string".to_string()),
                    ..default_prop()
                })),
                ..default_prop()
            },
        );

        let spec = ResourceSpec {
            description: None,
            input_properties: Some(input_props),
            properties: None,
            required_inputs: None,
            required: None,
            aliases: None,
            deprecation_message: None,
            is_component: None,
            state_inputs: None,
            methods: None,
        };

        let naming = NamingContext::new("mypkg");
        let gen = generate_resource("mypkg:index:Foo", &spec, &naming);

        assert!(gen.code.contains("resolve_input_map(\"tags\""));
    }
}
