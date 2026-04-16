use crate::output::{Output, OutputState};

/// A reference to an explicit provider instance.
///
/// Provider resources are custom resources of type `pulumi:providers:<pkg>`.
/// Generated provider types wrap this struct and call `register_resource` to
/// create the underlying provider instance.
///
/// The `reference()` method returns the `"urn::id"` string used in gRPC
/// `provider` fields on `RegisterResourceRequest`.
#[derive(Clone)]
pub struct ProviderResource {
    /// The URN of this provider resource.
    pub urn: String,
    /// The ID assigned by the engine.
    pub id: Output<String>,
}

impl ProviderResource {
    /// Create a new `ProviderResource` from a URN and ID output.
    pub fn new(urn: String, id: Output<String>) -> Self {
        Self { urn, id }
    }

    /// The provider reference string (`"urn::id"`) used in gRPC calls.
    ///
    /// If the ID is not yet known (e.g., during preview), the reference
    /// will be `"urn::"`.
    pub async fn reference(&self) -> String {
        let id = match self.id.wait().await {
            OutputState::Known(v) => v,
            _ => String::new(),
        };
        format!("{}::{}", self.urn, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_resource_new() {
        let pr = ProviderResource::new(
            "urn:pulumi:stack::project::pulumi:providers:aws::default".to_string(),
            Output::known("abc-123".to_string()),
        );
        assert_eq!(
            pr.urn,
            "urn:pulumi:stack::project::pulumi:providers:aws::default"
        );
    }

    #[tokio::test]
    async fn test_provider_reference() {
        let pr = ProviderResource::new(
            "urn:pulumi:stack::project::pulumi:providers:aws::default".to_string(),
            Output::known("abc-123".to_string()),
        );
        assert_eq!(
            pr.reference().await,
            "urn:pulumi:stack::project::pulumi:providers:aws::default::abc-123"
        );
    }

    #[tokio::test]
    async fn test_provider_reference_unknown_id() {
        let pr = ProviderResource::new(
            "urn:pulumi:stack::project::pulumi:providers:aws::default".to_string(),
            Output::unknown(vec![]),
        );
        assert_eq!(
            pr.reference().await,
            "urn:pulumi:stack::project::pulumi:providers:aws::default::"
        );
    }
}
