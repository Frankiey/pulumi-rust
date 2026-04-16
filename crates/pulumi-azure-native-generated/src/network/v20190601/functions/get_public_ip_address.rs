use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified public IP address in a specified resource group.
#[derive(Default)]
pub struct GetPublicIPAddressArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the subnet.
    pub public_ip_address_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetPublicIPAddressResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The DDoS protection custom policy associated with the public IP address.
    pub ddos_settings: Option<network::v20190601::DdosSettingsResponse>,
    /// The FQDN of the DNS record associated with the public IP address.
    pub dns_settings: Option<network::v20190601::PublicIPAddressDnsSettingsResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// The idle timeout of the public IP address.
    pub idle_timeout_in_minutes: Option<i64>,
    /// The IP address associated with the public IP address resource.
    pub ip_address: Option<String>,
    /// The IP configuration associated with the public IP address.
    pub ip_configuration: network::v20190601::IPConfigurationResponse,
    /// The list of tags associated with the public IP address.
    pub ip_tags: Option<Vec<network::v20190601::IpTagResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// The public IP address version.
    pub public_ip_address_version: Option<String>,
    /// The public IP address allocation method.
    pub public_ip_allocation_method: Option<String>,
    /// The Public IP Prefix this Public IP Address should be allocated from.
    pub public_ip_prefix: Option<network::v20190601::SubResourceResponse>,
    /// The resource GUID property of the public IP resource.
    pub resource_guid: Option<String>,
    /// The public IP address SKU.
    pub sku: Option<network::v20190601::PublicIPAddressSkuResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Option<Vec<String>>,
}

/// Gets the specified public IP address in a specified resource group.
pub async fn get_public_ip_address(
    ctx: &Context,
    args: GetPublicIPAddressArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPublicIPAddressResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("publicIpAddressName".to_string(), json!(args.public_ip_address_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20190601:getPublicIPAddress", invoke_args, &opts).await?;

    Ok(GetPublicIPAddressResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        ddos_settings: result.fields.get("ddosSettings").cloned().map(serde_json::from_value).transpose()?,
        dns_settings: result.fields.get("dnsSettings").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        idle_timeout_in_minutes: result.fields.get("idleTimeoutInMinutes").cloned().map(serde_json::from_value).transpose()?,
        ip_address: result.fields.get("ipAddress").cloned().map(serde_json::from_value).transpose()?,
        ip_configuration: serde_json::from_value(result.fields.get("ipConfiguration").cloned().unwrap_or_default())?
            ,
        ip_tags: result.fields.get("ipTags").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        public_ip_address_version: result.fields.get("publicIPAddressVersion").cloned().map(serde_json::from_value).transpose()?,
        public_ip_allocation_method: result.fields.get("publicIPAllocationMethod").cloned().map(serde_json::from_value).transpose()?,
        public_ip_prefix: result.fields.get("publicIPPrefix").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: result.fields.get("resourceGuid").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
