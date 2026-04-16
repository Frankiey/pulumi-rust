use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Virtual Router Peering resource.
pub struct VirtualRouterPeeringArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Name of the virtual router peering that is unique within a virtual router.
    pub name: Option<Input<String>>,
    /// Peer ASN.
    pub peer_asn: Option<Input<f64>>,
    /// Peer IP.
    pub peer_ip: Option<Input<String>>,
    /// The name of the Virtual Router Peering.
    pub peering_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the Virtual Router.
    pub virtual_router_name: Input<String>,
}

/// Virtual Router Peering resource.
pub struct VirtualRouterPeering {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Name of the virtual router peering that is unique within a virtual router.
    pub name: Output<serde_json::Value>,
    /// Peer ASN.
    pub peer_asn: Output<serde_json::Value>,
    /// Peer IP.
    pub peer_ip: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Peering type.
    pub type_: Output<serde_json::Value>,
}

impl VirtualRouterPeering {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190901:VirtualRouterPeering";

    /// Create a new VirtualRouterPeering resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualRouterPeeringArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peer_asn {
            pulumi_sdk::resolve_input("peerAsn", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peer_ip {
            pulumi_sdk::resolve_input("peerIp", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peering_name {
            pulumi_sdk::resolve_input("peeringName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("virtualRouterName", args.virtual_router_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peer_asn: registered.outputs.get("peerAsn")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peer_ip: registered.outputs.get("peerIp")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
