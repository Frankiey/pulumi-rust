use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified network profile in a specified resource group.
#[derive(Default)]
pub struct GetNetworkProfileArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the public IP prefix.
    pub network_profile_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkProfileResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// List of chid container network interface configurations.
    pub container_network_interface_configurations: Option<Vec<network::v20240301::ContainerNetworkInterfaceConfigurationResponse>>,
    /// List of child container network interfaces.
    pub container_network_interfaces: Vec<network::v20240301::ContainerNetworkInterfaceResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the network profile resource.
    pub provisioning_state: String,
    /// The resource GUID property of the network profile resource.
    pub resource_guid: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified network profile in a specified resource group.
pub async fn get_network_profile(
    ctx: &Context,
    args: GetNetworkProfileArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkProfileResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("networkProfileName".to_string(), json!(args.network_profile_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240301:getNetworkProfile", invoke_args, &opts).await?;

    Ok(GetNetworkProfileResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        container_network_interface_configurations: result.fields.get("containerNetworkInterfaceConfigurations").cloned().map(serde_json::from_value).transpose()?,
        container_network_interfaces: serde_json::from_value(result.fields.get("containerNetworkInterfaces").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
