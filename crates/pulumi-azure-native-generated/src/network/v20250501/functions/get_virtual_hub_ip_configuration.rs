use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a Virtual Hub Ip configuration.
#[derive(Default)]
pub struct GetVirtualHubIpConfigurationArgs {
    /// The name of the ipconfig.
    pub ip_config_name: String,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualHubIpConfigurationResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Name of the Ip Configuration.
    pub name: Option<String>,
    /// The private IP address of the IP configuration.
    pub private_ip_address: Option<String>,
    /// The private IP address allocation method.
    pub private_ip_allocation_method: Option<String>,
    /// The provisioning state of the IP configuration resource.
    pub provisioning_state: String,
    /// The reference to the public IP resource.
    pub public_ip_address: Option<network::v20250501::PublicIPAddressResponse>,
    /// The reference to the subnet resource.
    pub subnet: Option<network::v20250501::SubnetResponse>,
    /// Ipconfiguration type.
    pub type_: String,
}

/// Retrieves the details of a Virtual Hub Ip configuration.
pub async fn get_virtual_hub_ip_configuration(
    ctx: &Context,
    args: GetVirtualHubIpConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualHubIpConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("ipConfigName".to_string(), json!(args.ip_config_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250501:getVirtualHubIpConfiguration", invoke_args, &opts).await?;

    Ok(GetVirtualHubIpConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        private_ip_address: result.fields.get("privateIPAddress").cloned().map(serde_json::from_value).transpose()?,
        private_ip_allocation_method: result.fields.get("privateIPAllocationMethod").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        public_ip_address: result.fields.get("publicIPAddress").cloned().map(serde_json::from_value).transpose()?,
        subnet: result.fields.get("subnet").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
