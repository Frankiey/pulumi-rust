//! Schema type to Rust type mapping.
//!
//! Maps Pulumi schema property types to Rust type representations,
//! generating both Input variants (for resource args) and Output variants
//! (for resource outputs).

use crate::naming::NamingContext;
use crate::schema::PropertySpec;

/// A Rust type representation that can be rendered to code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    /// A primitive type: String, i64, f64, bool.
    Primitive(String),
    /// Vec<T>
    Vec(Box<RustType>),
    /// HashMap<String, V>
    Map(Box<RustType>),
    /// Option<T>
    Option(Box<RustType>),
    /// Input<T> — for resource input properties.
    Input(Box<RustType>),
    /// Output<T> — for resource output properties.
    Output(Box<RustType>),
    /// A named type from codegen (e.g., `AddressSpaceArgs`, `AddressSpace`).
    Named(NamedType),
    /// serde_json::Value — fallback for unresolvable types.
    Json,
}

/// A reference to a named generated type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedType {
    /// Module path segments (e.g., `["network", "types"]`).
    pub module_path: Vec<String>,
    /// The type name (e.g., `"SubnetType"` or `"SubnetTypeArgs"`).
    pub name: String,
}

impl RustType {
    /// Render this type to a Rust type string.
    pub fn to_type_string(&self) -> String {
        match self {
            RustType::Primitive(s) => s.clone(),
            RustType::Vec(inner) => format!("Vec<{}>", inner.to_type_string()),
            RustType::Map(inner) => {
                format!("HashMap<String, {}>", inner.to_type_string())
            }
            RustType::Option(inner) => format!("Option<{}>", inner.to_type_string()),
            RustType::Input(inner) => format!("Input<{}>", inner.to_type_string()),
            RustType::Output(inner) => format!("Output<{}>", inner.to_type_string()),
            RustType::Named(named) => {
                if named.module_path.is_empty() {
                    named.name.clone()
                } else {
                    format!("{}::{}", named.module_path.join("::"), named.name)
                }
            }
            RustType::Json => "serde_json::Value".to_string(),
        }
    }
}

/// Which side of the API we're generating types for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypePosition {
    /// Input (resource args) — wraps fields in Input<T>.
    Input,
    /// Output (resource outputs) — wraps fields in Output<T>.
    Output,
    /// Plain — no Input/Output wrapping (used for function invoke args/results).
    Plain,
}

/// Map a PropertySpec to a RustType.
///
/// `position` controls whether fields get wrapped in `Input<T>` or `Output<T>`.
/// `naming` provides the package context for resolving $ref types.
/// `is_required` controls whether the type gets wrapped in `Option<>`.
pub fn property_to_rust_type(
    prop: &PropertySpec,
    position: TypePosition,
    naming: &NamingContext,
    is_required: bool,
) -> RustType {
    let inner = resolve_property_type(prop, position, naming);
    let wrapped = wrap_for_position(inner, position, prop.plain.unwrap_or(false));

    if is_required {
        wrapped
    } else {
        RustType::Option(Box::new(wrapped))
    }
}

/// Resolve the base type of a property (without Input/Output/Option wrapping).
fn resolve_property_type(
    prop: &PropertySpec,
    position: TypePosition,
    naming: &NamingContext,
) -> RustType {
    // Check for $ref first
    if let Some(ref ref_type) = prop.ref_type {
        return resolve_ref_type(ref_type, position, naming);
    }

    // Check for oneOf
    if let Some(ref one_of) = prop.one_of {
        if !one_of.is_empty() {
            // For oneOf, use serde_json::Value as a pragmatic approach.
            // Full union type generation could be added later.
            return RustType::Json;
        }
    }

    // Map primitive types
    match prop.property_type.as_deref() {
        Some("string") => RustType::Primitive("String".to_string()),
        Some("integer") => RustType::Primitive("i64".to_string()),
        Some("number") => RustType::Primitive("f64".to_string()),
        Some("boolean") => RustType::Primitive("bool".to_string()),
        Some("array") => {
            let item_type = prop
                .items
                .as_ref()
                .map(|items| resolve_property_type(items, position, naming))
                .unwrap_or(RustType::Json);

            let element = if position == TypePosition::Input && !prop.plain.unwrap_or(false) {
                RustType::Input(Box::new(item_type))
            } else {
                item_type
            };
            RustType::Vec(Box::new(element))
        }
        Some("object") => {
            if let Some(ref add_props) = prop.additional_properties {
                let value_type = resolve_property_type(add_props, position, naming);
                let element = if position == TypePosition::Input && !prop.plain.unwrap_or(false) {
                    RustType::Input(Box::new(value_type))
                } else {
                    value_type
                };
                RustType::Map(Box::new(element))
            } else {
                // Plain object without additionalProperties — treat as JSON
                RustType::Json
            }
        }
        _ => RustType::Json,
    }
}

