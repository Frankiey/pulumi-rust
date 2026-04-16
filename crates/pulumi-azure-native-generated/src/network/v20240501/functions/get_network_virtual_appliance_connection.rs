use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of specified NVA connection.
#[derive(Default)]
pub struct GetNetworkVirtualApplianceConnectionArgs {
    /// The name of the NVA connection.
    pub connection_name: String,
    /// The name of the Network Virtual Appliance.
    pub network_virtual_appliance_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkVirtualApplianceConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource.
    pub name: Option<String>,
    /// Properties of the express route connection.
    pub properties: network::v20240501::NetworkVirtualApplianceConnectionPropertiesResponse,
}

/// Retrieves the details of specified NVA connection.
pub async fn get_network_virtual_appliance_connection(
    ctx: &Context,
    args: GetNetworkVirtualApplianceConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkVirtualApplianceConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("networkVirtualApplianceName".to_string(), json!(args.network_virtual_appliance_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240501:getNetworkVirtualApplianceConnection", invoke_args, &opts).await?;

    Ok(GetNetworkVirtualApplianceConnectionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
    })
}
