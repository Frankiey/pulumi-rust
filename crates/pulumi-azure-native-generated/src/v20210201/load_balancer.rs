use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// LoadBalancer resource.
pub struct LoadBalancerArgs {
    /// Collection of backend address pools used by a load balancer.
    pub backend_address_pools: Option<Vec<Input<network::v20210201::BackendAddressPoolArgs>>>,
    /// The extended location of the load balancer.
    pub extended_location: Option<Input<network::v20210201::ExtendedLocationArgs>>,
    /// Object representing the frontend IPs to be used for the load balancer.
    pub frontend_ip_configurations: Option<Vec<Input<network::v20210201::FrontendIPConfigurationArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Defines an external port range for inbound NAT to a single backend port on NICs associated with a load balancer. Inbound NAT rules are created automatically for each NIC associated with the Load Balancer using an external port from this range. Defining an Inbound NAT pool on your Load Balancer is mutually exclusive with defining inbound Nat rules. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an inbound NAT pool. They have to reference individual inbound NAT rules.
    pub inbound_nat_pools: Option<Vec<Input<network::v20210201::InboundNatPoolArgs>>>,
    /// Collection of inbound NAT Rules used by a load balancer. Defining inbound NAT rules on your load balancer is mutually exclusive with defining an inbound NAT pool. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an Inbound NAT pool. They have to reference individual inbound NAT rules.
    pub inbound_nat_rules: Option<Vec<Input<network::v20210201::InboundNatRuleArgs>>>,
    /// The name of the load balancer.
    pub load_balancer_name: Option<Input<String>>,
    /// Object collection representing the load balancing rules Gets the provisioning.
    pub load_balancing_rules: Option<Vec<Input<network::v20210201::LoadBalancingRuleArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The outbound rules.
    pub outbound_rules: Option<Vec<Input<network::v20210201::OutboundRuleArgs>>>,
    /// Collection of probe objects used in the load balancer.
    pub probes: Option<Vec<Input<network::v20210201::ProbeArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The load balancer SKU.
    pub sku: Option<Input<network::v20210201::LoadBalancerSkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// LoadBalancer resource.
pub struct LoadBalancer {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Collection of backend address pools used by a load balancer.
    pub backend_address_pools: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the load balancer.
    pub extended_location: Output<serde_json::Value>,
    /// Object representing the frontend IPs to be used for the load balancer.
    pub frontend_ip_configurations: Output<serde_json::Value>,
    /// Defines an external port range for inbound NAT to a single backend port on NICs associated with a load balancer. Inbound NAT rules are created automatically for each NIC associated with the Load Balancer using an external port from this range. Defining an Inbound NAT pool on your Load Balancer is mutually exclusive with defining inbound Nat rules. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an inbound NAT pool. They have to reference individual inbound NAT rules.
    pub inbound_nat_pools: Output<serde_json::Value>,
    /// Collection of inbound NAT Rules used by a load balancer. Defining inbound NAT rules on your load balancer is mutually exclusive with defining an inbound NAT pool. Inbound NAT pools are referenced from virtual machine scale sets. NICs that are associated with individual virtual machines cannot reference an Inbound NAT pool. They have to reference individual inbound NAT rules.
    pub inbound_nat_rules: Output<serde_json::Value>,
    /// Object collection representing the load balancing rules Gets the provisioning.
    pub load_balancing_rules: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The outbound rules.
    pub outbound_rules: Output<serde_json::Value>,
    /// Collection of probe objects used in the load balancer.
    pub probes: Output<serde_json::Value>,
    /// The provisioning state of the load balancer resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the load balancer resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The load balancer SKU.
    pub sku: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl LoadBalancer {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20210201:LoadBalancer";

    /// Create a new LoadBalancer resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: LoadBalancerArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.backend_address_pools {
            pulumi_sdk::resolve_input_vec("backendAddressPools", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.frontend_ip_configurations {
            pulumi_sdk::resolve_input_vec("frontendIPConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.inbound_nat_pools {
            pulumi_sdk::resolve_input_vec("inboundNatPools", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.inbound_nat_rules {
            pulumi_sdk::resolve_input_vec("inboundNatRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.load_balancer_name {
            pulumi_sdk::resolve_input("loadBalancerName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.load_balancing_rules {
            pulumi_sdk::resolve_input_vec("loadBalancingRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.outbound_rules {
            pulumi_sdk::resolve_input_vec("outboundRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.probes {
            pulumi_sdk::resolve_input_vec("probes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            backend_address_pools: registered.outputs.get("backendAddressPools")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            frontend_ip_configurations: registered.outputs.get("frontendIPConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            inbound_nat_pools: registered.outputs.get("inboundNatPools")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            inbound_nat_rules: registered.outputs.get("inboundNatRules")
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
            outbound_rules: registered.outputs.get("outboundRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            probes: registered.outputs.get("probes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
