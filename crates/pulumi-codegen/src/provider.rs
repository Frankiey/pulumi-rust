//! Provider resource code generation.
//!
//! Generates the Provider resource type from a schema's config/provider section.
//! The provider struct has `new()` and `as_provider_resource()` methods.

use std::fmt::Write;

use crate::naming::NamingContext;
use crate::schema::PulumiSchema;
use crate::types::{self, TypePosition};

/// Generated code for the provider resource.
#[derive(Debug)]
pub struct GeneratedProvider {
    /// The Pulumi type token (e.g., "pulumi:providers:azure-native").
    pub type_token: String,
    /// The generated Rust source code.
    pub code: String,
}

/// Generate the Provider resource from the schema.
pub fn generate_provider(schema: &PulumiSchema, naming: &NamingContext) -> GeneratedProvider {
    let pkg = &schema.name;
    let type_token = format!("pulumi:providers:{pkg}");
    let mut code = String::new();

    // Imports — only include Input if the provider has config properties
    let has_config_props = schema
        .config
        .as_ref()
        .and_then(|c| c.variables.as_ref())
        .is_some_and(|v| !v.is_empty());
    if has_config_props {
        writeln!(code, "use pulumi_sdk::{{Context, Input, Output, ProviderResource, ResourceOptions, Result}};").ok();
    } else {
        writeln!(
            code,
            "use pulumi_sdk::{{Context, Output, ProviderResource, ResourceOptions, Result}};"
        )
        .ok();
    }
    writeln!(code, "use std::collections::HashMap;").ok();
    writeln!(code).ok();

    // Generate ProviderArgs from config (inputProperties)
    generate_provider_args(&mut code, schema, naming);
    writeln!(code).ok();

    // Generate Provider struct
    writeln!(code, "/// The provider resource for the {pkg} package.").ok();
    writeln!(code, "pub struct Provider {{").ok();
    writeln!(code, "    /// The URN of the provider resource.").ok();
    writeln!(code, "    pub urn: String,").ok();
    writeln!(code, "    /// The provider-assigned unique ID.").ok();
    writeln!(code, "    pub id: Output<serde_json::Value>,").ok();
    writeln!(code, "}}").ok();
    writeln!(code).ok();

    // Generate impl
    writeln!(code, "impl Provider {{").ok();
    writeln!(
        code,
        "    const TYPE_TOKEN: &'static str = \"{type_token}\";"
    )
    .ok();
    writeln!(code).ok();
    writeln!(code, "    /// Create a new provider resource.").ok();
    writeln!(code, "    pub async fn new(").ok();
    writeln!(code, "        ctx: &Context,").ok();
    writeln!(code, "        name: &str,").ok();
    writeln!(code, "        args: ProviderArgs,").ok();
    writeln!(code, "        opts: Option<ResourceOptions>,").ok();
    writeln!(code, "    ) -> Result<Self> {{").ok();
    writeln!(code, "        let opts = opts.unwrap_or_default();").ok();
    writeln!(code, "        let mut inputs = HashMap::new();").ok();
    writeln!(code, "        let mut deps: Vec<String> = Vec::new();").ok();
    writeln!(
        code,
        "        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();"
    )
    .ok();
    writeln!(code).ok();

    // Resolve each config input property
    if let Some(ref config) = schema.config {
        if let Some(ref props) = config.variables {
            let required_set: Vec<&str> = config
                .required
                .as_ref()
                .map(|v| v.iter().map(|s| s.as_str()).collect())
                .unwrap_or_default();

            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(name, _)| *name);

            for (prop_name, prop_spec) in sorted {
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
    writeln!(code, "        Ok(Self {{").ok();
    writeln!(code, "            urn: registered.urn.clone(),").ok();
    writeln!(code, "            id: registered.outputs.get(\"id\")").ok();
    writeln!(code, "                .cloned()").ok();
    writeln!(
        code,
        "                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),"
    )
    .ok();
    writeln!(code, "        }})").ok();
    writeln!(code, "    }}").ok();
    writeln!(code).ok();

    // as_provider_resource method
    writeln!(
        code,
        "    /// Convert to a ProviderResource for use in ResourceOptions."
    )
    .ok();
    writeln!(
        code,
        "    pub fn as_provider_resource(&self) -> ProviderResource {{"
    )
    .ok();
    writeln!(code, "        ProviderResource::new(").ok();
    writeln!(code, "            self.urn.clone(),").ok();
    writeln!(
        code,
        "            self.id.clone().apply(|v| v.as_str().unwrap_or_default().to_string()),"
    )
    .ok();
    writeln!(code, "        )").ok();
    writeln!(code, "    }}").ok();
    writeln!(code, "}}").ok();

    GeneratedProvider { type_token, code }
}

fn generate_provider_args(code: &mut String, schema: &PulumiSchema, naming: &NamingContext) {
    writeln!(
        code,
        "/// Configuration arguments for the provider resource."
    )
    .ok();
    writeln!(code, "#[derive(Default)]").ok();
    writeln!(code, "pub struct ProviderArgs {{").ok();

    if let Some(ref config) = schema.config {
        if let Some(ref props) = config.variables {
            let required_set: Vec<&str> = config
                .required
                .as_ref()
                .map(|v| v.iter().map(|s| s.as_str()).collect())
                .unwrap_or_default();

            let mut sorted: Vec<_> = props.iter().collect();
            sorted.sort_by_key(|(name, _)| *name);

            for (prop_name, prop_spec) in sorted {
                let is_required = required_set.contains(&prop_name.as_str());
                let field_name = naming.property_to_field_name(prop_name);
                let rust_type = types::property_to_rust_type(
                    prop_spec,
                    TypePosition::Input,
                    naming,
                    is_required,
                );

                if let Some(ref desc) = prop_spec.description {
                    let first_line = desc.lines().next().unwrap_or("");
                    writeln!(code, "    /// {first_line}").ok();
                }

                writeln!(
                    code,
                    "    pub {field_name}: {},",
                    rust_type.to_type_string()
                )
                .ok();
            }
        }
    }

    writeln!(code, "}}").ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ConfigSpec, PropertySpec, PulumiSchema};
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

    fn minimal_schema(config: Option<ConfigSpec>) -> PulumiSchema {
        PulumiSchema {
            name: "mypkg".to_string(),
            display_name: None,
            version: Some("1.0.0".to_string()),
            description: None,
            homepage: None,
            repository: None,
            publisher: None,
            keywords: None,
            license: None,
            logo_url: None,
            plugin_download_url: None,
            config,
            provider: Default::default(),
            resources: Default::default(),
            functions: Default::default(),
            types: Default::default(),
            language: None,
            meta: None,
        }
    }

    #[test]
    fn test_generate_provider_no_config() {
        let schema = minimal_schema(None);
        let naming = NamingContext::new("mypkg");
        let gen = generate_provider(&schema, &naming);

        assert_eq!(gen.type_token, "pulumi:providers:mypkg");
        assert!(gen.code.contains("pub struct ProviderArgs"));
        assert!(gen.code.contains("pub struct Provider"));
        assert!(gen.code.contains("as_provider_resource"));
        assert!(gen.code.contains("ProviderResource::new("));
    }

    #[test]
    fn test_generate_provider_with_config() {
        let mut vars = HashMap::new();
        vars.insert(
            "region".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                description: Some("The AWS region.".to_string()),
                ..default_prop()
            },
        );
        vars.insert(
            "accessKey".to_string(),
            PropertySpec {
                property_type: Some("string".to_string()),
                ..default_prop()
            },
        );

        let config = ConfigSpec {
            variables: Some(vars),
            required: Some(vec!["region".to_string()]),
        };

        let schema = minimal_schema(Some(config));
        let naming = NamingContext::new("mypkg");
        let gen = generate_provider(&schema, &naming);

        // region is required input
        assert!(gen.code.contains("pub region: Input<String>"));
        // accessKey is optional
        assert!(gen.code.contains("pub access_key: Option<Input<String>>"));
        // resolve_input for required
        assert!(gen.code.contains("resolve_input(\"region\", args.region,"));
        // if let Some for optional
        assert!(gen.code.contains("if let Some(v) = args.access_key"));
    }
}
