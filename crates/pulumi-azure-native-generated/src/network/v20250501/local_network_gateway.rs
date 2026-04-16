use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// A common class for general resource information.
pub struct LocalNetworkGatewayArgs {
    /// Local network gateway's BGP speaker settings.
    pub bgp_settings: Option<Input<network::v20250501::BgpSettingsArgs>>,
    /// FQDN of local network gateway.
    pub fqdn: Option<Input<String>>,
    /// IP address of local network gateway.
    pub gateway_ip_address: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Local network site address space.
    pub local_network_address_space: Option<Input<network::v20250501::AddressSpaceArgs>>,
    /// The name of the local network gateway.
    pub local_network_gateway_name: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// A common class for general resource information.
pub struct LocalNetworkGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Local network gateway's BGP speaker settings.
    pub bgp_settings: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// FQDN of local network gateway.
    pub fqdn: Output<serde_json::Value>,
    /// IP address of local network gateway.
    pub gateway_ip_address: Output<serde_json::Value>,
    /// Local network site address space.
    pub local_network_address_space: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the local network gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the local network gateway resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl LocalNetworkGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250501:LocalNetworkGateway";

    /// Create a new LocalNetworkGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: LocalNetworkGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.bgp_settings {
            pulumi_sdk::resolve_input("bgpSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.fqdn {
            pulumi_sdk::resolve_input("fqdn", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_ip_address {
            pulumi_sdk::resolve_input("gatewayIpAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.local_network_address_space {
            pulumi_sdk::resolve_input("localNetworkAddressSpace", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.local_network_gateway_name {
            pulumi_sdk::resolve_input("localNetworkGatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            bgp_settings: registered.outputs.get("bgpSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            fqdn: registered.outputs.get("fqdn")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            gateway_ip_address: registered.outputs.get("gatewayIpAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            local_network_address_space: registered.outputs.get("localNetworkAddressSpace")
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
            resource_guid: registered.outputs.get("resourceGuid")
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
