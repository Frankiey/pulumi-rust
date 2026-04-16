//! Pulumi schema JSON deserialization types.
//!
//! These types mirror the Pulumi provider schema specification and support
//! deserializing any provider schema JSON (e.g., pulumi-random, pulumi-aws,
//! pulumi-azure-native).

use std::collections::HashMap;

use serde::Deserialize;

/// Root schema definition for a Pulumi provider package.
#[derive(Debug, Deserialize)]
pub struct PulumiSchema {
    /// The unqualified name of the package (e.g., "random", "aws").
    pub name: String,
    /// The version of the package.
    pub version: Option<String>,
    /// A human-readable description of the package.
    pub description: Option<String>,
    /// The URL of the package's homepage.
    pub homepage: Option<String>,
    /// The URL of the package's source repository.
    pub repository: Option<String>,
    /// The name of the person or organization that authored and published the package.
    pub publisher: Option<String>,
    /// Keyword/tag list for discovery.
    pub keywords: Option<Vec<String>>,
    /// The provider resource definition.
    pub provider: Option<ResourceSpec>,
    /// A map of resource type tokens to resource definitions.
    #[serde(default)]
    pub resources: HashMap<String, ResourceSpec>,
    /// A map of function tokens to function definitions.
    #[serde(default)]
    pub functions: HashMap<String, FunctionSpec>,
    /// A map of type tokens to complex type definitions (objects and enums).
    #[serde(default)]
    pub types: HashMap<String, ComplexTypeSpec>,
    /// Language-specific configuration.
    pub language: Option<HashMap<String, serde_json::Value>>,
    /// Package configuration spec.
    pub config: Option<ConfigSpec>,
    /// Display name for the package.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The logo URL for the package.
    #[serde(rename = "logoUrl")]
    pub logo_url: Option<String>,
    /// License for the package.
    pub license: Option<String>,
    /// Meta section.
    pub meta: Option<MetaSpec>,
    /// Plugin download URL.
    #[serde(rename = "pluginDownloadURL")]
    pub plugin_download_url: Option<String>,
}

/// Metadata about the schema itself.
#[derive(Debug, Deserialize)]
pub struct MetaSpec {
    /// The version of the module format.
    #[serde(rename = "moduleFormat")]
    pub module_format: Option<String>,
}

/// Definition of a Pulumi resource.
#[derive(Debug, Deserialize)]
pub struct ResourceSpec {
    /// Description of the resource.
    pub description: Option<String>,
    /// Input properties for creating/updating the resource.
    #[serde(rename = "inputProperties")]
    pub input_properties: Option<HashMap<String, PropertySpec>>,
    /// Output properties of the resource.
    pub properties: Option<HashMap<String, PropertySpec>>,
    /// Which input properties are required.
    #[serde(rename = "requiredInputs")]
    pub required_inputs: Option<Vec<String>>,
    /// Which output properties are required.
    pub required: Option<Vec<String>>,
    /// Alias definitions for backwards compatibility.
    pub aliases: Option<Vec<AliasSpec>>,
    /// Deprecation notice.
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
    /// Whether this is a component resource.
    #[serde(rename = "isComponent")]
    pub is_component: Option<bool>,
    /// State inputs (for import/refresh).
    #[serde(rename = "stateInputs")]
    pub state_inputs: Option<ObjectTypeSpec>,
    /// Methods available on this resource.
    pub methods: Option<HashMap<String, String>>,
}

/// Specification of a single property.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PropertySpec {
    /// Description of the property.
    pub description: Option<String>,
    /// Primitive type: "string", "integer", "number", "boolean", "array", "object".
    #[serde(rename = "type")]
    pub property_type: Option<String>,
    /// Reference to a named type: "#/types/pkg:mod:TypeName".
    #[serde(rename = "$ref")]
    pub ref_type: Option<String>,
    /// For arrays: the element type.
    pub items: Option<Box<PropertySpec>>,
    /// For maps: the value type.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Box<PropertySpec>>,
    /// Union types.
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<PropertySpec>>,
    /// Default value.
    pub default: Option<serde_json::Value>,
    /// Deprecation notice.
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
    /// Whether this property is secret.
    pub secret: Option<bool>,
    /// Whether changing this property triggers replacement.
    #[serde(rename = "replaceOnChanges")]
    pub replace_on_changes: Option<bool>,
    /// Constant value for this property.
    #[serde(rename = "const")]
    pub const_value: Option<serde_json::Value>,
    /// Whether this property will trigger replacement on changes.
    #[serde(rename = "willReplaceOnChanges")]
    pub will_replace_on_changes: Option<bool>,
    /// Language-specific overrides.
    pub language: Option<HashMap<String, serde_json::Value>>,
    /// Plain type — not wrapped in Input/Output.
    pub plain: Option<bool>,
}

