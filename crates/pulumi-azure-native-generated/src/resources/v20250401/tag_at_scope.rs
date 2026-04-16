use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Wrapper resource for tags API requests and responses.
pub struct TagAtScopeArgs {
    /// The set of tags.
    pub properties: Input<resources::v20250401::TagsArgs>,
    /// The resource scope.
    pub scope: Input<String>,
}

/// Wrapper resource for tags API requests and responses.
pub struct TagAtScope {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The name of the tags wrapper resource.
    pub name: Output<serde_json::Value>,
    /// The set of tags.
    pub properties: Output<serde_json::Value>,
    /// The type of the tags wrapper resource.
    pub type_: Output<serde_json::Value>,
}

impl TagAtScope {
    const TYPE_TOKEN: &'static str = "azure-native:resources/v20250401:TagAtScope";

    /// Create a new TagAtScope resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: TagAtScopeArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("properties", args.properties, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("scope", args.scope, &mut inputs, &mut deps, &mut prop_deps).await;

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            properties: registered.outputs.get("properties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
