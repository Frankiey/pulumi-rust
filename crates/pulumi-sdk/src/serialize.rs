use prost_types::value::Kind;
use prost_types::{Struct, Value};
use std::collections::{BTreeMap, HashMap};

/// The special signature key used by Pulumi to encode non-trivial values.
const SIG_KEY: &str = "4dabf18193072939515e22adb298388d";

/// Signature value indicating a secret.
const SECRET_SIG: &str = "1b47061264138c4ac30d75fd1eb44270";

/// Sentinel UUID indicating an unknown value during preview.
const UNKNOWN_SENTINEL: &str = "04da6b54-80e4-46f7-96ec-b56ff0331ba9";

/// Signature value indicating an output value.
const OUTPUT_SIG: &str = "d0e6a833031e9bbcd3f4e8bde6ca49a4";

/// Represents a deserialized property value from the Pulumi wire format.
#[derive(Debug, Clone)]
pub struct PropertyValue {
    pub value: serde_json::Value,
    pub known: bool,
    pub secret: bool,
    pub deps: Vec<String>,
}

/// Serialize a serde_json::Value into a prost_types::Value.
pub fn serialize_value(val: &serde_json::Value) -> Value {
    match val {
        serde_json::Value::Null => Value {
            kind: Some(Kind::NullValue(0)),
        },
        serde_json::Value::Bool(b) => Value {
            kind: Some(Kind::BoolValue(*b)),
        },
        serde_json::Value::Number(n) => Value {
            kind: Some(Kind::NumberValue(n.as_f64().unwrap_or(0.0))),
        },
        serde_json::Value::String(s) => Value {
            kind: Some(Kind::StringValue(s.clone())),
        },
        serde_json::Value::Array(arr) => {
            let values = arr.iter().map(serialize_value).collect();
            Value {
                kind: Some(Kind::ListValue(prost_types::ListValue { values })),
            }
        }
        serde_json::Value::Object(map) => {
            let fields = map
                .iter()
                .map(|(k, v)| (k.clone(), serialize_value(v)))
                .collect();
            Value {
                kind: Some(Kind::StructValue(Struct { fields })),
            }
        }
    }
}

/// Serialize a property value, wrapping secrets with the Pulumi special signature.
pub fn serialize_property(val: &serde_json::Value, secret: bool) -> Value {
    let inner = serialize_value(val);

    if secret {
        wrap_secret(inner)
    } else {
        inner
    }
}

/// Wrap a value in a Pulumi secret envelope.
fn wrap_secret(inner: Value) -> Value {
    let mut fields = BTreeMap::new();
    fields.insert(
        SIG_KEY.to_string(),
        Value {
            kind: Some(Kind::StringValue(SECRET_SIG.to_string())),
        },
    );
    fields.insert("value".to_string(), inner);
    Value {
        kind: Some(Kind::StructValue(Struct { fields })),
    }
}

/// Create a Value representing an unknown sentinel.
pub fn serialize_unknown() -> Value {
    Value {
        kind: Some(Kind::StringValue(UNKNOWN_SENTINEL.to_string())),
    }
}

/// Serialize a HashMap of properties into a protobuf Struct.
pub fn serialize_properties(
    props: &HashMap<String, serde_json::Value>,
    secret_keys: &[String],
) -> Struct {
    let fields = props
        .iter()
        .map(|(k, v)| {
            let is_secret = secret_keys.contains(k);
            (k.clone(), serialize_property(v, is_secret))
        })
        .collect();
    Struct { fields }
}

/// Deserialize a prost_types::Value back to a serde_json::Value.
pub fn deserialize_value(val: &Value) -> serde_json::Value {
    match &val.kind {
        None | Some(Kind::NullValue(_)) => serde_json::Value::Null,
        Some(Kind::BoolValue(b)) => serde_json::Value::Bool(*b),
        Some(Kind::NumberValue(n)) => serde_json::Value::Number(
            serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0)),
        ),
        Some(Kind::StringValue(s)) => serde_json::Value::String(s.clone()),
        Some(Kind::ListValue(list)) => {
            let items: Vec<serde_json::Value> = list.values.iter().map(deserialize_value).collect();
            serde_json::Value::Array(items)
        }
        Some(Kind::StructValue(s)) => {
            let map: serde_json::Map<String, serde_json::Value> = s
                .fields
                .iter()
                .map(|(k, v)| (k.clone(), deserialize_value(v)))
                .collect();
            serde_json::Value::Object(map)
        }
    }
}

