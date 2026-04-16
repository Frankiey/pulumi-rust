//! Enum code generation.
//!
//! Generates Rust enums for each enum type in a Pulumi schema,
//! including `as_str()`, `From<T> for serde_json::Value`, `Serialize`, and `Deserialize` impls.

use std::fmt::Write;

use crate::naming::NamingContext;
use crate::schema::ComplexTypeSpec;

/// Generated code for a single enum.
#[derive(Debug)]
pub struct GeneratedEnum {
    /// The enum name (e.g., "SkuName").
    pub name: String,
    /// The Pulumi type token.
    pub token: String,
    /// The generated Rust source code.
    pub code: String,
}

/// A processed enum variant.
struct EnumVariant {
    /// Rust variant name (PascalCase).
    rust_name: String,
    /// The wire value (the actual value sent to Pulumi).
    wire_value: String,
    /// Description, if any.
    description: Option<String>,
}

/// Generate code for a single enum type.
pub fn generate_enum(
    token: &str,
    spec: &ComplexTypeSpec,
    naming: &NamingContext,
) -> Option<GeneratedEnum> {
    let values = spec.enum_values.as_ref()?;
    if values.is_empty() {
        return None;
    }

    let type_name = naming.token_to_type_name(token);
    let is_string = spec.enum_type.as_deref() != Some("integer");

    let variants: Vec<EnumVariant> = values
        .iter()
        .map(|v| {
            let wire_value = match &v.value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                other => other.to_string(),
            };

            let rust_name = if let Some(ref name) = v.name {
                value_to_variant_name(name)
            } else {
                value_to_variant_name(&wire_value)
            };

            EnumVariant {
                rust_name,
                wire_value,
                description: v.description.clone(),
            }
        })
        .collect();

    let mut code = String::new();

    // Doc comment
    if let Some(ref desc) = spec.description {
        for line in desc.lines() {
            writeln!(code, "/// {line}").ok();
        }
    }

    // Enum definition
    writeln!(code, "#[derive(Debug, Clone, PartialEq, Eq)]").ok();
    writeln!(code, "pub enum {type_name} {{").ok();
    for v in &variants {
        if let Some(ref desc) = v.description {
            let first_line = desc.lines().next().unwrap_or("");
            writeln!(code, "    /// {first_line}").ok();
        }
        writeln!(code, "    {},", v.rust_name).ok();
    }
    writeln!(code, "}}").ok();
    writeln!(code).ok();

    // as_str() / as_value() method
    if is_string {
        generate_as_str(&mut code, &type_name, &variants);
    } else {
        generate_as_i64(&mut code, &type_name, &variants);
    }
    writeln!(code).ok();

    // From impl for serde_json::Value
    generate_from_json_value(&mut code, &type_name, is_string);
    writeln!(code).ok();

    // Serialize impl
    generate_serialize(&mut code, &type_name, is_string);
    writeln!(code).ok();

    // Deserialize impl
    generate_deserialize(&mut code, &type_name, &variants, is_string);

    Some(GeneratedEnum {
        name: type_name,
        token: token.to_string(),
        code,
    })
}