/// Specification of a function (invoke).
#[derive(Debug, Deserialize)]
pub struct FunctionSpec {
    /// Description of the function.
    pub description: Option<String>,
    /// Input arguments (object type).
    pub inputs: Option<ObjectTypeSpec>,
    /// Output properties (object type or ref).
    pub outputs: Option<ObjectTypeSpec>,
    /// Whether the function returns multiple outputs.
    #[serde(rename = "multiOutputs")]
    pub multi_outputs: Option<bool>,
    /// Deprecation notice.
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
}

/// Object type specification (used for function inputs/outputs and nested types).
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectTypeSpec {
    /// Properties of the object.
    pub properties: Option<HashMap<String, PropertySpec>>,
    /// Required property names.
    pub required: Option<Vec<String>>,
    /// Type discriminator.
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    /// Description.
    pub description: Option<String>,
}

/// A complex type — either an object type or an enum type.
/// Discriminated by the presence of `enum_values`.
#[derive(Debug, Deserialize)]
pub struct ComplexTypeSpec {
    // Object fields
    /// Properties of the object type.
    pub properties: Option<HashMap<String, PropertySpec>>,
    /// Required property names.
    pub required: Option<Vec<String>>,
    /// Description.
    pub description: Option<String>,

    // Enum fields
    /// The underlying type for enum values ("string", "integer", etc.).
    #[serde(rename = "type")]
    pub enum_type: Option<String>,
    /// The enum variant definitions.
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<EnumValueSpec>>,
}

impl ComplexTypeSpec {
    /// Returns true if this is an enum type.
    pub fn is_enum(&self) -> bool {
        self.enum_values.is_some()
    }

    /// Returns true if this is an object type.
    pub fn is_object(&self) -> bool {
        self.properties.is_some() && self.enum_values.is_none()
    }
}

/// A single enum variant.
#[derive(Debug, Deserialize)]
pub struct EnumValueSpec {
    /// Optional human-readable name for the variant.
    pub name: Option<String>,
    /// The actual value of the enum variant.
    pub value: serde_json::Value,
    /// Description of this variant.
    pub description: Option<String>,
    /// Deprecation notice.
    #[serde(rename = "deprecationMessage")]
    pub deprecation_message: Option<String>,
}

/// Alias specification for resource backwards compatibility.
#[derive(Debug, Deserialize)]
pub struct AliasSpec {
    /// Alternative name.
    pub name: Option<String>,
    /// Alternative project.
    pub project: Option<String>,
    /// Alternative type token.
    #[serde(rename = "type")]
    pub type_token: Option<String>,
}

