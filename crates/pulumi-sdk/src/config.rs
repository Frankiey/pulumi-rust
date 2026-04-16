use std::collections::{HashMap, HashSet};

use serde::de::DeserializeOwned;

use crate::error::{PulumiError, Result};
use crate::output::Output;

/// Reads Pulumi stack configuration values.
///
/// Configuration is sourced from environment variables set by the language host
/// (`PULUMI_CONFIG` JSON blob) and may be supplemented by direct passing through
/// the [`Context`](crate::Context).
pub struct Config {
    namespace: String,
    values: HashMap<String, String>,
    secret_keys: HashSet<String>,
}

impl Config {
    /// Create a Config reader for the given namespace (typically the project name).
    ///
    /// Reads `PULUMI_CONFIG` (a JSON object) and `PULUMI_CONFIG_SECRET_KEYS`
    /// (a JSON array) from environment variables.
    pub fn new(namespace: &str) -> Self {
        let mut values = HashMap::new();
        let mut secret_keys = HashSet::new();

        // Parse PULUMI_CONFIG JSON blob
        if let Ok(config_json) = std::env::var("PULUMI_CONFIG") {
            if let Ok(map) =
                serde_json::from_str::<HashMap<String, serde_json::Value>>(&config_json)
            {
                for (key, val) in map {
                    let str_val = match val {
                        serde_json::Value::String(s) => s,
                        other => other.to_string(),
                    };
                    values.insert(key, str_val);
                }
            }
        }

        // Parse PULUMI_CONFIG_SECRET_KEYS
        if let Ok(secret_json) = std::env::var("PULUMI_CONFIG_SECRET_KEYS") {
            if let Ok(keys) = serde_json::from_str::<Vec<String>>(&secret_json) {
                secret_keys = keys.into_iter().collect();
            }
        }

        Self {
            namespace: namespace.to_string(),
            values,
            secret_keys,
        }
    }

    /// Create a Config from a pre-built map (useful for testing or component providers).
    pub fn from_map(
        namespace: &str,
        values: HashMap<String, String>,
        secret_keys: HashSet<String>,
    ) -> Self {
        Self {
            namespace: namespace.to_string(),
            values,
            secret_keys,
        }
    }

    /// Fully-qualified key: `{namespace}:{key}`.
    fn full_key(&self, key: &str) -> String {
        format!("{}:{}", self.namespace, key)
    }

    /// Get a plain configuration value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(&self.full_key(key)).map(|s| s.as_str())
    }

    /// Get a required configuration value. Returns an error if missing.
    pub fn require(&self, key: &str) -> Result<&str> {
        self.get(key).ok_or_else(|| PulumiError::MissingConfig {
            namespace: self.namespace.clone(),
            key: key.to_string(),
        })
    }

    /// Get a secret configuration value, returned as an [`Output`] to preserve secrecy.
    pub fn get_secret(&self, key: &str) -> Option<Output<String>> {
        self.get(key)
            .map(|v| Output::known(v.to_string()).with_secret(true))
    }

    /// Get a required secret configuration value.
    pub fn require_secret(&self, key: &str) -> Result<Output<String>> {
        let val = self.require(key)?;
        Ok(Output::known(val.to_string()).with_secret(true))
    }

    /// Get a config value and deserialize from JSON into `T`.
    pub fn get_object<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        match self.get(key) {
            Some(val) => {
                let obj: T = serde_json::from_str(val)?;
                Ok(Some(obj))
            }
            None => Ok(None),
        }
    }

    /// Get a required config object, deserialized from JSON.
    pub fn require_object<T: DeserializeOwned>(&self, key: &str) -> Result<T> {
        let val = self.require(key)?;
        let obj: T = serde_json::from_str(val)?;
        Ok(obj)
    }

    /// Returns whether a key is marked as secret.
    pub fn is_secret(&self, key: &str) -> bool {
        self.secret_keys.contains(&self.full_key(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::OutputState;

    fn test_config() -> Config {
        let mut values = HashMap::new();
        values.insert("myapp:name".to_string(), "test-app".to_string());
        values.insert("myapp:count".to_string(), "42".to_string());
        values.insert(
            "myapp:tags".to_string(),
            r#"{"env":"dev","team":"platform"}"#.to_string(),
        );
        values.insert("myapp:dbPassword".to_string(), "s3cret".to_string());

        let mut secret_keys = HashSet::new();
        secret_keys.insert("myapp:dbPassword".to_string());

        Config::from_map("myapp", values, secret_keys)
    }

    #[test]
    fn test_get() {
        let cfg = test_config();
        assert_eq!(cfg.get("name"), Some("test-app"));
        assert_eq!(cfg.get("count"), Some("42"));
        assert_eq!(cfg.get("nonexistent"), None);
    }

    #[test]
    fn test_require() {
        let cfg = test_config();
        assert_eq!(cfg.require("name").unwrap(), "test-app");
    }

    #[test]
    fn test_require_missing() {
        let cfg = test_config();
        let err = cfg.require("missing").unwrap_err();
        assert!(err
            .to_string()
            .contains("missing required configuration: myapp:missing"));
    }

    #[tokio::test]
    async fn test_get_secret() {
        let cfg = test_config();
        let output = cfg.get_secret("dbPassword").unwrap();
        assert!(output.is_secret());
        match output.wait().await {
            OutputState::Known(v) => assert_eq!(v, "s3cret"),
            _ => panic!("expected Known"),
        }
    }

    #[tokio::test]
    async fn test_require_secret() {
        let cfg = test_config();
        let output = cfg.require_secret("dbPassword").unwrap();
        assert!(output.is_secret());
        match output.wait().await {
            OutputState::Known(v) => assert_eq!(v, "s3cret"),
            _ => panic!("expected Known"),
        }
    }

    #[test]
    fn test_get_object() {
        let cfg = test_config();
        let tags: HashMap<String, String> = cfg.get_object("tags").unwrap().unwrap();
        assert_eq!(tags.get("env").unwrap(), "dev");
        assert_eq!(tags.get("team").unwrap(), "platform");
    }

    #[test]
    fn test_require_object() {
        let cfg = test_config();
        let tags: HashMap<String, String> = cfg.require_object("tags").unwrap();
        assert_eq!(tags.get("env").unwrap(), "dev");
    }

    #[test]
    fn test_get_object_missing() {
        let cfg = test_config();
        let result: Result<Option<HashMap<String, String>>> = cfg.get_object("nonexistent");
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_is_secret() {
        let cfg = test_config();
        assert!(cfg.is_secret("dbPassword"));
        assert!(!cfg.is_secret("name"));
    }
}
