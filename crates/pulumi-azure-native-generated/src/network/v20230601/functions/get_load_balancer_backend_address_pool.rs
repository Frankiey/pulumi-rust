use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets load balancer backend address pool.
#[derive(Default)]
pub struct GetLoadBalancerBackendAddressPoolArgs {
    /// The name of the backend address pool.
    pub backend_address_pool_name: String,
    /// The name of the load balancer.
    pub load_balancer_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetLoadBalancerBackendAddressPoolResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// An array of references to IP addresses defined in network interfaces.
    pub backend_ip_configurations: Vec<network::v20230601::NetworkInterfaceIPConfigurationResponse>,
    /// Amount of seconds Load Balancer waits for before sending RESET to client and backend address.
    pub drain_period_in_seconds: Option<i64>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// An array of references to inbound NAT rules that use this backend address pool.
    pub inbound_nat_rules: Vec<network::v20230601::SubResourceResponse>,
    /// An array of backend addresses.
    pub load_balancer_backend_addresses: Option<Vec<network::v20230601::LoadBalancerBackendAddressResponse>>,
    /// An array of references to load balancing rules that use this backend address pool.
    pub load_balancing_rules: Vec<network::v20230601::SubResourceResponse>,
    /// The location of the backend address pool.
    pub location: Option<String>,
    /// The name of the resource that is unique within the set of backend address pools used by the load balancer. This name can be used to access the resource.
    pub name: Option<String>,
    /// A reference to an outbound rule that uses this backend address pool.
    pub outbound_rule: network::v20230601::SubResourceResponse,
    /// An array of references to outbound rules that use this backend address pool.
    pub outbound_rules: Vec<network::v20230601::SubResourceResponse>,
    /// The provisioning state of the backend address pool resource.
    pub provisioning_state: String,
    /// Backend address synchronous mode for the backend pool
    pub sync_mode: Option<String>,
    /// An array of gateway load balancer tunnel interfaces.
    pub tunnel_interfaces: Option<Vec<network::v20230601::GatewayLoadBalancerTunnelInterfaceResponse>>,
    /// Type of the resource.
    pub type_: String,
    /// A reference to a virtual network.
    pub virtual_network: Option<network::v20230601::SubResourceResponse>,
}

/// Gets load balancer backend address pool.
pub async fn get_load_balancer_backend_address_pool(
    ctx: &Context,
    args: GetLoadBalancerBackendAddressPoolArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetLoadBalancerBackendAddressPoolResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("backendAddressPoolName".to_string(), json!(args.backend_address_pool_name));
    invoke_args.insert("loadBalancerName".to_string(), json!(args.load_balancer_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230601:getLoadBalancerBackendAddressPool", invoke_args, &opts).await?;

    Ok(GetLoadBalancerBackendAddressPoolResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        backend_ip_configurations: serde_json::from_value(result.fields.get("backendIPConfigurations").cloned().unwrap_or_default())?
            ,
        drain_period_in_seconds: result.fields.get("drainPeriodInSeconds").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        inbound_nat_rules: serde_json::from_value(result.fields.get("inboundNatRules").cloned().unwrap_or_default())?
            ,
        load_balancer_backend_addresses: result.fields.get("loadBalancerBackendAddresses").cloned().map(serde_json::from_value).transpose()?,
        load_balancing_rules: serde_json::from_value(result.fields.get("loadBalancingRules").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        outbound_rule: serde_json::from_value(result.fields.get("outboundRule").cloned().unwrap_or_default())?
            ,
        outbound_rules: serde_json::from_value(result.fields.get("outboundRules").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        sync_mode: result.fields.get("syncMode").cloned().map(serde_json::from_value).transpose()?,
        tunnel_interfaces: result.fields.get("tunnelInterfaces").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_network: result.fields.get("virtualNetwork").cloned().map(serde_json::from_value).transpose()?,
    })
}
