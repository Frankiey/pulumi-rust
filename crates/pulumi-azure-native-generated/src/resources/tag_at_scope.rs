use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Wrapper resource for tags API requests and responses.
/// 
/// Uses Azure REST API version 2024-03-01. In version 2.x of the Azure Native provider, it used API version 2022-09-01.
/// 
/// Other available API versions: 2020-10-01, 2021-01-01, 2021-04-01, 2022-09-01, 2023-07-01, 2024-07-01, 2024-11-01, 2025-03-01, 2025-04-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native resources [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct TagAtScopeArgs {
    /// The set of tags.
    pub properties: Input<resources::TagsArgs>,
    /// The resource scope.
    pub scope: Input<String>,
}

/// Wrapper resource for tags API requests and responses.
/// 
/// Uses Azure REST API version 2024-03-01. In version 2.x of the Azure Native provider, it used API version 2022-09-01.
/// 
/// Other available API versions: 2020-10-01, 2021-01-01, 2021-04-01, 2022-09-01, 2023-07-01, 2024-07-01, 2024-11-01, 2025-03-01, 2025-04-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native resources [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
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
    const TYPE_TOKEN: &'static str = "azure-native:resources:TagAtScope";

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
