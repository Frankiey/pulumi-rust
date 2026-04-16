use std::collections::HashMap;

/// Options for invoking a provider function.
#[derive(Debug, Clone, Default)]
pub struct InvokeOptions {
    /// An optional provider reference to use for this invoke.
    pub provider: Option<String>,
    /// The version of the provider to use.
    pub version: Option<String>,
    /// The plugin download URL for the provider.
    pub plugin_download_url: Option<String>,
}

/// The result of a provider function invocation.
#[derive(Debug)]
pub struct InvokeResult {
    /// Deserialized return values from the invoke.
    pub fields: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_invoke_options() {
        let opts = InvokeOptions::default();
        assert!(opts.provider.is_none());
        assert!(opts.version.is_none());
        assert!(opts.plugin_download_url.is_none());
    }

    #[test]
    fn test_invoke_result_fields() {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), serde_json::json!("abc-123"));
        fields.insert("name".to_string(), serde_json::json!("my-resource"));
        let result = InvokeResult { fields };
        assert_eq!(result.fields["id"], serde_json::json!("abc-123"));
        assert_eq!(result.fields["name"], serde_json::json!("my-resource"));
    }
}
