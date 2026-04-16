use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified IpAllocation by resource group.
#[derive(Default)]
pub struct GetIpAllocationArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the IpAllocation.
    pub ip_allocation_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetIpAllocationResult {
    /// IpAllocation tags.
    pub allocation_tags: Option<HashMap<String, String>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The IPAM allocation ID.
    pub ipam_allocation_id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The address prefix for the IpAllocation.
    pub prefix: Option<String>,
    /// The address prefix length for the IpAllocation.
    pub prefix_length: Option<i64>,
    /// The address prefix Type for the IpAllocation.
    pub prefix_type: Option<String>,
    /// The Subnet that using the prefix of this IpAllocation resource.
    pub subnet: network::v20200601::SubResourceResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The VirtualNetwork that using the prefix of this IpAllocation resource.
    pub virtual_network: network::v20200601::SubResourceResponse,
}

/// Gets the specified IpAllocation by resource group.
pub async fn get_ip_allocation(
    ctx: &Context,
    args: GetIpAllocationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetIpAllocationResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("ipAllocationName".to_string(), json!(args.ip_allocation_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200601:getIpAllocation", invoke_args, &opts).await?;

    Ok(GetIpAllocationResult {
        allocation_tags: result.fields.get("allocationTags").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ipam_allocation_id: result.fields.get("ipamAllocationId").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        prefix: result.fields.get("prefix").cloned().map(serde_json::from_value).transpose()?,
        prefix_length: result.fields.get("prefixLength").cloned().map(serde_json::from_value).transpose()?,
        prefix_type: result.fields.get("prefixType").cloned().map(serde_json::from_value).transpose()?,
        subnet: serde_json::from_value(result.fields.get("subnet").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_network: serde_json::from_value(result.fields.get("virtualNetwork").cloned().unwrap_or_default())?
            ,
    })
}
