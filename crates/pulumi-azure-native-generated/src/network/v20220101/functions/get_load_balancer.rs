use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified load balancer.
#[derive(Default)]
pub struct GetLoadBalancerArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the load balancer.
    pub load_balancer_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetLoadBalancerResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Collection of backend address pools used by a load balancer.
    pub backend_address_pools: Option<Vec<network::v20220101::BackendAddressPoolResponse>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The extended location of the load balancer.
    pub extended_location: Option<network::v20220101::ExtendedLocationResponse>,
    /// Object representing the frontend IPs to be used for the load balancer.
    pub frontend_ip_configurations: Option<Vec<network::v20220101::FrontendIPConfigurationResponse>>,
    /// Resource ID.
    pub id: Option<String>,
    /// Defines an port range to be used by inbound NAT Pools. Inbound NAT pools are used to define a range of NAT ports to be used by a VMSS cluster. After the creation of an inbound NAT pool, individual inbound NAT rules are automatically created for every VM in a VMSS cluster.  Defining inbound NAT rules on your load balancer is mutually exclusive with defining an inbound NAT pool. Inbound NAT pools are associated with VMSS, while inbound NAT rules are associated with individual VMs.
    pub inbound_nat_pools: Option<Vec<network::v20220101::InboundNatPoolResponse>>,
    /// collection of inbound NAT Rules used by a load balancer. An inbound NAT rule is used to forward traffic from a load balancer frontend to one or more instances in the backend pool. Defining inbound NAT rules on your load balancer is mutually exclusive with defining an inbound NAT pool. Inbound NAT pools are associated with VMSS, while inbound NAT rules are associated with individual VMs.
    pub inbound_nat_rules: Option<Vec<network::v20220101::InboundNatRuleResponse>>,
    /// Object collection representing the load balancing rules Gets the provisioning.
    pub load_balancing_rules: Option<Vec<network::v20220101::LoadBalancingRuleResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The outbound rules.
    pub outbound_rules: Option<Vec<network::v20220101::OutboundRuleResponse>>,
    /// Collection of probe objects used in the load balancer.
    pub probes: Option<Vec<network::v20220101::ProbeResponse>>,
    /// The provisioning state of the load balancer resource.
    pub provisioning_state: String,
    /// The resource GUID property of the load balancer resource.
    pub resource_guid: String,
    /// The load balancer SKU.
    pub sku: Option<network::v20220101::LoadBalancerSkuResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified load balancer.
pub async fn get_load_balancer(
    ctx: &Context,
    args: GetLoadBalancerArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetLoadBalancerResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("loadBalancerName".to_string(), json!(args.load_balancer_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220101:getLoadBalancer", invoke_args, &opts).await?;

    Ok(GetLoadBalancerResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        backend_address_pools: result.fields.get("backendAddressPools").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        extended_location: result.fields.get("extendedLocation").cloned().map(serde_json::from_value).transpose()?,
        frontend_ip_configurations: result.fields.get("frontendIPConfigurations").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        inbound_nat_pools: result.fields.get("inboundNatPools").cloned().map(serde_json::from_value).transpose()?,
        inbound_nat_rules: result.fields.get("inboundNatRules").cloned().map(serde_json::from_value).transpose()?,
        load_balancing_rules: result.fields.get("loadBalancingRules").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        outbound_rules: result.fields.get("outboundRules").cloned().map(serde_json::from_value).transpose()?,
        probes: result.fields.get("probes").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
