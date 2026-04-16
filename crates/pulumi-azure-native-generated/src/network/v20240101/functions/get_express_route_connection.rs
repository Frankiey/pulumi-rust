use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified ExpressRouteConnection.
#[derive(Default)]
pub struct GetExpressRouteConnectionArgs {
    /// The name of the ExpressRoute connection.
    pub connection_name: String,
    /// The name of the ExpressRoute gateway.
    pub express_route_gateway_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRouteConnectionResult {
    /// Authorization key to establish the connection.
    pub authorization_key: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Enable internet security.
    pub enable_internet_security: Option<bool>,
    /// Bypass the ExpressRoute gateway when accessing private-links. ExpressRoute FastPath (expressRouteGatewayBypass) must be enabled.
    pub enable_private_link_fast_path: Option<bool>,
    /// The ExpressRoute circuit peering.
    pub express_route_circuit_peering: network::v20240101::ExpressRouteCircuitPeeringIdResponse,
    /// Enable FastPath to vWan Firewall hub.
    pub express_route_gateway_bypass: Option<bool>,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource.
    pub name: String,
    /// The provisioning state of the express route connection resource.
    pub provisioning_state: String,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Option<network::v20240101::RoutingConfigurationResponse>,
    /// The routing weight associated to the connection.
    pub routing_weight: Option<i64>,
}

/// Gets the specified ExpressRouteConnection.
pub async fn get_express_route_connection(
    ctx: &Context,
    args: GetExpressRouteConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRouteConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("expressRouteGatewayName".to_string(), json!(args.express_route_gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240101:getExpressRouteConnection", invoke_args, &opts).await?;

    Ok(GetExpressRouteConnectionResult {
        authorization_key: result.fields.get("authorizationKey").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        enable_internet_security: result.fields.get("enableInternetSecurity").cloned().map(serde_json::from_value).transpose()?,
        enable_private_link_fast_path: result.fields.get("enablePrivateLinkFastPath").cloned().map(serde_json::from_value).transpose()?,
        express_route_circuit_peering: serde_json::from_value(result.fields.get("expressRouteCircuitPeering").cloned().unwrap_or_default())?
            ,
        express_route_gateway_bypass: result.fields.get("expressRouteGatewayBypass").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        routing_configuration: result.fields.get("routingConfiguration").cloned().map(serde_json::from_value).transpose()?,
        routing_weight: result.fields.get("routingWeight").cloned().map(serde_json::from_value).transpose()?,
    })
}
