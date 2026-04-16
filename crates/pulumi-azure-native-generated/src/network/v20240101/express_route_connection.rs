use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// ExpressRouteConnection resource.
pub struct ExpressRouteConnectionArgs {
    /// Authorization key to establish the connection.
    pub authorization_key: Option<Input<String>>,
    /// The name of the connection subresource.
    pub connection_name: Option<Input<String>>,
    /// Enable internet security.
    pub enable_internet_security: Option<Input<bool>>,
    /// Bypass the ExpressRoute gateway when accessing private-links. ExpressRoute FastPath (expressRouteGatewayBypass) must be enabled.
    pub enable_private_link_fast_path: Option<Input<bool>>,
    /// The ExpressRoute circuit peering.
    pub express_route_circuit_peering: Input<network::v20240101::ExpressRouteCircuitPeeringIdArgs>,
    /// Enable FastPath to vWan Firewall hub.
    pub express_route_gateway_bypass: Option<Input<bool>>,
    /// The name of the ExpressRoute gateway.
    pub express_route_gateway_name: Input<String>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource.
    pub name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Option<Input<network::v20240101::RoutingConfigurationArgs>>,
    /// The routing weight associated to the connection.
    pub routing_weight: Option<Input<i64>>,
}

/// ExpressRouteConnection resource.
pub struct ExpressRouteConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Authorization key to establish the connection.
    pub authorization_key: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Enable internet security.
    pub enable_internet_security: Output<serde_json::Value>,
    /// Bypass the ExpressRoute gateway when accessing private-links. ExpressRoute FastPath (expressRouteGatewayBypass) must be enabled.
    pub enable_private_link_fast_path: Output<serde_json::Value>,
    /// The ExpressRoute circuit peering.
    pub express_route_circuit_peering: Output<serde_json::Value>,
    /// Enable FastPath to vWan Firewall hub.
    pub express_route_gateway_bypass: Output<serde_json::Value>,
    /// The name of the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the express route connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Output<serde_json::Value>,
    /// The routing weight associated to the connection.
    pub routing_weight: Output<serde_json::Value>,
}

impl ExpressRouteConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240101:ExpressRouteConnection";

    /// Create a new ExpressRouteConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ExpressRouteConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.authorization_key {
            pulumi_sdk::resolve_input("authorizationKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.connection_name {
            pulumi_sdk::resolve_input("connectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_internet_security {
            pulumi_sdk::resolve_input("enableInternetSecurity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_private_link_fast_path {
            pulumi_sdk::resolve_input("enablePrivateLinkFastPath", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("expressRouteCircuitPeering", args.express_route_circuit_peering, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.express_route_gateway_bypass {
            pulumi_sdk::resolve_input("expressRouteGatewayBypass", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("expressRouteGatewayName", args.express_route_gateway_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("name", args.name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.routing_configuration {
            pulumi_sdk::resolve_input("routingConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.routing_weight {
            pulumi_sdk::resolve_input("routingWeight", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            authorization_key: registered.outputs.get("authorizationKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_internet_security: registered.outputs.get("enableInternetSecurity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_private_link_fast_path: registered.outputs.get("enablePrivateLinkFastPath")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_circuit_peering: registered.outputs.get("expressRouteCircuitPeering")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_gateway_bypass: registered.outputs.get("expressRouteGatewayBypass")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_configuration: registered.outputs.get("routingConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_weight: registered.outputs.get("routingWeight")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
