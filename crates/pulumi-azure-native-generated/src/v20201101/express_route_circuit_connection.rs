use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Express Route Circuit Connection in an ExpressRouteCircuitPeering resource.
pub struct ExpressRouteCircuitConnectionArgs {
    /// /29 IP address space to carve out Customer addresses for tunnels.
    pub address_prefix: Option<Input<String>>,
    /// The authorization key.
    pub authorization_key: Option<Input<String>>,
    /// The name of the express route circuit.
    pub circuit_name: Input<String>,
    /// The name of the express route circuit connection.
    pub connection_name: Option<Input<String>>,
    /// Reference to Express Route Circuit Private Peering Resource of the circuit initiating connection.
    pub express_route_circuit_peering: Option<Input<network::v20201101::SubResourceArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// IPv6 Address PrefixProperties of the express route circuit connection.
    pub ipv6circuit_connection_config: Option<Input<network::v20201101::Ipv6CircuitConnectionConfigArgs>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// Reference to Express Route Circuit Private Peering Resource of the peered circuit.
    pub peer_express_route_circuit_peering: Option<Input<network::v20201101::SubResourceArgs>>,
    /// The name of the peering.
    pub peering_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// Express Route Circuit Connection in an ExpressRouteCircuitPeering resource.
pub struct ExpressRouteCircuitConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// /29 IP address space to carve out Customer addresses for tunnels.
    pub address_prefix: Output<serde_json::Value>,
    /// The authorization key.
    pub authorization_key: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Express Route Circuit connection state.
    pub circuit_connection_status: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Reference to Express Route Circuit Private Peering Resource of the circuit initiating connection.
    pub express_route_circuit_peering: Output<serde_json::Value>,
    /// IPv6 Address PrefixProperties of the express route circuit connection.
    pub ipv6circuit_connection_config: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// Reference to Express Route Circuit Private Peering Resource of the peered circuit.
    pub peer_express_route_circuit_peering: Output<serde_json::Value>,
    /// The provisioning state of the express route circuit connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Type of the resource.
    pub type_: Output<serde_json::Value>,
}

impl ExpressRouteCircuitConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20201101:ExpressRouteCircuitConnection";

    /// Create a new ExpressRouteCircuitConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ExpressRouteCircuitConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.address_prefix {
            pulumi_sdk::resolve_input("addressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.authorization_key {
            pulumi_sdk::resolve_input("authorizationKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("circuitName", args.circuit_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.connection_name {
            pulumi_sdk::resolve_input("connectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_circuit_peering {
            pulumi_sdk::resolve_input("expressRouteCircuitPeering", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ipv6circuit_connection_config {
            pulumi_sdk::resolve_input("ipv6CircuitConnectionConfig", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peer_express_route_circuit_peering {
            pulumi_sdk::resolve_input("peerExpressRouteCircuitPeering", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("peeringName", args.peering_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            address_prefix: registered.outputs.get("addressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            authorization_key: registered.outputs.get("authorizationKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            circuit_connection_status: registered.outputs.get("circuitConnectionStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_circuit_peering: registered.outputs.get("expressRouteCircuitPeering")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ipv6circuit_connection_config: registered.outputs.get("ipv6CircuitConnectionConfig")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peer_express_route_circuit_peering: registered.outputs.get("peerExpressRouteCircuitPeering")
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
