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
    /// Collection of backend address pools used by a load balancer
    pub backend_address_pools: Option<Vec<network::v20180801::BackendAddressPoolResponse>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Object representing the frontend IPs to be used for the load balancer
    pub frontend_ip_configurations: Option<Vec<network::v20180801::FrontendIPConfigurationResponse>>,
    /// Resource ID.
    pub id: Option<String>,
    /// Defines an external port range for inbound NAT to a single backend port on NICs associated with a load balancer. Inbound NAT rules are created automatically for each NIC associated with the Load Balancer using an external port from this range. Defining an Inbound NAT pool on your Load Balancer is mutually exclusive with defining inbound Nat rules. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an inbound NAT pool. They have to reference individual inbound NAT rules.
    pub inbound_nat_pools: Option<Vec<network::v20180801::InboundNatPoolResponse>>,
    /// Collection of inbound NAT Rules used by a load balancer. Defining inbound NAT rules on your load balancer is mutually exclusive with defining an inbound NAT pool. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an Inbound NAT pool. They have to reference individual inbound NAT rules.
    pub inbound_nat_rules: Option<Vec<network::v20180801::InboundNatRuleResponse>>,
    /// Object collection representing the load balancing rules Gets the provisioning 
    pub load_balancing_rules: Option<Vec<network::v20180801::LoadBalancingRuleResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The outbound rules.
    pub outbound_rules: Option<Vec<network::v20180801::OutboundRuleResponse>>,
    /// Collection of probe objects used in the load balancer
    pub probes: Option<Vec<network::v20180801::ProbeResponse>>,
    /// Gets the provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// The resource GUID property of the load balancer resource.
    pub resource_guid: Option<String>,
    /// The load balancer SKU.
    pub sku: Option<network::v20180801::LoadBalancerSkuResponse>,
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
    let result = ctx.invoke("azure-native:network/v20180801:getLoadBalancer", invoke_args, &opts).await?;

    Ok(GetLoadBalancerResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        backend_address_pools: result.fields.get("backendAddressPools").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
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
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: result.fields.get("resourceGuid").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
