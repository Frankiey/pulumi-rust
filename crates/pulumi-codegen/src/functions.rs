//! Function/invoke code generation.
//!
//! Generates Rust source code for each function (invoke) in a Pulumi schema:
//! - A `FooArgs` input struct with plain types (not Input<T>)
//! - A `FooResult` output struct with plain types
//! - An async function that serializes args, calls ctx.invoke(), and deserializes the result

use std::fmt::Write;

use crate::naming::NamingContext;
use crate::schema::FunctionSpec;
use crate::types::{self, TypePosition};

/// Generated code for a single function.
#[derive(Debug)]
pub struct GeneratedFunction {
    /// The function name (snake_case, e.g., "get_resource_group").
    pub fn_name: String,
    /// The args struct name (e.g., "GetResourceGroupArgs").
    pub args_name: String,
    /// The result struct name (e.g., "GetResourceGroupResult").
    pub result_name: String,
    /// The Pulumi function token.
    pub token: String,
    /// The generated Rust source code.
    pub code: String,
}

/// Generate code for a single function/invoke.
pub fn generate_function(
    token: &str,
    spec: &FunctionSpec,
    naming: &NamingContext,
) -> GeneratedFunction {
    let raw_type_name = naming.token_to_type_name(token);
    // Ensure PascalCase for struct names (function tokens often start lowercase)
    let type_name = {
        let mut chars = raw_type_name.chars();
        match chars.next() {
            Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
            None => raw_type_name.clone(),
        }
    };
    let fn_name = crate::naming::camel_to_snake(&type_name);
    let args_name = format!("{type_name}Args");
    let result_name = format!("{type_name}Result");

    let mut code = String::new();

    // Imports
    writeln!(code, "use pulumi_sdk::{{Context, InvokeOptions, Result}};").ok();
    writeln!(code, "use serde_json::json;").ok();
    writeln!(code, "use std::collections::HashMap;").ok();
    writeln!(code).ok();

    // Generate Args struct
    generate_args_struct(&mut code, &args_name, spec, naming);
    writeln!(code).ok();

    // Generate Result struct
    generate_result_struct(&mut code, &result_name, spec, naming);
    writeln!(code).ok();

    // Generate the invoke function
    generate_invoke_fn(&mut code, &fn_name, &args_name, &result_name, token, spec, naming);

    GeneratedFunction {
        fn_name,
        args_name,
        result_name,
        token: token.to_string(),
        code,
    }
}

fn generate_args_struct(
    code: &mut String,
    args_name: &str,
    spec: &FunctionSpec,
    naming: &NamingContext,
) {
    if let Some(ref desc) = spec.description {
        writeln!(code, "/// Input arguments for this function.").ok();
        writeln!(code, "///").ok();
        for line in desc.lines().take(3) {
            writeln!(code, "/// {line}").ok();
        }
    } else {
        writeln!(code, "/// Input arguments for this function.").ok();
    }

    writeln!(code, "#[derive(Default)]").ok();
    writeln!(code, "pub struct {args_name} {{").ok();

    if let Some(ref inputs) = spec.inputs {
        let required_set: Vec<&str> = inputs
            .required
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();

        if let Some(ref props) = inputs.properties {
            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(name, _)| *name);

            for (prop_name, prop_spec) in sorted {
                let is_required = required_set.contains(&prop_name.as_str());
                let field_name = naming.property_to_field_name(prop_name);
                // Plain types for invokes — no Input/Output wrapping
                let rust_type = types::property_to_rust_type(
                    prop_spec,
                    TypePosition::Plain,
                    naming,
                    is_required,
                );

                if let Some(ref desc) = prop_spec.description {
                    let first_line = desc.lines().next().unwrap_or("");
                    writeln!(code, "    /// {first_line}").ok();
                }

                writeln!(code, "    pub {field_name}: {},", rust_type.to_type_string()).ok();
            }
        }
    }

    writeln!(code, "}}").ok();
}

fn generate_result_struct(
    code: &mut String,
    result_name: &str,
    spec: &FunctionSpec,
    naming: &NamingContext,
) {
    writeln!(code, "/// Result of the function invocation.").ok();
    writeln!(code, "pub struct {result_name} {{").ok();

    if let Some(ref outputs) = spec.outputs {
        let required_set: Vec<&str> = outputs
            .required
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();

        if let Some(ref props) = outputs.properties {
            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(name, _)| *name);

            for (prop_name, prop_spec) in sorted {
                let is_required = required_set.contains(&prop_name.as_str());
                let field_name = naming.property_to_field_name(prop_name);
                let rust_type = types::property_to_rust_type(
                    prop_spec,
                    TypePosition::Plain,
                    naming,
                    is_required,
                );

                if let Some(ref desc) = prop_spec.description {
                    let first_line = desc.lines().next().unwrap_or("");
                    writeln!(code, "    /// {first_line}").ok();
                }

                writeln!(code, "    pub {field_name}: {},", rust_type.to_type_string()).ok();
            }
        }
    }

    writeln!(code, "}}").ok();
}

