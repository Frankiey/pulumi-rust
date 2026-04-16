use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified private endpoint by resource group.
#[derive(Default)]
pub struct GetPrivateEndpointArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the private endpoint.
    pub private_endpoint_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetPrivateEndpointResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// An array of custom dns configurations.
    pub custom_dns_configs: Option<Vec<network::v20200401::CustomDnsConfigPropertiesFormatResponse>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// A grouping of information about the connection to the remote resource. Used when the network admin does not have access to approve connections to the remote resource.
    pub manual_private_link_service_connections: Option<Vec<network::v20200401::PrivateLinkServiceConnectionResponse>>,
    /// Resource name.
    pub name: String,
    /// An array of references to the network interfaces created for this private endpoint.
    pub network_interfaces: Vec<network::v20200401::NetworkInterfaceResponse>,
    /// A grouping of information about the connection to the remote resource.
    pub private_link_service_connections: Option<Vec<network::v20200401::PrivateLinkServiceConnectionResponse>>,
    /// The provisioning state of the private endpoint resource.
    pub provisioning_state: String,
    /// The ID of the subnet from which the private IP will be allocated.
    pub subnet: Option<network::v20200401::SubnetResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified private endpoint by resource group.
pub async fn get_private_endpoint(
    ctx: &Context,
    args: GetPrivateEndpointArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPrivateEndpointResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("privateEndpointName".to_string(), json!(args.private_endpoint_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200401:getPrivateEndpoint", invoke_args, &opts).await?;

    Ok(GetPrivateEndpointResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        custom_dns_configs: result.fields.get("customDnsConfigs").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        manual_private_link_service_connections: result.fields.get("manualPrivateLinkServiceConnections").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_interfaces: serde_json::from_value(result.fields.get("networkInterfaces").cloned().unwrap_or_default())?
            ,
        private_link_service_connections: result.fields.get("privateLinkServiceConnections").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        subnet: result.fields.get("subnet").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
