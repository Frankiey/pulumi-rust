use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a Network Connectivity Configuration, specified by the resource group, network manager name, and connectivity Configuration name
#[derive(Default)]
pub struct GetConnectivityConfigurationArgs {
    /// The name of the network manager connectivity configuration.
    pub configuration_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetConnectivityConfigurationResult {
    /// Groups for configuration
    pub applies_to_groups: Vec<network::v20220501::ConnectivityGroupItemResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Connectivity topology type.
    pub connectivity_topology: String,
    /// Flag if need to remove current existing peerings.
    pub delete_existing_peering: Option<String>,
    /// A description of the connectivity configuration.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// List of hubItems
    pub hubs: Option<Vec<network::v20220501::HubResponse>>,
    /// Resource ID.
    pub id: String,
    /// Flag if global mesh is supported.
    pub is_global: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the connectivity configuration resource.
    pub provisioning_state: String,
    /// The system metadata related to this resource.
    pub system_data: network::v20220501::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Gets a Network Connectivity Configuration, specified by the resource group, network manager name, and connectivity Configuration name
pub async fn get_connectivity_configuration(
    ctx: &Context,
    args: GetConnectivityConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetConnectivityConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationName".to_string(), json!(args.configuration_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220501:getConnectivityConfiguration", invoke_args, &opts).await?;

    Ok(GetConnectivityConfigurationResult {
        applies_to_groups: serde_json::from_value(result.fields.get("appliesToGroups").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        connectivity_topology: serde_json::from_value(result.fields.get("connectivityTopology").cloned().unwrap_or_default())?
            ,
        delete_existing_peering: result.fields.get("deleteExistingPeering").cloned().map(serde_json::from_value).transpose()?,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        hubs: result.fields.get("hubs").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        is_global: result.fields.get("isGlobal").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
