use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Subnet in a virtual network resource.
pub struct SubnetArgs {
    /// The address prefix for the subnet.
    pub address_prefix: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The reference of the NetworkSecurityGroup resource.
    pub network_security_group: Option<Input<network::v20180601::NetworkSecurityGroupArgs>>,
    /// The provisioning state of the resource.
    pub provisioning_state: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Gets an array of references to the external resources using subnet.
    pub resource_navigation_links: Option<Vec<Input<network::v20180601::ResourceNavigationLinkArgs>>>,
    /// The reference of the RouteTable resource.
    pub route_table: Option<Input<network::v20180601::RouteTableArgs>>,
    /// An array of service endpoints.
    pub service_endpoints: Option<Vec<Input<network::v20180601::ServiceEndpointPropertiesFormatArgs>>>,
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
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Gets an array of references to the network interface IP configurations using subnet.
    pub ip_configurations: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The reference of the NetworkSecurityGroup resource.
    pub network_security_group: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Gets an array of references to the external resources using subnet.
    pub resource_navigation_links: Output<serde_json::Value>,
    /// The reference of the RouteTable resource.
    pub route_table: Output<serde_json::Value>,
    /// An array of service endpoints.
    pub service_endpoints: Output<serde_json::Value>,
}

impl Subnet {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20180601:Subnet";

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
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_security_group {
            pulumi_sdk::resolve_input("networkSecurityGroup", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.provisioning_state {
            pulumi_sdk::resolve_input("provisioningState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.resource_navigation_links {
            pulumi_sdk::resolve_input_vec("resourceNavigationLinks", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.route_table {
            pulumi_sdk::resolve_input("routeTable", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_security_group: registered.outputs.get("networkSecurityGroup")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_navigation_links: registered.outputs.get("resourceNavigationLinks")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_table: registered.outputs.get("routeTable")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_endpoints: registered.outputs.get("serviceEndpoints")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
