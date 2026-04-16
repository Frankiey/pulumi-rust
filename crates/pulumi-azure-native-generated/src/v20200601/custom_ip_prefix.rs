use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Custom IP prefix resource.
pub struct CustomIPPrefixArgs {
    /// The prefix range in CIDR notation. Should include the start address and the prefix length.
    pub cidr: Option<Input<String>>,
    /// The commissioned state of the Custom IP Prefix.
    pub commissioned_state: Option<Input<serde_json::Value>>,
    /// The name of the custom IP prefix.
    pub custom_ip_prefix_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Option<Vec<Input<String>>>,
}

/// Custom IP prefix resource.
pub struct CustomIPPrefix {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The prefix range in CIDR notation. Should include the start address and the prefix length.
    pub cidr: Output<serde_json::Value>,
    /// The commissioned state of the Custom IP Prefix.
    pub commissioned_state: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the custom IP prefix resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The list of all referenced PublicIpPrefixes.
    pub public_ip_prefixes: Output<serde_json::Value>,
    /// The resource GUID property of the custom IP prefix resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Output<serde_json::Value>,
}

impl CustomIPPrefix {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20200601:CustomIPPrefix";

    /// Create a new CustomIPPrefix resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: CustomIPPrefixArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.cidr {
            pulumi_sdk::resolve_input("cidr", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.commissioned_state {
            pulumi_sdk::resolve_input("commissionedState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.custom_ip_prefix_name {
            pulumi_sdk::resolve_input("customIpPrefixName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.zones {
            pulumi_sdk::resolve_input_vec("zones", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            cidr: registered.outputs.get("cidr")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            commissioned_state: registered.outputs.get("commissionedState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_prefixes: registered.outputs.get("publicIpPrefixes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
