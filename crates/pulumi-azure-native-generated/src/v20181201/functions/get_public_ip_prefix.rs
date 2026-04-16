use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified public IP prefix in a specified resource group.
#[derive(Default)]
pub struct GetPublicIPPrefixArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the public IP prefix.
    pub public_ip_prefix_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetPublicIPPrefixResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// The allocated Prefix
    pub ip_prefix: Option<String>,
    /// The list of tags associated with the public IP prefix.
    pub ip_tags: Option<Vec<network::v20181201::IpTagResponse>>,
    /// The reference to load balancer frontend IP configuration associated with the public IP prefix.
    pub load_balancer_frontend_ip_configuration: network::v20181201::SubResourceResponse,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The Length of the Public IP Prefix.
    pub prefix_length: Option<i64>,
    /// The provisioning state of the Public IP prefix resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// The public IP address version. Possible values are: 'IPv4' and 'IPv6'.
    pub public_ip_address_version: Option<String>,
    /// The list of all referenced PublicIPAddresses
    pub public_ip_addresses: Option<Vec<network::v20181201::ReferencedPublicIpAddressResponse>>,
    /// The resource GUID property of the public IP prefix resource.
    pub resource_guid: Option<String>,
    /// The public IP prefix SKU.
    pub sku: Option<network::v20181201::PublicIPPrefixSkuResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Option<Vec<String>>,
}

/// Gets the specified public IP prefix in a specified resource group.
pub async fn get_public_ip_prefix(
    ctx: &Context,
    args: GetPublicIPPrefixArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPublicIPPrefixResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("publicIpPrefixName".to_string(), json!(args.public_ip_prefix_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20181201:getPublicIPPrefix", invoke_args, &opts).await?;

    Ok(GetPublicIPPrefixResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_prefix: result.fields.get("ipPrefix").cloned().map(serde_json::from_value).transpose()?,
        ip_tags: result.fields.get("ipTags").cloned().map(serde_json::from_value).transpose()?,
        load_balancer_frontend_ip_configuration: serde_json::from_value(result.fields.get("loadBalancerFrontendIpConfiguration").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        prefix_length: result.fields.get("prefixLength").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        public_ip_address_version: result.fields.get("publicIPAddressVersion").cloned().map(serde_json::from_value).transpose()?,
        public_ip_addresses: result.fields.get("publicIPAddresses").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: result.fields.get("resourceGuid").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
