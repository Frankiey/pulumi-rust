use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VirtualWAN Resource.
pub struct VirtualWanArgs {
    /// True if branch to branch traffic is allowed.
    pub allow_branch_to_branch_traffic: Option<Input<bool>>,
    /// True if Vnet to Vnet traffic is allowed.
    pub allow_vnet_to_vnet_traffic: Option<Input<bool>>,
    /// Vpn encryption to be disabled or not.
    pub disable_vpn_encryption: Option<Input<bool>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The resource group name of the VirtualWan.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The type of the VirtualWAN.
    pub type_: Option<Input<String>>,
    /// The name of the VirtualWAN being created or updated.
    pub virtual_wan_name: Option<Input<String>>,
}

/// VirtualWAN Resource.
pub struct VirtualWan {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// True if branch to branch traffic is allowed.
    pub allow_branch_to_branch_traffic: Output<serde_json::Value>,
    /// True if Vnet to Vnet traffic is allowed.
    pub allow_vnet_to_vnet_traffic: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Vpn encryption to be disabled or not.
    pub disable_vpn_encryption: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The office local breakout category.
    pub office365local_breakout_category: Output<serde_json::Value>,
    /// The provisioning state of the virtual WAN resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// List of VirtualHubs in the VirtualWAN.
    pub virtual_hubs: Output<serde_json::Value>,
    /// List of VpnSites in the VirtualWAN.
    pub vpn_sites: Output<serde_json::Value>,
}

impl VirtualWan {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20191201:VirtualWan";

    /// Create a new VirtualWan resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualWanArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.allow_branch_to_branch_traffic {
            pulumi_sdk::resolve_input("allowBranchToBranchTraffic", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_vnet_to_vnet_traffic {
            pulumi_sdk::resolve_input("allowVnetToVnetTraffic", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.disable_vpn_encryption {
            pulumi_sdk::resolve_input("disableVpnEncryption", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.type_ {
            pulumi_sdk::resolve_input("type", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_wan_name {
            pulumi_sdk::resolve_input("virtualWANName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            allow_branch_to_branch_traffic: registered.outputs.get("allowBranchToBranchTraffic")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_vnet_to_vnet_traffic: registered.outputs.get("allowVnetToVnetTraffic")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            disable_vpn_encryption: registered.outputs.get("disableVpnEncryption")
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
            office365local_breakout_category: registered.outputs.get("office365LocalBreakoutCategory")
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
            virtual_hubs: registered.outputs.get("virtualHubs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_sites: registered.outputs.get("vpnSites")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
