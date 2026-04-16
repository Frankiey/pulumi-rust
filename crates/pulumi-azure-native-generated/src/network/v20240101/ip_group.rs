use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The IpGroups resource information.
pub struct IpGroupArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// IpAddresses/IpAddressPrefixes in the IpGroups resource.
    pub ip_addresses: Option<Vec<Input<String>>>,
    /// The name of the ipGroups.
    pub ip_groups_name: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// The IpGroups resource information.
pub struct IpGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// List of references to Firewall Policies resources that this IpGroups is associated with.
    pub firewall_policies: Output<serde_json::Value>,
    /// List of references to Firewall resources that this IpGroups is associated with.
    pub firewalls: Output<serde_json::Value>,
    /// IpAddresses/IpAddressPrefixes in the IpGroups resource.
    pub ip_addresses: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the IpGroups resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl IpGroup {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240101:IpGroup";

    /// Create a new IpGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: IpGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_addresses {
            pulumi_sdk::resolve_input_vec("ipAddresses", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_groups_name {
            pulumi_sdk::resolve_input("ipGroupsName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
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
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            firewall_policies: registered.outputs.get("firewallPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            firewalls: registered.outputs.get("firewalls")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_addresses: registered.outputs.get("ipAddresses")
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
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
