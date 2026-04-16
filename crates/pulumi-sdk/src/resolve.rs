//! Input resolution helpers for generated code.
//!
//! These functions convert `Input<T>` values to serialized JSON properties
//! while collecting dependency URNs for the Pulumi engine.

use std::collections::HashMap;

use serde::Serialize;

use crate::input::Input;
use crate::output::OutputState;

/// Resolve a single `Input<T>` field for resource registration.
///
/// 1. Converts the `Input<T>` to an `Output<T>` and waits for its value.
/// 2. Serializes the resolved value to JSON and inserts it into `inputs`.
/// 3. Collects dependency URNs into both `deps` (global) and `prop_deps`
///    (per-property).
pub async fn resolve_input<T>(
    field_name: &str,
    input: Input<T>,
    inputs: &mut HashMap<String, serde_json::Value>,
    deps: &mut Vec<String>,
    prop_deps: &mut HashMap<String, Vec<String>>,
) where
    T: Clone + Send + Sync + Serialize + 'static,
{
    let output = input.into_output();

    // Collect deps from the output
    let output_deps = output.deps().to_vec();
    if !output_deps.is_empty() {
        deps.extend(output_deps.iter().cloned());
        prop_deps
            .entry(field_name.to_string())
            .or_default()
            .extend(output_deps);
    }

    match output.wait().await {
        OutputState::Known(val) => {
            if let Ok(json_val) = serde_json::to_value(&val) {
                inputs.insert(field_name.to_string(), json_val);
            }
        }
        _ => {
            // Unknown or pending — insert null; the engine handles previews
            inputs.insert(field_name.to_string(), serde_json::Value::Null);
        }
    }
}

/// Resolve a map of `Input<V>` values for resource registration.
///
/// Each entry in the map is resolved independently. The serialized map
/// is inserted as a single JSON object under `field_name`.
pub async fn resolve_input_map<V>(
    field_name: &str,
    map: HashMap<String, Input<V>>,
    inputs: &mut HashMap<String, serde_json::Value>,
    deps: &mut Vec<String>,
    prop_deps: &mut HashMap<String, Vec<String>>,
) where
    V: Clone + Send + Sync + Serialize + 'static,
{
    let mut json_map = serde_json::Map::new();

    for (key, input) in map {
        let output = input.into_output();

        let output_deps = output.deps().to_vec();
        if !output_deps.is_empty() {
            deps.extend(output_deps.iter().cloned());
            prop_deps
                .entry(field_name.to_string())
                .or_default()
                .extend(output_deps);
        }

        match output.wait().await {
            OutputState::Known(val) => {
                if let Ok(json_val) = serde_json::to_value(&val) {
                    json_map.insert(key, json_val);
                }
            }
            _ => {
                json_map.insert(key, serde_json::Value::Null);
            }
        }
    }

    inputs.insert(
        field_name.to_string(),
        serde_json::Value::Object(json_map),
    );
}

/// Resolve a vector of `Input<T>` values for resource registration.
///
/// Each element is resolved independently. The serialized array
/// is inserted as a single JSON array under `field_name`.
pub async fn resolve_input_vec<T>(
    field_name: &str,
    vec: Vec<Input<T>>,
    inputs: &mut HashMap<String, serde_json::Value>,
    deps: &mut Vec<String>,
    prop_deps: &mut HashMap<String, Vec<String>>,
) where
    T: Clone + Send + Sync + Serialize + 'static,
{
    let mut json_arr = Vec::new();

    for input in vec {
        let output = input.into_output();

        let output_deps = output.deps().to_vec();
        if !output_deps.is_empty() {
            deps.extend(output_deps.iter().cloned());
            prop_deps
                .entry(field_name.to_string())
                .or_default()
                .extend(output_deps);
        }

        match output.wait().await {
            OutputState::Known(val) => {
                if let Ok(json_val) = serde_json::to_value(&val) {
                    json_arr.push(json_val);
                } else {
                    json_arr.push(serde_json::Value::Null);
                }
            }
            _ => {
                json_arr.push(serde_json::Value::Null);
            }
        }
    }

    inputs.insert(field_name.to_string(), serde_json::Value::Array(json_arr));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::Output;

    #[tokio::test]
    async fn test_resolve_input_plain_value() {
        let mut inputs = HashMap::new();
        let mut deps = Vec::new();
        let mut prop_deps = HashMap::new();

        resolve_input("count", Input::Value(42i64), &mut inputs, &mut deps, &mut prop_deps).await;

        assert_eq!(inputs.get("count"), Some(&serde_json::json!(42)));
        assert!(deps.is_empty());
        assert!(prop_deps.is_empty());
    }

    #[tokio::test]
    async fn test_resolve_input_output_with_deps() {
        let mut inputs = HashMap::new();
        let mut deps = Vec::new();
        let mut prop_deps = HashMap::new();

        let output = Output::known("hello".to_string()).with_deps(vec!["urn:dep1".to_string()]);
        resolve_input(
            "greeting",
            Input::Output(output),
            &mut inputs,
            &mut deps,
            &mut prop_deps,
        )
        .await;

        assert_eq!(inputs.get("greeting"), Some(&serde_json::json!("hello")));
        assert_eq!(deps, vec!["urn:dep1"]);
        assert_eq!(
            prop_deps.get("greeting"),
            Some(&vec!["urn:dep1".to_string()])
        );
    }

    #[tokio::test]
    async fn test_resolve_input_unknown() {
        let mut inputs = HashMap::new();
        let mut deps = Vec::new();
        let mut prop_deps = HashMap::new();

        let output: Output<String> = Output::unknown(vec!["urn:unknown".to_string()]);
        resolve_input(
            "name",
            Input::Output(output),
            &mut inputs,
            &mut deps,
            &mut prop_deps,
        )
        .await;

        assert_eq!(inputs.get("name"), Some(&serde_json::Value::Null));
        assert_eq!(deps, vec!["urn:unknown"]);
    }

    #[tokio::test]
    async fn test_resolve_input_map() {
        let mut inputs = HashMap::new();
        let mut deps = Vec::new();
        let mut prop_deps = HashMap::new();

        let mut map = HashMap::new();
        map.insert("env".to_string(), Input::Value("prod".to_string()));
        map.insert("region".to_string(), Input::Value("us-east-1".to_string()));

        resolve_input_map("tags", map, &mut inputs, &mut deps, &mut prop_deps).await;

        let tags = inputs.get("tags").unwrap();
        assert_eq!(tags.get("env"), Some(&serde_json::json!("prod")));
        assert_eq!(tags.get("region"), Some(&serde_json::json!("us-east-1")));
    }

    #[tokio::test]
    async fn test_resolve_input_vec() {
        let mut inputs = HashMap::new();
        let mut deps = Vec::new();
        let mut prop_deps = HashMap::new();

        let vec = vec![Input::Value(1i64), Input::Value(2i64), Input::Value(3i64)];

        resolve_input_vec("numbers", vec, &mut inputs, &mut deps, &mut prop_deps).await;

        assert_eq!(
            inputs.get("numbers"),
            Some(&serde_json::json!([1, 2, 3]))
        );
    }
}