/// Resolve a `$ref` reference to a Rust type.
fn resolve_ref_type(ref_str: &str, position: TypePosition, naming: &NamingContext) -> RustType {
    // Special built-in refs
    match ref_str {
        "pulumi.json#/Archive" => return RustType::Primitive("AssetOrArchive".to_string()),
        "pulumi.json#/Asset" => return RustType::Primitive("AssetOrArchive".to_string()),
        "pulumi.json#/Any" => return RustType::Json,
        _ => {}
    }

    // Strip the "#/types/" prefix
    let token = if let Some(stripped) = ref_str.strip_prefix("#/types/") {
        stripped
    } else {
        return RustType::Json;
    };

    let module_path = naming.token_to_module_path(token);
    let base_name = naming.token_to_type_name(token);

    let name = match position {
        TypePosition::Input => format!("{base_name}Args"),
        TypePosition::Output | TypePosition::Plain => base_name,
    };

    RustType::Named(NamedType { module_path, name })
}

/// Wrap a base type with Input/Output based on position.
///
/// For container types (Vec, Map) in Input position, the elements are already
/// individually wrapped in Input, so we skip the outer Input wrapping.
fn wrap_for_position(ty: RustType, position: TypePosition, is_plain: bool) -> RustType {
    if is_plain || position == TypePosition::Plain {
        return ty;
    }
    match position {
        TypePosition::Input => {
            // Vec<Input<T>> and HashMap<String, Input<T>> are already Input-aware
            // containers — don't add another Input wrapper around them.
            match &ty {
                RustType::Vec(inner) if matches!(inner.as_ref(), RustType::Input(_)) => ty,
                RustType::Map(inner) if matches!(inner.as_ref(), RustType::Input(_)) => ty,
                _ => RustType::Input(Box::new(ty)),
            }
        }
        TypePosition::Output => RustType::Output(Box::new(ty)),
        TypePosition::Plain => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naming() -> NamingContext {
        NamingContext::new("mypkg")
    }

    fn prop_string() -> PropertySpec {
        PropertySpec {
            property_type: Some("string".to_string()),
            ..default_prop()
        }
    }

    fn prop_integer() -> PropertySpec {
        PropertySpec {
            property_type: Some("integer".to_string()),
            ..default_prop()
        }
    }

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
    fn test_primitive_input_required() {
        let ty = property_to_rust_type(&prop_string(), TypePosition::Input, &naming(), true);
        assert_eq!(ty.to_type_string(), "Input<String>");
    }

    #[test]
    fn test_primitive_input_optional() {
        let ty = property_to_rust_type(&prop_string(), TypePosition::Input, &naming(), false);
        assert_eq!(ty.to_type_string(), "Option<Input<String>>");
    }

    #[test]
    fn test_primitive_output_required() {
        let ty = property_to_rust_type(&prop_string(), TypePosition::Output, &naming(), true);
        assert_eq!(ty.to_type_string(), "Output<String>");
    }

    #[test]
    fn test_integer_mapping() {
        let ty = property_to_rust_type(&prop_integer(), TypePosition::Input, &naming(), true);
        assert_eq!(ty.to_type_string(), "Input<i64>");
    }

    #[test]
    fn test_array_input() {
        let prop = PropertySpec {
            property_type: Some("array".to_string()),
            items: Some(Box::new(prop_string())),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Input, &naming(), true);
        assert_eq!(ty.to_type_string(), "Vec<Input<String>>");
    }

    #[test]
    fn test_array_output() {
        let prop = PropertySpec {
            property_type: Some("array".to_string()),
            items: Some(Box::new(prop_string())),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Output, &naming(), true);
        assert_eq!(ty.to_type_string(), "Output<Vec<String>>");
    }

    #[test]
    fn test_map_input() {
        let prop = PropertySpec {
            property_type: Some("object".to_string()),
            additional_properties: Some(Box::new(prop_string())),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Input, &naming(), true);
        assert_eq!(
            ty.to_type_string(),
            "HashMap<String, Input<String>>"
        );
    }

    #[test]
    fn test_ref_type_input() {
        let prop = PropertySpec {
            ref_type: Some("#/types/mypkg:network:SubnetType".to_string()),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Input, &naming(), true);
        assert_eq!(ty.to_type_string(), "Input<network::SubnetTypeArgs>");
    }

    #[test]
    fn test_ref_type_output() {
        let prop = PropertySpec {
            ref_type: Some("#/types/mypkg:network:SubnetType".to_string()),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Output, &naming(), true);
        assert_eq!(ty.to_type_string(), "Output<network::SubnetType>");
    }

    #[test]
    fn test_one_of_fallback() {
        let prop = PropertySpec {
            one_of: Some(vec![prop_string(), prop_integer()]),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Input, &naming(), true);
        assert_eq!(ty.to_type_string(), "Input<serde_json::Value>");
    }

    #[test]
    fn test_plain_type_no_wrapping() {
        let prop = PropertySpec {
            property_type: Some("string".to_string()),
            plain: Some(true),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Input, &naming(), true);
        assert_eq!(ty.to_type_string(), "String");
    }

    #[test]
    fn test_builtin_ref_archive() {
        let prop = PropertySpec {
            ref_type: Some("pulumi.json#/Archive".to_string()),
            ..default_prop()
        };
        let ty = property_to_rust_type(&prop, TypePosition::Input, &naming(), true);
        assert_eq!(ty.to_type_string(), "Input<AssetOrArchive>");
    }
}
