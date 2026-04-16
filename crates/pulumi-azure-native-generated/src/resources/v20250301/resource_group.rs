use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Resource group information.
#[derive(Default)]
pub struct ResourceGroupArgs {
    /// The location of the resource group. It cannot be changed after the resource group has been created. It must be one of the supported Azure locations.
    pub location: Option<Input<String>>,
    /// The ID of the resource that manages this resource group.
    pub managed_by: Option<Input<String>>,
    /// The name of the resource group to create or update. Can include alphanumeric, underscore, parentheses, hyphen, period (except at end), and Unicode characters that match the allowed characters.
    pub resource_group_name: Option<Input<String>>,
    /// The tags attached to the resource group.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Resource group information.
pub struct ResourceGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The location of the resource group. It cannot be changed after the resource group has been created. It must be one of the supported Azure locations.
    pub location: Output<serde_json::Value>,
    /// The ID of the resource that manages this resource group.
    pub managed_by: Output<serde_json::Value>,
    /// The name of the resource group.
    pub name: Output<serde_json::Value>,
    /// The resource group properties.
    pub properties: Output<serde_json::Value>,
    /// The tags attached to the resource group.
    pub tags: Output<serde_json::Value>,
    /// The type of the resource group.
    pub type_: Output<serde_json::Value>,
}

impl ResourceGroup {
    const TYPE_TOKEN: &'static str = "azure-native:resources/v20250301:ResourceGroup";

    /// Create a new ResourceGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ResourceGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.managed_by {
            pulumi_sdk::resolve_input("managedBy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.resource_group_name {
            pulumi_sdk::resolve_input("resourceGroupName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

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
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            managed_by: registered.outputs.get("managedBy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            properties: registered.outputs.get("properties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
