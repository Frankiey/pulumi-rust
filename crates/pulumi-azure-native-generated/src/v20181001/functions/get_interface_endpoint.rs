use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified interface endpoint by resource group.
#[derive(Default)]
pub struct GetInterfaceEndpointArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the interface endpoint.
    pub interface_endpoint_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetInterfaceEndpointResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A reference to the service being brought into the virtual network.
    pub endpoint_service: Option<network::v20181001::EndpointServiceResponse>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// A first-party service's FQDN that is mapped to the private IP allocated via this interface endpoint.
    pub fqdn: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Gets an array of references to the network interfaces created for this interface endpoint.
    pub network_interfaces: Vec<network::v20181001::NetworkInterfaceResponse>,
    /// A read-only property that identifies who created this interface endpoint.
    pub owner: String,
    /// The provisioning state of the interface endpoint. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: String,
    /// The ID of the subnet from which the private IP will be allocated.
    pub subnet: Option<network::v20181001::SubnetResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified interface endpoint by resource group.
pub async fn get_interface_endpoint(
    ctx: &Context,
    args: GetInterfaceEndpointArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetInterfaceEndpointResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("interfaceEndpointName".to_string(), json!(args.interface_endpoint_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20181001:getInterfaceEndpoint", invoke_args, &opts).await?;

    Ok(GetInterfaceEndpointResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        endpoint_service: result.fields.get("endpointService").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        fqdn: result.fields.get("fqdn").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_interfaces: serde_json::from_value(result.fields.get("networkInterfaces").cloned().unwrap_or_default())?
            ,
        owner: serde_json::from_value(result.fields.get("owner").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        subnet: result.fields.get("subnet").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
