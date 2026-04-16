use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Subnet in a virtual network resource.
pub struct SubnetArgs {
    /// The address prefix for the subnet.
    pub address_prefix: Option<Input<String>>,
    /// List of address prefixes for the subnet.
    pub address_prefixes: Option<Vec<Input<String>>>,
    /// An array of references to the delegations on the subnet.
    pub delegations: Option<Vec<Input<network::v20200501::DelegationArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Array of IpAllocation which reference this subnet.
    pub ip_allocations: Option<Vec<Input<network::v20200501::SubResourceArgs>>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// Nat gateway associated with this subnet.
    pub nat_gateway: Option<Input<network::v20200501::SubResourceArgs>>,
    /// The reference to the NetworkSecurityGroup resource.
    pub network_security_group: Option<Input<network::v20200501::NetworkSecurityGroupArgs>>,
    /// Enable or Disable apply network policies on private end point in the subnet.
    pub private_endpoint_network_policies: Option<Input<String>>,
    /// Enable or Disable apply network policies on private link service in the subnet.
    pub private_link_service_network_policies: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The reference to the RouteTable resource.
    pub route_table: Option<Input<network::v20200501::RouteTableArgs>>,
    /// An array of service endpoint policies.
    pub service_endpoint_policies: Option<Vec<Input<network::v20200501::ServiceEndpointPolicyArgs>>>,
    /// An array of service endpoints.
    pub service_endpoints: Option<Vec<Input<network::v20200501::ServiceEndpointPropertiesFormatArgs>>>,
    /// The name of the subnet.
    pub subnet_name: Option<Input<String>>,
    /// The name of the virtual network.
    pub virtual_network_name: Input<String>,
}

/// Subnet in a virtual network resource.
pub struct Subnet {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The address prefix for the subnet.
    pub address_prefix: Output<serde_json::Value>,
    /// List of address prefixes for the subnet.
    pub address_prefixes: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// An array of references to the delegations on the subnet.
    pub delegations: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Array of IpAllocation which reference this subnet.
    pub ip_allocations: Output<serde_json::Value>,
    /// Array of IP configuration profiles which reference this subnet.
    pub ip_configuration_profiles: Output<serde_json::Value>,
    /// An array of references to the network interface IP configurations using subnet.
    pub ip_configurations: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// Nat gateway associated with this subnet.
    pub nat_gateway: Output<serde_json::Value>,
    /// The reference to the NetworkSecurityGroup resource.
    pub network_security_group: Output<serde_json::Value>,
    /// Enable or Disable apply network policies on private end point in the subnet.
    pub private_endpoint_network_policies: Output<serde_json::Value>,
    /// An array of references to private endpoints.
    pub private_endpoints: Output<serde_json::Value>,
    /// Enable or Disable apply network policies on private link service in the subnet.
    pub private_link_service_network_policies: Output<serde_json::Value>,
    /// The provisioning state of the subnet resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// A read-only string identifying the intention of use for this subnet based on delegations and other user-defined properties.
    pub purpose: Output<serde_json::Value>,
    /// An array of references to the external resources using subnet.
    pub resource_navigation_links: Output<serde_json::Value>,
    /// The reference to the RouteTable resource.
    pub route_table: Output<serde_json::Value>,
    /// An array of references to services injecting into this subnet.
    pub service_association_links: Output<serde_json::Value>,
    /// An array of service endpoint policies.
    pub service_endpoint_policies: Output<serde_json::Value>,
    /// An array of service endpoints.
    pub service_endpoints: Output<serde_json::Value>,
}

impl Subnet {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20200501:Subnet";

    /// Create a new Subnet resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: SubnetArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.address_prefix {
            pulumi_sdk::resolve_input("addressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.address_prefixes {
            pulumi_sdk::resolve_input_vec("addressPrefixes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.delegations {
            pulumi_sdk::resolve_input_vec("delegations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_allocations {
            pulumi_sdk::resolve_input_vec("ipAllocations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nat_gateway {
            pulumi_sdk::resolve_input("natGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_security_group {
            pulumi_sdk::resolve_input("networkSecurityGroup", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_endpoint_network_policies {
            pulumi_sdk::resolve_input("privateEndpointNetworkPolicies", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_link_service_network_policies {
            pulumi_sdk::resolve_input("privateLinkServiceNetworkPolicies", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.route_table {
            pulumi_sdk::resolve_input("routeTable", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_endpoint_policies {
            pulumi_sdk::resolve_input_vec("serviceEndpointPolicies", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_endpoints {
            pulumi_sdk::resolve_input_vec("serviceEndpoints", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.subnet_name {
            pulumi_sdk::resolve_input("subnetName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualNetworkName", args.virtual_network_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            address_prefix: registered.outputs.get("addressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            address_prefixes: registered.outputs.get("addressPrefixes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            delegations: registered.outputs.get("delegations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_allocations: registered.outputs.get("ipAllocations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configuration_profiles: registered.outputs.get("ipConfigurationProfiles")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            nat_gateway: registered.outputs.get("natGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_security_group: registered.outputs.get("networkSecurityGroup")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_endpoint_network_policies: registered.outputs.get("privateEndpointNetworkPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_endpoints: registered.outputs.get("privateEndpoints")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_link_service_network_policies: registered.outputs.get("privateLinkServiceNetworkPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            purpose: registered.outputs.get("purpose")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_navigation_links: registered.outputs.get("resourceNavigationLinks")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_table: registered.outputs.get("routeTable")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_association_links: registered.outputs.get("serviceAssociationLinks")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_endpoint_policies: registered.outputs.get("serviceEndpointPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_endpoints: registered.outputs.get("serviceEndpoints")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
