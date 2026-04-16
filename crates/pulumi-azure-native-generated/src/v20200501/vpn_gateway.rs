use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VpnGateway Resource.
pub struct VpnGatewayArgs {
    /// Local network gateway's BGP speaker settings.
    pub bgp_settings: Option<Input<network::v20200501::BgpSettingsArgs>>,
    /// List of all vpn connections to the gateway.
    pub connections: Option<Vec<Input<network::v20200501::VpnConnectionArgs>>>,
    /// The name of the gateway.
    pub gateway_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The resource group name of the VpnGateway.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The VirtualHub to which the gateway belongs.
    pub virtual_hub: Option<Input<network::v20200501::SubResourceArgs>>,
    /// The scale unit for this vpn gateway.
    pub vpn_gateway_scale_unit: Option<Input<i64>>,
}

/// VpnGateway Resource.
pub struct VpnGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Local network gateway's BGP speaker settings.
    pub bgp_settings: Output<serde_json::Value>,
    /// List of all vpn connections to the gateway.
    pub connections: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the VPN gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The VirtualHub to which the gateway belongs.
    pub virtual_hub: Output<serde_json::Value>,
    /// The scale unit for this vpn gateway.
    pub vpn_gateway_scale_unit: Output<serde_json::Value>,
}

impl VpnGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20200501:VpnGateway";

    /// Create a new VpnGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VpnGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.bgp_settings {
            pulumi_sdk::resolve_input("bgpSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.connections {
            pulumi_sdk::resolve_input_vec("connections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_name {
            pulumi_sdk::resolve_input("gatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.virtual_hub {
            pulumi_sdk::resolve_input("virtualHub", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_gateway_scale_unit {
            pulumi_sdk::resolve_input("vpnGatewayScaleUnit", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            connections: registered.outputs.get("connections")
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
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_hub: registered.outputs.get("virtualHub")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_gateway_scale_unit: registered.outputs.get("vpnGatewayScaleUnit")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
