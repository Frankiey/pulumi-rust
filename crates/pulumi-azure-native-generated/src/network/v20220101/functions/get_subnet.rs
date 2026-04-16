use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified subnet by virtual network and resource group.
#[derive(Default)]
pub struct GetSubnetArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the subnet.
    pub subnet_name: String,
    /// The name of the virtual network.
    pub virtual_network_name: String,
}

/// Result of the function invocation.
pub struct GetSubnetResult {
    /// The address prefix for the subnet.
    pub address_prefix: Option<String>,
    /// List of address prefixes for the subnet.
    pub address_prefixes: Option<Vec<String>>,
    /// Application gateway IP configurations of virtual network resource.
    pub application_gateway_ip_configurations: Option<Vec<network::v20220101::ApplicationGatewayIPConfigurationResponse>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// An array of references to the delegations on the subnet.
    pub delegations: Option<Vec<network::v20220101::DelegationResponse>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Array of IpAllocation which reference this subnet.
    pub ip_allocations: Option<Vec<network::v20220101::SubResourceResponse>>,
    /// Array of IP configuration profiles which reference this subnet.
    pub ip_configuration_profiles: Vec<network::v20220101::IPConfigurationProfileResponse>,
    /// An array of references to the network interface IP configurations using subnet.
    pub ip_configurations: Vec<network::v20220101::IPConfigurationResponse>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Nat gateway associated with this subnet.
    pub nat_gateway: Option<network::v20220101::SubResourceResponse>,
    /// The reference to the NetworkSecurityGroup resource.
    pub network_security_group: Option<network::v20220101::NetworkSecurityGroupResponse>,
    /// Enable or Disable apply network policies on private end point in the subnet.
    pub private_endpoint_network_policies: Option<String>,
    /// An array of references to private endpoints.
    pub private_endpoints: Vec<network::v20220101::PrivateEndpointResponse>,
    /// Enable or Disable apply network policies on private link service in the subnet.
    pub private_link_service_network_policies: Option<String>,
    /// The provisioning state of the subnet resource.
    pub provisioning_state: String,
    /// A read-only string identifying the intention of use for this subnet based on delegations and other user-defined properties.
    pub purpose: String,
    /// An array of references to the external resources using subnet.
    pub resource_navigation_links: Vec<network::v20220101::ResourceNavigationLinkResponse>,
    /// The reference to the RouteTable resource.
    pub route_table: Option<network::v20220101::RouteTableResponse>,
    /// An array of references to services injecting into this subnet.
    pub service_association_links: Vec<network::v20220101::ServiceAssociationLinkResponse>,
    /// An array of service endpoint policies.
    pub service_endpoint_policies: Option<Vec<network::v20220101::ServiceEndpointPolicyResponse>>,
    /// An array of service endpoints.
    pub service_endpoints: Option<Vec<network::v20220101::ServiceEndpointPropertiesFormatResponse>>,
    /// Resource type.
    pub type_: Option<String>,
}

/// Gets the specified subnet by virtual network and resource group.
pub async fn get_subnet(
    ctx: &Context,
    args: GetSubnetArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetSubnetResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("subnetName".to_string(), json!(args.subnet_name));
    invoke_args.insert("virtualNetworkName".to_string(), json!(args.virtual_network_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220101:getSubnet", invoke_args, &opts).await?;

    Ok(GetSubnetResult {
        address_prefix: result.fields.get("addressPrefix").cloned().map(serde_json::from_value).transpose()?,
        address_prefixes: result.fields.get("addressPrefixes").cloned().map(serde_json::from_value).transpose()?,
        application_gateway_ip_configurations: result.fields.get("applicationGatewayIpConfigurations").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        delegations: result.fields.get("delegations").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_allocations: result.fields.get("ipAllocations").cloned().map(serde_json::from_value).transpose()?,
        ip_configuration_profiles: serde_json::from_value(result.fields.get("ipConfigurationProfiles").cloned().unwrap_or_default())?
            ,
        ip_configurations: serde_json::from_value(result.fields.get("ipConfigurations").cloned().unwrap_or_default())?
            ,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        nat_gateway: result.fields.get("natGateway").cloned().map(serde_json::from_value).transpose()?,
        network_security_group: result.fields.get("networkSecurityGroup").cloned().map(serde_json::from_value).transpose()?,
        private_endpoint_network_policies: result.fields.get("privateEndpointNetworkPolicies").cloned().map(serde_json::from_value).transpose()?,
        private_endpoints: serde_json::from_value(result.fields.get("privateEndpoints").cloned().unwrap_or_default())?
            ,
        private_link_service_network_policies: result.fields.get("privateLinkServiceNetworkPolicies").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        purpose: serde_json::from_value(result.fields.get("purpose").cloned().unwrap_or_default())?
            ,
        resource_navigation_links: serde_json::from_value(result.fields.get("resourceNavigationLinks").cloned().unwrap_or_default())?
            ,
        route_table: result.fields.get("routeTable").cloned().map(serde_json::from_value).transpose()?,
        service_association_links: serde_json::from_value(result.fields.get("serviceAssociationLinks").cloned().unwrap_or_default())?
            ,
        service_endpoint_policies: result.fields.get("serviceEndpointPolicies").cloned().map(serde_json::from_value).transpose()?,
        service_endpoints: result.fields.get("serviceEndpoints").cloned().map(serde_json::from_value).transpose()?,
        type_: result.fields.get("type").cloned().map(serde_json::from_value).transpose()?,
    })
}
