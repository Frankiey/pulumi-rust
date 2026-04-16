use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets information about the specified virtual network appliance.
/// 
/// Uses Azure REST API version 2025-05-01.
#[derive(Default)]
pub struct GetVirtualNetworkApplianceArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network appliance.
    pub virtual_network_appliance_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkApplianceResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Bandwidth of the VirtualNetworkAppliance resource in Gbps.
    pub bandwidth_in_gbps: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// A list of IPConfigurations of the virtual network appliance.
    pub ip_configurations: Vec<network::VirtualNetworkApplianceIpConfigurationResponse>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the virtual network appliance resource.
    pub provisioning_state: String,
    /// The resource GUID property of the virtual network appliance resource.
    pub resource_guid: String,
    /// The reference to the subnet resource.
    pub subnet: Option<network::SubnetResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets information about the specified virtual network appliance.
pub async fn get_virtual_network_appliance(
    ctx: &Context,
    args: GetVirtualNetworkApplianceArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkApplianceResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkApplianceName".to_string(), json!(args.virtual_network_appliance_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:getVirtualNetworkAppliance", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkApplianceResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bandwidth_in_gbps: result.fields.get("bandwidthInGbps").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: serde_json::from_value(result.fields.get("ipConfigurations").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        subnet: result.fields.get("subnet").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
