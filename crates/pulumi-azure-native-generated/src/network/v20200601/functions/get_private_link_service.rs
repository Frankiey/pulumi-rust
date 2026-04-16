use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified private link service by resource group.
#[derive(Default)]
pub struct GetPrivateLinkServiceArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the private link service.
    pub service_name: String,
}

/// Result of the function invocation.
pub struct GetPrivateLinkServiceResult {
    /// The alias of the private link service.
    pub alias: String,
    /// The auto-approval list of the private link service.
    pub auto_approval: Option<network::v20200601::PrivateLinkServicePropertiesResponseAutoApproval>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Whether the private link service is enabled for proxy protocol or not.
    pub enable_proxy_protocol: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The list of Fqdn.
    pub fqdns: Option<Vec<String>>,
    /// Resource ID.
    pub id: Option<String>,
    /// An array of private link service IP configurations.
    pub ip_configurations: Option<Vec<network::v20200601::PrivateLinkServiceIpConfigurationResponse>>,
    /// An array of references to the load balancer IP configurations.
    pub load_balancer_frontend_ip_configurations: Option<Vec<network::v20200601::FrontendIPConfigurationResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// An array of references to the network interfaces created for this private link service.
    pub network_interfaces: Vec<network::v20200601::NetworkInterfaceResponse>,
    /// An array of list about connections to the private endpoint.
    pub private_endpoint_connections: Vec<network::v20200601::PrivateEndpointConnectionResponse>,
    /// The provisioning state of the private link service resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The visibility list of the private link service.
    pub visibility: Option<network::v20200601::PrivateLinkServicePropertiesResponseVisibility>,
}

/// Gets the specified private link service by resource group.
pub async fn get_private_link_service(
    ctx: &Context,
    args: GetPrivateLinkServiceArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPrivateLinkServiceResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("serviceName".to_string(), json!(args.service_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200601:getPrivateLinkService", invoke_args, &opts).await?;

    Ok(GetPrivateLinkServiceResult {
        alias: serde_json::from_value(result.fields.get("alias").cloned().unwrap_or_default())?
            ,
        auto_approval: result.fields.get("autoApproval").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        enable_proxy_protocol: result.fields.get("enableProxyProtocol").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        fqdns: result.fields.get("fqdns").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        load_balancer_frontend_ip_configurations: result.fields.get("loadBalancerFrontendIpConfigurations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_interfaces: serde_json::from_value(result.fields.get("networkInterfaces").cloned().unwrap_or_default())?
            ,
        private_endpoint_connections: serde_json::from_value(result.fields.get("privateEndpointConnections").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        visibility: result.fields.get("visibility").cloned().map(serde_json::from_value).transpose()?,
    })
}
