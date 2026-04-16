use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified network group.
#[derive(Default)]
pub struct GetNetworkGroupArgs {
    /// The name of the network group.
    pub network_group_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description of the network group.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the scope assignment resource.
    pub provisioning_state: String,
    /// Unique identifier for this resource.
    pub resource_guid: String,
    /// The system metadata related to this resource.
    pub system_data: network::v20230601::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified network group.
pub async fn get_network_group(
    ctx: &Context,
    args: GetNetworkGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkGroupName".to_string(), json!(args.network_group_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230601:getNetworkGroup", invoke_args, &opts).await?;

    Ok(GetNetworkGroupResult {
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