/// Deserialize a protobuf Value into a PropertyValue, handling Pulumi special encodings.
pub fn deserialize_property(val: &Value) -> PropertyValue {
    // Check for unknown sentinel
    if let Some(Kind::StringValue(s)) = &val.kind {
        if s == UNKNOWN_SENTINEL {
            return PropertyValue {
                value: serde_json::Value::Null,
                known: false,
                secret: false,
                deps: Vec::new(),
            };
        }
    }

    // Check for special signatures (struct with SIG_KEY)
    if let Some(Kind::StructValue(s)) = &val.kind {
        if let Some(sig_val) = s.fields.get(SIG_KEY) {
            if let Some(Kind::StringValue(sig)) = &sig_val.kind {
                // Secret
                if sig == SECRET_SIG {
                    let inner_val =
                        s.fields
                            .get("value")
                            .map(deserialize_property)
                            .unwrap_or(PropertyValue {
                                value: serde_json::Value::Null,
                                known: true,
                                secret: true,
                                deps: Vec::new(),
                            });
                    return PropertyValue {
                        value: inner_val.value,
                        known: inner_val.known,
                        secret: true,
                        deps: inner_val.deps,
                    };
                }

                // Output value
                if sig == OUTPUT_SIG {
                    let inner_value = s
                        .fields
                        .get("value")
                        .map(deserialize_value)
                        .unwrap_or(serde_json::Value::Null);
                    let secret = s
                        .fields
                        .get("secret")
                        .and_then(|v| {
                            if let Some(Kind::BoolValue(b)) = &v.kind {
                                Some(*b)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(false);
                    let deps = s
                        .fields
                        .get("dependencies")
                        .and_then(|v| {
                            if let Some(Kind::ListValue(list)) = &v.kind {
                                Some(
                                    list.values
                                        .iter()
                                        .filter_map(|v| {
                                            if let Some(Kind::StringValue(s)) = &v.kind {
                                                Some(s.clone())
                                            } else {
                                                None
                                            }
                                        })
                                        .collect(),
                                )
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();

                    let known = s.fields.contains_key("value");
                    return PropertyValue {
                        value: inner_value,
                        known,
                        secret,
                        deps,
                    };
                }
            }
        }
    }

    // Plain value
    PropertyValue {
        value: deserialize_value(val),
        known: true,
        secret: false,
        deps: Vec::new(),
    }
}

/// Deserialize a protobuf Struct into a map of PropertyValues.
pub fn deserialize_properties(s: &Struct) -> HashMap<String, PropertyValue> {
    s.fields
        .iter()
        .map(|(k, v)| (k.clone(), deserialize_property(v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_primitives() {
        let val = serialize_value(&serde_json::json!(42));
        assert!(matches!(val.kind, Some(Kind::NumberValue(n)) if (n - 42.0).abs() < f64::EPSILON));

        let val = serialize_value(&serde_json::json!("hello"));
        assert!(matches!(&val.kind, Some(Kind::StringValue(s)) if s == "hello"));

        let val = serialize_value(&serde_json::json!(true));
        assert!(matches!(val.kind, Some(Kind::BoolValue(true))));

        let val = serialize_value(&serde_json::json!(null));
        assert!(matches!(val.kind, Some(Kind::NullValue(_))));
    }

    #[test]
    fn test_serialize_array() {
        let val = serialize_value(&serde_json::json!([1, 2, 3]));
        if let Some(Kind::ListValue(list)) = val.kind {
            assert_eq!(list.values.len(), 3);
        } else {
            panic!("expected ListValue");
        }
    }

    #[test]
    fn test_serialize_object() {
        let val = serialize_value(&serde_json::json!({"a": 1, "b": "two"}));
        if let Some(Kind::StructValue(s)) = val.kind {
            assert_eq!(s.fields.len(), 2);
        } else {
            panic!("expected StructValue");
        }
    }

    #[test]
    fn test_secret_wrapping() {
        let val = serialize_property(&serde_json::json!("secret-value"), true);
        if let Some(Kind::StructValue(s)) = &val.kind {
            assert!(s.fields.contains_key(SIG_KEY));
            let sig = &s.fields[SIG_KEY];
            assert!(matches!(&sig.kind, Some(Kind::StringValue(s)) if s == SECRET_SIG));
        } else {
            panic!("expected StructValue for secret");
        }
    }

    #[test]
    fn test_deserialize_plain() {
        let val = serialize_value(&serde_json::json!("hello"));
        let prop = deserialize_property(&val);
        assert!(prop.known);
        assert!(!prop.secret);
        assert_eq!(prop.value, serde_json::json!("hello"));
    }

    #[test]
    fn test_deserialize_unknown() {
        let val = Value {
            kind: Some(Kind::StringValue(UNKNOWN_SENTINEL.to_string())),
        };
        let prop = deserialize_property(&val);
        assert!(!prop.known);
        assert!(!prop.secret);
    }

    #[test]
    fn test_deserialize_secret() {
        let val = serialize_property(&serde_json::json!("my-secret"), true);
        let prop = deserialize_property(&val);
        assert!(prop.known);
        assert!(prop.secret);
        assert_eq!(prop.value, serde_json::json!("my-secret"));
    }

    #[test]
    fn test_roundtrip() {
        let original = serde_json::json!({
            "name": "test",
            "count": 42.0,
            "tags": ["a", "b"],
            "nested": {"x": true}
        });
        let proto_val = serialize_value(&original);
        let roundtripped = deserialize_value(&proto_val);
        assert_eq!(original, roundtripped);
    }

    #[test]
    fn test_serialize_properties_map() {
        let mut props = HashMap::new();
        props.insert("length".to_string(), serde_json::json!(8));
        props.insert("special".to_string(), serde_json::json!(false));

        let s = serialize_properties(&props, &[]);
        assert_eq!(s.fields.len(), 2);
    }
}
