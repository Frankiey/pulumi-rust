/// Structured error types for the Pulumi Rust SDK.
///
/// Library code uses [`PulumiError`] instead of `anyhow::Error`.
/// The [`run()`](crate::run) entrypoint catches all errors and logs them
/// to the engine before exiting.
#[derive(Debug, thiserror::Error)]
pub enum PulumiError {
    /// A gRPC transport or status error.
    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::Status),

    /// A gRPC transport-level connection error.
    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    /// Serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(String),

    /// A required configuration key was not set.
    #[error("missing required configuration: {namespace}:{key}")]
    MissingConfig { namespace: String, key: String },

    /// A resource registration RPC failed.
    #[error("resource registration failed: {0}")]
    ResourceRegistration(String),

    /// A provider invoke RPC failed.
    #[error("invoke failed for {token}: {message}")]
    InvokeFailed { token: String, message: String },

    /// An environment variable required by the SDK was missing.
    #[error("missing environment variable: {0}")]
    MissingEnvVar(String),

    /// A catch-all for domain errors not covered by other variants.
    #[error("{0}")]
    Custom(String),
}

/// A convenience alias used throughout the SDK's public API.
pub type Result<T> = std::result::Result<T, PulumiError>;

impl From<serde_json::Error> for PulumiError {
    fn from(err: serde_json::Error) -> Self {
        PulumiError::Serialization(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_config_display() {
        let err = PulumiError::MissingConfig {
            namespace: "myapp".to_string(),
            key: "dbPassword".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "missing required configuration: myapp:dbPassword"
        );
    }

    #[test]
    fn test_invoke_failed_display() {
        let err = PulumiError::InvokeFailed {
            token: "aws:ec2/getAmi:getAmi".to_string(),
            message: "not found".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "invoke failed for aws:ec2/getAmi:getAmi: not found"
        );
    }

    #[test]
    fn test_custom_display() {
        let err = PulumiError::Custom("something went wrong".to_string());
        assert_eq!(err.to_string(), "something went wrong");
    }

    #[test]
    fn test_serialization_from_serde() {
        let json_err = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
        let pulumi_err: PulumiError = json_err.into();
        assert!(pulumi_err.to_string().contains("serialization error"));
    }

    #[test]
    fn test_missing_env_var() {
        let err = PulumiError::MissingEnvVar("PULUMI_MONITOR".to_string());
        assert_eq!(
            err.to_string(),
            "missing environment variable: PULUMI_MONITOR"
        );
    }
}
