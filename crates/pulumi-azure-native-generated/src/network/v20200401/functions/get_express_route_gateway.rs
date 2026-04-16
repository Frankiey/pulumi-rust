use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Fetches the details of a ExpressRoute gateway in a resource group.
#[derive(Default)]
pub struct GetExpressRouteGatewayArgs {
    /// The name of the ExpressRoute gateway.
    pub express_route_gateway_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRouteGatewayResult {
    /// Configuration for auto scaling.
    pub auto_scale_configuration: Option<network::v20200401::ExpressRouteGatewayPropertiesResponseAutoScaleConfiguration>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// List of ExpressRoute connections to the ExpressRoute gateway.
    pub express_route_connections: Vec<network::v20200401::ExpressRouteConnectionResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the express route gateway resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The Virtual Hub where the ExpressRoute gateway is or will be deployed.
    pub virtual_hub: network::v20200401::VirtualHubIdResponse,
}

/// Fetches the details of a ExpressRoute gateway in a resource group.
pub async fn get_express_route_gateway(
    ctx: &Context,
    args: GetExpressRouteGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRouteGatewayResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("expressRouteGatewayName".to_string(), json!(args.express_route_gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200401:getExpressRouteGateway", invoke_args, &opts).await?;

    Ok(GetExpressRouteGatewayResult {
        auto_scale_configuration: result.fields.get("autoScaleConfiguration").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        express_route_connections: serde_json::from_value(result.fields.get("expressRouteConnections").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hub: serde_json::from_value(result.fields.get("virtualHub").cloned().unwrap_or_default())?
            ,
    })
}
