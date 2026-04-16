use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a RouteTable.
#[derive(Default)]
pub struct GetHubRouteTableArgs {
    /// The resource group name of the VirtualHub.
    pub resource_group_name: String,
    /// The name of the RouteTable.
    pub route_table_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetHubRouteTableResult {
    /// List of all connections associated with this route table.
    pub associated_connections: Vec<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// List of labels associated with this route table.
    pub labels: Option<Vec<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// List of all connections that advertise to this route table.
    pub propagating_connections: Vec<String>,
    /// The provisioning state of the RouteTable resource.
    pub provisioning_state: String,
    /// List of all routes.
    pub routes: Option<Vec<network::v20220501::HubRouteResponse>>,
    /// Resource type.
    pub type_: String,
}

/// Retrieves the details of a RouteTable.
pub async fn get_hub_route_table(
    ctx: &Context,
    args: GetHubRouteTableArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetHubRouteTableResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routeTableName".to_string(), json!(args.route_table_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220501:getHubRouteTable", invoke_args, &opts).await?;

    Ok(GetHubRouteTableResult {
        associated_connections: serde_json::from_value(result.fields.get("associatedConnections").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        labels: result.fields.get("labels").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        propagating_connections: serde_json::from_value(result.fields.get("propagatingConnections").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        routes: result.fields.get("routes").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