fn generate_invoke_fn(
    code: &mut String,
    fn_name: &str,
    args_name: &str,
    result_name: &str,
    token: &str,
    spec: &FunctionSpec,
    naming: &NamingContext,
) {
    if let Some(ref desc) = spec.description {
        let first_line = desc.lines().next().unwrap_or("");
        writeln!(code, "/// {first_line}").ok();
    }

    writeln!(code, "pub async fn {fn_name}(").ok();
    writeln!(code, "    ctx: &Context,").ok();
    writeln!(code, "    args: {args_name},").ok();
    writeln!(code, "    opts: Option<InvokeOptions>,").ok();
    writeln!(code, ") -> Result<{result_name}> {{").ok();
    writeln!(code, "    let mut invoke_args = HashMap::new();").ok();

    // Serialize each input argument
    if let Some(ref inputs) = spec.inputs {
        let required_set: Vec<&str> = inputs
            .required
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();

        if let Some(ref props) = inputs.properties {
            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(name, _)| *name);

            for (prop_name, _) in sorted {
                let is_required = required_set.contains(&prop_name.as_str());
                let field_name = naming.property_to_field_name(prop_name);

                if is_required {
                    writeln!(
                        code,
                        "    invoke_args.insert(\"{prop_name}\".to_string(), json!(args.{field_name}));"
                    )
                    .ok();
                } else {
                    writeln!(code, "    if let Some(v) = args.{field_name} {{").ok();
                    writeln!(
                        code,
                        "        invoke_args.insert(\"{prop_name}\".to_string(), json!(v));"
                    )
                    .ok();
                    writeln!(code, "    }}").ok();
                }
            }
        }
    }

    writeln!(code).ok();
    writeln!(
        code,
        "    let opts = opts.unwrap_or_default();"
    )
    .ok();
    writeln!(
        code,
        "    let result = ctx.invoke(\"{token}\", invoke_args, &opts).await?;"
    )
    .ok();
    writeln!(code).ok();

    // Deserialize the result
    writeln!(code, "    Ok({result_name} {{").ok();

    if let Some(ref outputs) = spec.outputs {
        let required_set: Vec<&str> = outputs
            .required
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();

        if let Some(ref props) = outputs.properties {
            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(name, _)| *name);

            for (prop_name, _) in sorted {
                let is_required = required_set.contains(&prop_name.as_str());
                let field_name = naming.property_to_field_name(prop_name);

                if is_required {
                    writeln!(
                        code,
                        "        {field_name}: serde_json::from_value(result.fields.get(\"{prop_name}\").cloned().unwrap_or_default())?"
                    )
                    .ok();
                    writeln!(code, "            ,").ok();
                } else {
                    writeln!(
                        code,
                        "        {field_name}: result.fields.get(\"{prop_name}\").cloned().map(serde_json::from_value).transpose()?,"
                    )
                    .ok();
                }
            }
        }
    }

    writeln!(code, "    }})").ok();
    writeln!(code, "}}").ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{FunctionSpec, ObjectTypeSpec, PropertySpec};
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
    fn test_generate_simple_function() {
        let mut input_props = HashMap::new();
        input_props.insert(
            "name".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                description: Some("The name to look up.".to_string()),
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
            "location".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..default_prop()
            },
        );

        let spec = FunctionSpec {
            description: Some("Gets a resource group.".to_string()),
            inputs: Some(ObjectTypeSpec {
                properties: Some(input_props),
                required: Some(vec!["name".to_string()]),
                description: None,
                type_name: None,
            }),
            outputs: Some(ObjectTypeSpec {
                properties: Some(output_props),
                required: Some(vec!["id".to_string(), "location".to_string()]),
                description: None,
                type_name: None,
            }),
            deprecation_message: None,
            multi_outputs: None,
        };

        let naming = NamingContext::new("azure");
        let gen = generate_function("azure:resources:getResourceGroup", &spec, &naming);

        assert_eq!(gen.fn_name, "get_resource_group");
        assert_eq!(gen.args_name, "GetResourceGroupArgs");
        assert_eq!(gen.result_name, "GetResourceGroupResult");

        assert!(gen.code.contains("pub struct GetResourceGroupArgs"));
        assert!(gen.code.contains("pub name: String"));
        assert!(gen.code.contains("pub struct GetResourceGroupResult"));
        assert!(gen.code.contains("pub async fn get_resource_group("));
        assert!(gen.code.contains("ctx.invoke("));
        assert!(gen.code.contains("invoke_args.insert(\"name\""));
    }

    #[test]
    fn test_optional_args_generate_if_let() {
        let mut input_props = HashMap::new();
        input_props.insert(
            "name".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..default_prop()
            },
        );
        input_props.insert(
            "filter".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..default_prop()
            },
        );

        let spec = FunctionSpec {
            description: None,
            inputs: Some(ObjectTypeSpec {
                properties: Some(input_props),
                required: Some(vec!["name".to_string()]),
                description: None,
                type_name: None,
            }),
            outputs: None,
            deprecation_message: None,
            multi_outputs: None,
        };

        let naming = NamingContext::new("mypkg");
        let gen = generate_function("mypkg:index:lookup", &spec, &naming);

        // "name" required → direct insert
        assert!(gen.code.contains("invoke_args.insert(\"name\""));
        // "filter" optional → if let Some
        assert!(gen.code.contains("if let Some(v) = args.filter"));
    }
}
