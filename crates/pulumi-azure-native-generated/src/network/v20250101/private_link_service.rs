use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Private link service resource.
pub struct PrivateLinkServiceArgs {
    /// The access mode of the private link service.
    pub access_mode: Option<Input<serde_json::Value>>,
    /// The auto-approval list of the private link service.
    pub auto_approval: Option<Input<network::v20250101::PrivateLinkServicePropertiesAutoApprovalArgs>>,
    /// The destination IP address of the private link service.
    pub destination_ip_address: Option<Input<String>>,
    /// Whether the private link service is enabled for proxy protocol or not.
    pub enable_proxy_protocol: Option<Input<bool>>,
    /// The extended location of the load balancer.
    pub extended_location: Option<Input<network::v20250101::ExtendedLocationArgs>>,
    /// The list of Fqdn.
    pub fqdns: Option<Vec<Input<String>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// An array of private link service IP configurations.
    pub ip_configurations: Option<Vec<Input<network::v20250101::PrivateLinkServiceIpConfigurationArgs>>>,
    /// An array of references to the load balancer IP configurations.
    pub load_balancer_frontend_ip_configurations: Option<Vec<Input<network::v20250101::FrontendIPConfigurationArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the private link service.
    pub service_name: Option<Input<String>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The visibility list of the private link service.
    pub visibility: Option<Input<network::v20250101::PrivateLinkServicePropertiesVisibilityArgs>>,
}

/// Private link service resource.
pub struct PrivateLinkService {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The access mode of the private link service.
    pub access_mode: Output<serde_json::Value>,
    /// The alias of the private link service.
    pub alias: Output<serde_json::Value>,
    /// The auto-approval list of the private link service.
    pub auto_approval: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The destination IP address of the private link service.
    pub destination_ip_address: Output<serde_json::Value>,
    /// Whether the private link service is enabled for proxy protocol or not.
    pub enable_proxy_protocol: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the load balancer.
    pub extended_location: Output<serde_json::Value>,
    /// The list of Fqdn.
    pub fqdns: Output<serde_json::Value>,
    /// An array of private link service IP configurations.
    pub ip_configurations: Output<serde_json::Value>,
    /// An array of references to the load balancer IP configurations.
    pub load_balancer_frontend_ip_configurations: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// An array of references to the network interfaces created for this private link service.
    pub network_interfaces: Output<serde_json::Value>,
    /// An array of list about connections to the private endpoint.
    pub private_endpoint_connections: Output<serde_json::Value>,
    /// The provisioning state of the private link service resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The visibility list of the private link service.
    pub visibility: Output<serde_json::Value>,
}

impl PrivateLinkService {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250101:PrivateLinkService";

    /// Create a new PrivateLinkService resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PrivateLinkServiceArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.access_mode {
            pulumi_sdk::resolve_input("accessMode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.auto_approval {
            pulumi_sdk::resolve_input("autoApproval", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_ip_address {
            pulumi_sdk::resolve_input("destinationIPAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_proxy_protocol {
            pulumi_sdk::resolve_input("enableProxyProtocol", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.fqdns {
            pulumi_sdk::resolve_input_vec("fqdns", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_configurations {
            pulumi_sdk::resolve_input_vec("ipConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.load_balancer_frontend_ip_configurations {
            pulumi_sdk::resolve_input_vec("loadBalancerFrontendIpConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.service_name {
            pulumi_sdk::resolve_input("serviceName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.visibility {
            pulumi_sdk::resolve_input("visibility", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            access_mode: registered.outputs.get("accessMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            alias: registered.outputs.get("alias")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            auto_approval: registered.outputs.get("autoApproval")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_ip_address: registered.outputs.get("destinationIPAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_proxy_protocol: registered.outputs.get("enableProxyProtocol")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            fqdns: registered.outputs.get("fqdns")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            load_balancer_frontend_ip_configurations: registered.outputs.get("loadBalancerFrontendIpConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_interfaces: registered.outputs.get("networkInterfaces")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_endpoint_connections: registered.outputs.get("privateEndpointConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            visibility: registered.outputs.get("visibility")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