/// Package configuration specification.
#[derive(Debug, Deserialize)]
pub struct ConfigSpec {
    /// Configuration variables.
    pub variables: Option<HashMap<String, PropertySpec>>,
    /// Required configuration keys.
    pub required: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal valid schema.
    fn minimal_schema_json() -> &'static str {
        r#"{
            "name": "test-provider",
            "resources": {},
            "functions": {},
            "types": {}
        }"#
    }

    #[test]
    fn test_deserialize_minimal_schema() {
        let schema: PulumiSchema = serde_json::from_str(minimal_schema_json()).unwrap();
        assert_eq!(schema.name, "test-provider");
        assert!(schema.resources.is_empty());
        assert!(schema.functions.is_empty());
        assert!(schema.types.is_empty());
    }

    #[test]
    fn test_deserialize_resource_with_properties() {
        let json = r#"{
            "name": "mypkg",
            "resources": {
                "mypkg:index:MyResource": {
                    "description": "A test resource",
                    "inputProperties": {
                        "name": {
                            "type": "string",
                            "description": "The name"
                        },
                        "count": {
                            "type": "integer"
                        }
                    },
                    "requiredInputs": ["name"],
                    "properties": {
                        "id": { "type": "string" },
                        "name": { "type": "string" },
                        "count": { "type": "integer" }
                    },
                    "required": ["id", "name"]
                }
            },
            "functions": {},
            "types": {}
        }"#;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        let res = &schema.resources["mypkg:index:MyResource"];
        assert_eq!(res.description.as_deref(), Some("A test resource"));

        let inputs = res.input_properties.as_ref().unwrap();
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs["name"].property_type.as_deref(), Some("string"));
        assert_eq!(inputs["count"].property_type.as_deref(), Some("integer"));

        let required = res.required_inputs.as_ref().unwrap();
        assert_eq!(required, &["name"]);
    }

    #[test]
    fn test_deserialize_property_with_ref() {
        let json = r##"{
            "name": "mypkg",
            "resources": {
                "mypkg:index:MyResource": {
                    "inputProperties": {
                        "config": {
                            "$ref": "#/types/mypkg:index:ConfigType",
                            "description": "A referenced type"
                        }
                    }
                }
            },
            "functions": {},
            "types": {
                "mypkg:index:ConfigType": {
                    "properties": {
                        "key": { "type": "string" },
                        "value": { "type": "string" }
                    },
                    "required": ["key"]
                }
            }
        }"##;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        let prop = &schema.resources["mypkg:index:MyResource"]
            .input_properties
            .as_ref()
            .unwrap()["config"];
        assert_eq!(
            prop.ref_type.as_deref(),
            Some("#/types/mypkg:index:ConfigType")
        );

        let complex = &schema.types["mypkg:index:ConfigType"];
        assert!(complex.is_object());
        assert!(!complex.is_enum());
    }

    #[test]
    fn test_deserialize_array_and_map_types() {
        let json = r#"{
            "name": "mypkg",
            "resources": {
                "mypkg:index:MyResource": {
                    "inputProperties": {
                        "tags": {
                            "type": "object",
                            "additionalProperties": { "type": "string" }
                        },
                        "items": {
                            "type": "array",
                            "items": { "type": "integer" }
                        }
                    }
                }
            },
            "functions": {},
            "types": {}
        }"#;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        let inputs = schema.resources["mypkg:index:MyResource"]
            .input_properties
            .as_ref()
            .unwrap();

        // Map type
        let tags = &inputs["tags"];
        assert_eq!(tags.property_type.as_deref(), Some("object"));
        let add_props = tags.additional_properties.as_ref().unwrap();
        assert_eq!(add_props.property_type.as_deref(), Some("string"));

        // Array type
        let items = &inputs["items"];
        assert_eq!(items.property_type.as_deref(), Some("array"));
        let item_type = items.items.as_ref().unwrap();
        assert_eq!(item_type.property_type.as_deref(), Some("integer"));
    }

    #[test]
    fn test_deserialize_enum_type() {
        let json = r#"{
            "name": "mypkg",
            "resources": {},
            "functions": {},
            "types": {
                "mypkg:index:Color": {
                    "type": "string",
                    "enum": [
                        { "name": "Red", "value": "red", "description": "The color red" },
                        { "name": "Green", "value": "green" },
                        { "name": "Blue", "value": "blue" }
                    ]
                }
            }
        }"#;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        let color = &schema.types["mypkg:index:Color"];
        assert!(color.is_enum());
        assert!(!color.is_object());
        assert_eq!(color.enum_type.as_deref(), Some("string"));

        let variants = color.enum_values.as_ref().unwrap();
        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].name.as_deref(), Some("Red"));
        assert_eq!(variants[0].value, serde_json::json!("red"));
        assert_eq!(variants[0].description.as_deref(), Some("The color red"));
    }

    #[test]
    fn test_deserialize_function() {
        let json = r#"{
            "name": "mypkg",
            "resources": {},
            "functions": {
                "mypkg:index:getWidget": {
                    "description": "Get a widget by name",
                    "inputs": {
                        "properties": {
                            "name": { "type": "string" }
                        },
                        "required": ["name"]
                    },
                    "outputs": {
                        "properties": {
                            "id": { "type": "string" },
                            "name": { "type": "string" },
                            "size": { "type": "integer" }
                        },
                        "required": ["id", "name"]
                    }
                }
            },
            "types": {}
        }"#;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        let func = &schema.functions["mypkg:index:getWidget"];
        assert_eq!(func.description.as_deref(), Some("Get a widget by name"));

        let inputs = func.inputs.as_ref().unwrap();
        assert!(inputs.properties.as_ref().unwrap().contains_key("name"));

        let outputs = func.outputs.as_ref().unwrap();
        assert_eq!(outputs.properties.as_ref().unwrap().len(), 3);
        assert_eq!(outputs.required.as_ref().unwrap(), &["id", "name"]);
    }

    #[test]
    fn test_deserialize_one_of() {
        let json = r#"{
            "name": "mypkg",
            "resources": {
                "mypkg:index:MyResource": {
                    "inputProperties": {
                        "value": {
                            "oneOf": [
                                { "type": "string" },
                                { "type": "number" }
                            ]
                        }
                    }
                }
            },
            "functions": {},
            "types": {}
        }"#;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        let prop = &schema.resources["mypkg:index:MyResource"]
            .input_properties
            .as_ref()
            .unwrap()["value"];

        let one_of = prop.one_of.as_ref().unwrap();
        assert_eq!(one_of.len(), 2);
        assert_eq!(one_of[0].property_type.as_deref(), Some("string"));
        assert_eq!(one_of[1].property_type.as_deref(), Some("number"));
    }

    #[test]
    fn test_deserialize_config() {
        let json = r#"{
            "name": "mypkg",
            "resources": {},
            "functions": {},
            "types": {},
            "config": {
                "variables": {
                    "region": {
                        "type": "string",
                        "description": "The AWS region"
                    },
                    "project": {
                        "type": "string"
                    }
                },
                "required": ["region"]
            }
        }"#;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        let config = schema.config.as_ref().unwrap();
        let vars = config.variables.as_ref().unwrap();
        assert_eq!(vars.len(), 2);
        assert!(vars.contains_key("region"));
        assert_eq!(config.required.as_ref().unwrap(), &["region"]);
    }

    #[test]
    fn test_deserialize_pulumi_random_snippet() {
        // A snippet inspired by the real pulumi-random schema
        let json = r#"{
            "name": "random",
            "version": "4.16.0",
            "description": "A Pulumi package for creating random values",
            "resources": {
                "random:index/randomString:RandomString": {
                    "description": "Generates a random string",
                    "inputProperties": {
                        "length": {
                            "type": "integer",
                            "description": "The length of the string desired."
                        },
                        "special": {
                            "type": "boolean",
                            "description": "Include special characters.",
                            "default": true
                        },
                        "upper": {
                            "type": "boolean",
                            "default": true
                        },
                        "lower": {
                            "type": "boolean",
                            "default": true
                        },
                        "numeric": {
                            "type": "boolean",
                            "default": true
                        },
                        "overrideSpecial": {
                            "type": "string"
                        },
                        "keepers": {
                            "type": "object",
                            "additionalProperties": { "type": "string" }
                        }
                    },
                    "requiredInputs": ["length"],
                    "properties": {
                        "id": { "type": "string" },
                        "length": { "type": "integer" },
                        "result": {
                            "type": "string",
                            "description": "The random string generated."
                        },
                        "special": { "type": "boolean" },
                        "upper": { "type": "boolean" },
                        "lower": { "type": "boolean" },
                        "numeric": { "type": "boolean" }
                    },
                    "required": ["id", "result", "length"]
                }
            },
            "functions": {},
            "types": {}
        }"#;

        let schema: PulumiSchema = serde_json::from_str(json).unwrap();
        assert_eq!(schema.name, "random");
        assert_eq!(schema.version.as_deref(), Some("4.16.0"));

        let rs = &schema.resources["random:index/randomString:RandomString"];
        let inputs = rs.input_properties.as_ref().unwrap();
        assert_eq!(inputs.len(), 7);
        assert_eq!(inputs["length"].property_type.as_deref(), Some("integer"));
        assert_eq!(inputs["special"].default, Some(serde_json::json!(true)));

        let required_inputs = rs.required_inputs.as_ref().unwrap();
        assert_eq!(required_inputs, &["length"]);

        let outputs = rs.properties.as_ref().unwrap();
        assert!(outputs.contains_key("result"));
        assert_eq!(rs.required.as_ref().unwrap().len(), 3);
    }
}
