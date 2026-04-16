use pulumi_sdk::{Context, Output, ProviderResource, ResourceOptions, Result};
use std::collections::HashMap;
/// Configuration arguments for the provider resource.
#[derive(Default)]
pub struct ProviderArgs {}
/// The provider resource for the random package.
pub struct Provider {
    /// The URN of the provider resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
}
impl Provider {
    const TYPE_TOKEN: &'static str = "pulumi:providers:random";
    /// Create a new provider resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ProviderArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();
        let registered = ctx
            .register_resource(Self::TYPE_TOKEN, name, inputs, prop_deps, &opts)
            .await?;
        Ok(Self {
            urn: registered.urn.clone(),
            id: registered
                .outputs
                .get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
    /// Convert to a ProviderResource for use in ResourceOptions.
    pub fn as_provider_resource(&self) -> ProviderResource {
        ProviderResource::new(
            self.urn.clone(),
            self.id.clone().apply(|v| v.as_str().unwrap_or_default().to_string()),
        )
    }
}