/// Convert a wire value or name into a PascalCase Rust variant name.
///
/// Non-alphanumeric characters are treated as word separators.
/// Values starting with a digit are prefixed with `_`.
fn value_to_variant_name(value: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in value.chars() {
        if !ch.is_ascii_alphanumeric() {
            // Any non-alphanumeric character is a word boundary
            capitalize_next = true;
        } else if ch.is_ascii_digit() && result.is_empty() {
            // Prefix with underscore if starts with digit
            result.push('_');
            result.push(ch);
            capitalize_next = false;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    // If empty or starts with non-alpha, prefix
    if result.is_empty() {
        "Unknown".to_string()
    } else {
        result
    }
}

fn generate_as_str(code: &mut String, type_name: &str, variants: &[EnumVariant]) {
    writeln!(code, "impl {type_name} {{").ok();
    writeln!(code, "    /// The wire value sent to the Pulumi engine.").ok();
    writeln!(code, "    pub fn as_str(&self) -> &'static str {{").ok();
    writeln!(code, "        match self {{").ok();
    for v in variants {
        writeln!(
            code,
            "            Self::{} => \"{}\",",
            v.rust_name, v.wire_value
        )
        .ok();
    }
    writeln!(code, "        }}").ok();
    writeln!(code, "    }}").ok();
    writeln!(code, "}}").ok();
}

fn generate_as_i64(code: &mut String, type_name: &str, variants: &[EnumVariant]) {
    writeln!(code, "impl {type_name} {{").ok();
    writeln!(code, "    /// The wire value sent to the Pulumi engine.").ok();
    writeln!(code, "    pub fn as_i64(&self) -> i64 {{").ok();
    writeln!(code, "        match self {{").ok();
    for v in variants {
        writeln!(
            code,
            "            Self::{} => {},",
            v.rust_name, v.wire_value
        )
        .ok();
    }
    writeln!(code, "        }}").ok();
    writeln!(code, "    }}").ok();
    writeln!(code, "}}").ok();
}

fn generate_from_json_value(code: &mut String, type_name: &str, is_string: bool) {
    writeln!(code, "impl From<{type_name}> for serde_json::Value {{").ok();
    writeln!(code, "    fn from(v: {type_name}) -> Self {{").ok();
    if is_string {
        writeln!(
            code,
            "        serde_json::Value::String(v.as_str().to_string())"
        )
        .ok();
    } else {
        writeln!(
            code,
            "        serde_json::Value::Number(serde_json::Number::from(v.as_i64()))"
        )
        .ok();
    }
    writeln!(code, "    }}").ok();
    writeln!(code, "}}").ok();
}

fn generate_serialize(code: &mut String, type_name: &str, is_string: bool) {
    writeln!(code, "impl serde::Serialize for {type_name} {{").ok();
    writeln!(
        code,
        "    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {{"
    )
    .ok();
    if is_string {
        writeln!(code, "        serializer.serialize_str(self.as_str())").ok();
    } else {
        writeln!(code, "        serializer.serialize_i64(self.as_i64())").ok();
    }
    writeln!(code, "    }}").ok();
    writeln!(code, "}}").ok();
}

fn generate_deserialize(
    code: &mut String,
    type_name: &str,
    variants: &[EnumVariant],
    is_string: bool,
) {
    writeln!(code, "impl<'de> serde::Deserialize<'de> for {type_name} {{").ok();
    writeln!(
        code,
        "    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {{"
    )
    .ok();

    if is_string {
        writeln!(code, "        let s = String::deserialize(deserializer)?;").ok();
        writeln!(code, "        match s.as_str() {{").ok();
        for v in variants {
            writeln!(
                code,
                "            \"{}\" => Ok(Self::{}),",
                v.wire_value, v.rust_name
            )
            .ok();
        }
        writeln!(
            code,
            "            _ => Err(serde::de::Error::unknown_variant(&s, &[{}])),",
            variants
                .iter()
                .map(|v| format!("\"{}\"", v.wire_value))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .ok();
    } else {
        writeln!(code, "        let n = i64::deserialize(deserializer)?;").ok();
        writeln!(code, "        match n {{").ok();
        for v in variants {
            writeln!(
                code,
                "            {} => Ok(Self::{}),",
                v.wire_value, v.rust_name
            )
            .ok();
        }
        writeln!(
            code,
            "            _ => Err(serde::de::Error::custom(format!(\"unknown enum value: {{}}\", n))),"
        )
        .ok();
    }

    writeln!(code, "        }}").ok();
    writeln!(code, "    }}").ok();
    writeln!(code, "}}").ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ComplexTypeSpec, EnumValueSpec};

    fn make_string_enum() -> ComplexTypeSpec {
        ComplexTypeSpec {
            properties: None,
            required: None,
            description: Some("The SKU name.".to_string()),
            enum_type: Some("string".to_string()),
            enum_values: Some(vec![
                EnumValueSpec {
                    name: None,
                    value: serde_json::Value::String("Standard_LRS".to_string()),
                    description: Some("Locally redundant.".to_string()),
                    deprecation_message: None,
                },
                EnumValueSpec {
                    name: None,
                    value: serde_json::Value::String("Standard_GRS".to_string()),
                    description: None,
                    deprecation_message: None,
                },
            ]),
        }
    }

    #[test]
    fn test_generate_string_enum() {
        let spec = make_string_enum();
        let naming = NamingContext::new("azure");
        let gen = generate_enum("azure:storage:SkuName", &spec, &naming).unwrap();

        assert_eq!(gen.name, "SkuName");
        assert!(gen.code.contains("pub enum SkuName"));
        assert!(gen.code.contains("StandardLRS"));
        assert!(gen.code.contains("StandardGRS"));
        assert!(gen.code.contains("fn as_str"));
        assert!(gen.code.contains("\"Standard_LRS\""));
        assert!(gen.code.contains("impl serde::Serialize"));
        assert!(gen.code.contains("impl<'de> serde::Deserialize"));
        assert!(gen
            .code
            .contains("impl From<SkuName> for serde_json::Value"));
    }

    #[test]
    fn test_generate_integer_enum() {
        let spec = ComplexTypeSpec {
            properties: None,
            required: None,
            description: None,
            enum_type: Some("integer".to_string()),
            enum_values: Some(vec![
                EnumValueSpec {
                    name: Some("Off".to_string()),
                    value: serde_json::json!(0),
                    description: None,
                    deprecation_message: None,
                },
                EnumValueSpec {
                    name: Some("On".to_string()),
                    value: serde_json::json!(1),
                    description: None,
                    deprecation_message: None,
                },
            ]),
        };

        let naming = NamingContext::new("mypkg");
        let gen = generate_enum("mypkg:index:Toggle", &spec, &naming).unwrap();

        assert!(gen.code.contains("Off"));
        assert!(gen.code.contains("On"));
        assert!(gen.code.contains("fn as_i64"));
        assert!(gen.code.contains("=> 0,"));
        assert!(gen.code.contains("=> 1,"));
    }

    #[test]
    fn test_enum_variant_naming() {
        assert_eq!(value_to_variant_name("Standard_LRS"), "StandardLRS");
        assert_eq!(value_to_variant_name("my-value"), "MyValue");
        assert_eq!(value_to_variant_name("some.dotted"), "SomeDotted");
        assert_eq!(value_to_variant_name("123"), "_123");
    }

    #[test]
    fn test_named_variants_override_value() {
        let spec = ComplexTypeSpec {
            properties: None,
            required: None,
            description: None,
            enum_type: Some("string".to_string()),
            enum_values: Some(vec![EnumValueSpec {
                name: Some("MyCustomName".to_string()),
                value: serde_json::Value::String("wire_value".to_string()),
                description: None,
                deprecation_message: None,
            }]),
        };

        let naming = NamingContext::new("pkg");
        let gen = generate_enum("pkg:index:Foo", &spec, &naming).unwrap();

        assert!(gen.code.contains("MyCustomName"));
        assert!(gen.code.contains("\"wire_value\""));
    }

    #[test]
    fn test_no_enum_values_returns_none() {
        let spec = ComplexTypeSpec {
            properties: None,
            required: None,
            description: None,
            enum_type: None,
            enum_values: None,
        };

        let naming = NamingContext::new("pkg");
        assert!(generate_enum("pkg:index:Foo", &spec, &naming).is_none());
    }
}
