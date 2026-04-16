use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves a network manager routing configuration.
#[derive(Default)]
pub struct GetNetworkManagerRoutingConfigurationArgs {
    /// The name of the network manager Routing Configuration.
    pub configuration_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkManagerRoutingConfigurationResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description of the routing configuration.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Unique identifier for this resource.
    pub resource_guid: String,
    /// The system metadata related to this resource.
    pub system_data: network::v20240301::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Retrieves a network manager routing configuration.
pub async fn get_network_manager_routing_configuration(
    ctx: &Context,
    args: GetNetworkManagerRoutingConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkManagerRoutingConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationName".to_string(), json!(args.configuration_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240301:getNetworkManagerRoutingConfiguration", invoke_args, &opts).await?;

    Ok(GetNetworkManagerRoutingConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
