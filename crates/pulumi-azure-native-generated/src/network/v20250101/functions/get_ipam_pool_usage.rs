use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// IpamPool usage information.
#[derive(Default)]
pub struct GetIpamPoolUsageArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// Pool resource name.
    pub pool_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetIpamPoolUsageResult {
    /// List of IP address prefixes of the resource.
    pub address_prefixes: Vec<String>,
    /// List of assigned IP address prefixes.
    pub allocated_address_prefixes: Vec<String>,
    /// List of available IP address prefixes.
    pub available_address_prefixes: Vec<String>,
    /// List of IpamPool that are children of this IpamPool.
    pub child_pools: Vec<network::v20250101::ResourceBasicsResponse>,
    /// Total number of assigned IP addresses in the IpamPool.
    pub number_of_allocated_ip_addresses: String,
    /// Total number of available IP addresses in the IpamPool.
    pub number_of_available_ip_addresses: String,
    /// Total number of reserved IP addresses in the IpamPool.
    pub number_of_reserved_ip_addresses: String,
    /// List of reserved IP address prefixes. These IP addresses could be reclaimed if not assigned in the given time.
    pub reserved_address_prefixes: Vec<String>,
    /// Total number of IP addresses managed in the IpamPool.
    pub total_number_of_ip_addresses: String,
}

/// IpamPool usage information.
pub async fn get_ipam_pool_usage(
    ctx: &Context,
    args: GetIpamPoolUsageArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetIpamPoolUsageResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("poolName".to_string(), json!(args.pool_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250101:getIpamPoolUsage", invoke_args, &opts).await?;

    Ok(GetIpamPoolUsageResult {
        address_prefixes: serde_json::from_value(result.fields.get("addressPrefixes").cloned().unwrap_or_default())?
            ,
        allocated_address_prefixes: serde_json::from_value(result.fields.get("allocatedAddressPrefixes").cloned().unwrap_or_default())?
            ,
        available_address_prefixes: serde_json::from_value(result.fields.get("availableAddressPrefixes").cloned().unwrap_or_default())?
            ,
        child_pools: serde_json::from_value(result.fields.get("childPools").cloned().unwrap_or_default())?
            ,
        number_of_allocated_ip_addresses: serde_json::from_value(result.fields.get("numberOfAllocatedIPAddresses").cloned().unwrap_or_default())?
            ,
        number_of_available_ip_addresses: serde_json::from_value(result.fields.get("numberOfAvailableIPAddresses").cloned().unwrap_or_default())?
            ,
        number_of_reserved_ip_addresses: serde_json::from_value(result.fields.get("numberOfReservedIPAddresses").cloned().unwrap_or_default())?
            ,
        reserved_address_prefixes: serde_json::from_value(result.fields.get("reservedAddressPrefixes").cloned().unwrap_or_default())?
            ,
        total_number_of_ip_addresses: serde_json::from_value(result.fields.get("totalNumberOfIPAddresses").cloned().unwrap_or_default())?
            ,
    })
}
