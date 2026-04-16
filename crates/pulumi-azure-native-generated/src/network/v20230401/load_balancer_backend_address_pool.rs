use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Pool of backend IP addresses.
pub struct LoadBalancerBackendAddressPoolArgs {
    /// The name of the backend address pool.
    pub backend_address_pool_name: Option<Input<String>>,
    /// Amount of seconds Load Balancer waits for before sending RESET to client and backend address.
    pub drain_period_in_seconds: Option<Input<i64>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// An array of backend addresses.
    pub load_balancer_backend_addresses: Option<Vec<Input<network::v20230401::LoadBalancerBackendAddressArgs>>>,
    /// The name of the load balancer.
    pub load_balancer_name: Input<String>,
    /// The location of the backend address pool.
    pub location: Option<Input<String>>,
    /// The name of the resource that is unique within the set of backend address pools used by the load balancer. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Backend address synchronous mode for the backend pool
    pub sync_mode: Option<Input<serde_json::Value>>,
    /// An array of gateway load balancer tunnel interfaces.
    pub tunnel_interfaces: Option<Vec<Input<network::v20230401::GatewayLoadBalancerTunnelInterfaceArgs>>>,
    /// A reference to a virtual network.
    pub virtual_network: Option<Input<network::v20230401::SubResourceArgs>>,
}

/// Pool of backend IP addresses.
pub struct LoadBalancerBackendAddressPool {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// An array of references to IP addresses defined in network interfaces.
    pub backend_ip_configurations: Output<serde_json::Value>,
    /// Amount of seconds Load Balancer waits for before sending RESET to client and backend address.
    pub drain_period_in_seconds: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// An array of references to inbound NAT rules that use this backend address pool.
    pub inbound_nat_rules: Output<serde_json::Value>,
    /// An array of backend addresses.
    pub load_balancer_backend_addresses: Output<serde_json::Value>,
    /// An array of references to load balancing rules that use this backend address pool.
    pub load_balancing_rules: Output<serde_json::Value>,
    /// The location of the backend address pool.
    pub location: Output<serde_json::Value>,
    /// The name of the resource that is unique within the set of backend address pools used by the load balancer. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// A reference to an outbound rule that uses this backend address pool.
    pub outbound_rule: Output<serde_json::Value>,
    /// An array of references to outbound rules that use this backend address pool.
    pub outbound_rules: Output<serde_json::Value>,
    /// The provisioning state of the backend address pool resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Backend address synchronous mode for the backend pool
    pub sync_mode: Output<serde_json::Value>,
    /// An array of gateway load balancer tunnel interfaces.
    pub tunnel_interfaces: Output<serde_json::Value>,
    /// Type of the resource.
    pub type_: Output<serde_json::Value>,
    /// A reference to a virtual network.
    pub virtual_network: Output<serde_json::Value>,
}

impl LoadBalancerBackendAddressPool {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230401:LoadBalancerBackendAddressPool";

    /// Create a new LoadBalancerBackendAddressPool resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: LoadBalancerBackendAddressPoolArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.backend_address_pool_name {
            pulumi_sdk::resolve_input("backendAddressPoolName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.drain_period_in_seconds {
            pulumi_sdk::resolve_input("drainPeriodInSeconds", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.load_balancer_backend_addresses {
            pulumi_sdk::resolve_input_vec("loadBalancerBackendAddresses", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("loadBalancerName", args.load_balancer_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.sync_mode {
            pulumi_sdk::resolve_input("syncMode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tunnel_interfaces {
            pulumi_sdk::resolve_input_vec("tunnelInterfaces", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network {
            pulumi_sdk::resolve_input("virtualNetwork", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            backend_ip_configurations: registered.outputs.get("backendIPConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            drain_period_in_seconds: registered.outputs.get("drainPeriodInSeconds")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            inbound_nat_rules: registered.outputs.get("inboundNatRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            load_balancer_backend_addresses: registered.outputs.get("loadBalancerBackendAddresses")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            load_balancing_rules: registered.outputs.get("loadBalancingRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            outbound_rule: registered.outputs.get("outboundRule")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            outbound_rules: registered.outputs.get("outboundRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sync_mode: registered.outputs.get("syncMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tunnel_interfaces: registered.outputs.get("tunnelInterfaces")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network: registered.outputs.get("virtualNetwork")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
