use std::collections::HashMap;

use crate::context::Context;
use crate::error::{PulumiError, Result};
use crate::output::Output;
use crate::resource_options::ResourceOptions;

/// A reference to another Pulumi stack's outputs.
///
/// Registered as a resource of type `pulumi:pulumi:StackReference`.
pub struct StackReference {
    /// The URN of this stack reference resource.
    pub urn: String,
    outputs: HashMap<String, Output<serde_json::Value>>,
}

impl StackReference {
    /// Create a new stack reference to read outputs from another stack.
    ///
    /// `name` should be the fully-qualified stack name
    /// (e.g., `org/project/stack`).
    pub async fn new(ctx: &Context, name: &str) -> Result<Self> {
        let mut inputs = HashMap::new();
        inputs.insert(
            "name".to_string(),
            serde_json::Value::String(name.to_string()),
        );

        let registered = ctx
            .register_resource(
                "pulumi:pulumi:StackReference",
                name,
                inputs,
                HashMap::new(),
                &ResourceOptions::default(),
            )
            .await?;

        Ok(Self {
            urn: registered.urn,
            outputs: registered.outputs,
        })
    }

    /// Get an output value by name.
    ///
    /// Returns `Output::unknown()` if the key is not found.
    pub fn get_output(&self, name: &str) -> Output<serde_json::Value> {
        self.outputs
            .get(name)
            .cloned()
            .unwrap_or_else(|| Output::unknown(vec![]))
    }

    /// Get an output value by name, returning an error if not found.
    pub fn require_output(&self, name: &str) -> Result<Output<serde_json::Value>> {
        self.outputs.get(name).cloned().ok_or_else(|| {
            PulumiError::Custom(format!(
                "stack reference output '{name}' not found in '{}'",
                self.urn
            ))
        })
    }

    /// Get an output value by name, marked as secret.
    ///
    /// Returns `Output::unknown()` if the key is not found.
    pub fn get_secret_output(&self, name: &str) -> Output<serde_json::Value> {
        self.get_output(name).with_secret(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_missing_returns_unknown() {
        let sr = StackReference {
            urn: "urn:pulumi:stack::project::pulumi:pulumi:StackReference::ref".to_string(),
            outputs: HashMap::new(),
        };
        let out = sr.get_output("missing");
        assert!(!out.is_secret());
    }

    #[test]
    fn test_require_output_missing_errors() {
        let sr = StackReference {
            urn: "urn:pulumi:stack::project::pulumi:pulumi:StackReference::ref".to_string(),
            outputs: HashMap::new(),
        };
        let result = sr.require_output("missing");
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("not found"));
    }

    #[test]
    fn test_get_secret_output_is_secret() {
        let mut outputs = HashMap::new();
        outputs.insert(
            "key".to_string(),
            Output::known(serde_json::Value::String("val".to_string())),
        );
        let sr = StackReference {
            urn: "urn:pulumi:stack::project::pulumi:pulumi:StackReference::ref".to_string(),
            outputs,
        };
        let out = sr.get_secret_output("key");
        assert!(out.is_secret());
    }
}
