use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a VirtualHubRouteTableV2.
#[derive(Default)]
pub struct GetVirtualHubRouteTableV2Args {
    /// The resource group name of the VirtualHubRouteTableV2.
    pub resource_group_name: String,
    /// The name of the VirtualHubRouteTableV2.
    pub route_table_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualHubRouteTableV2Result {
    /// List of all connections attached to this route table v2.
    pub attached_connections: Option<Vec<String>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the virtual hub route table v2 resource.
    pub provisioning_state: String,
    /// List of all routes.
    pub routes: Option<Vec<network::v20250301::VirtualHubRouteV2Response>>,
}

/// Retrieves the details of a VirtualHubRouteTableV2.
pub async fn get_virtual_hub_route_table_v2(
    ctx: &Context,
    args: GetVirtualHubRouteTableV2Args,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualHubRouteTableV2Result> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routeTableName".to_string(), json!(args.route_table_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getVirtualHubRouteTableV2", invoke_args, &opts).await?;

    Ok(GetVirtualHubRouteTableV2Result {
        attached_connections: result.fields.get("attachedConnections").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        routes: result.fields.get("routes").cloned().map(serde_json::from_value).transpose()?,
    })
}
