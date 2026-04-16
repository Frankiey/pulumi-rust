use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Private endpoint resource.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2019-04-01, 2019-06-01, 2019-07-01, 2019-08-01, 2019-09-01, 2019-11-01, 2019-12-01, 2020-03-01, 2020-04-01, 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct PrivateEndpointArgs {
    /// Application security groups in which the private endpoint IP configuration is included.
    pub application_security_groups: Option<Vec<Input<network::ApplicationSecurityGroupArgs>>>,
    /// An array of custom dns configurations.
    pub custom_dns_configs: Option<Vec<Input<network::CustomDnsConfigPropertiesFormatArgs>>>,
    /// The custom name of the network interface attached to the private endpoint.
    pub custom_network_interface_name: Option<Input<String>>,
    /// The extended location of the load balancer.
    pub extended_location: Option<Input<network::ExtendedLocationArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// A list of IP configurations of the private endpoint. This will be used to map to the First Party Service's endpoints.
    pub ip_configurations: Option<Vec<Input<network::PrivateEndpointIPConfigurationArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// A grouping of information about the connection to the remote resource. Used when the network admin does not have access to approve connections to the remote resource.
    pub manual_private_link_service_connections: Option<Vec<Input<network::PrivateLinkServiceConnectionArgs>>>,
    /// The name of the private endpoint.
    pub private_endpoint_name: Option<Input<String>>,
    /// A grouping of information about the connection to the remote resource.
    pub private_link_service_connections: Option<Vec<Input<network::PrivateLinkServiceConnectionArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The ID of the subnet from which the private IP will be allocated.
    pub subnet: Option<Input<network::SubnetArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Private endpoint resource.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2019-04-01, 2019-06-01, 2019-07-01, 2019-08-01, 2019-09-01, 2019-11-01, 2019-12-01, 2020-03-01, 2020-04-01, 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct PrivateEndpoint {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Application security groups in which the private endpoint IP configuration is included.
    pub application_security_groups: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// An array of custom dns configurations.
    pub custom_dns_configs: Output<serde_json::Value>,
    /// The custom name of the network interface attached to the private endpoint.
    pub custom_network_interface_name: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the load balancer.
    pub extended_location: Output<serde_json::Value>,
    /// A list of IP configurations of the private endpoint. This will be used to map to the First Party Service's endpoints.
    pub ip_configurations: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// A grouping of information about the connection to the remote resource. Used when the network admin does not have access to approve connections to the remote resource.
    pub manual_private_link_service_connections: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// An array of references to the network interfaces created for this private endpoint.
    pub network_interfaces: Output<serde_json::Value>,
    /// A grouping of information about the connection to the remote resource.
    pub private_link_service_connections: Output<serde_json::Value>,
    /// The provisioning state of the private endpoint resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The ID of the subnet from which the private IP will be allocated.
    pub subnet: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl PrivateEndpoint {
    const TYPE_TOKEN: &'static str = "azure-native:network:PrivateEndpoint";

    /// Create a new PrivateEndpoint resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PrivateEndpointArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.application_security_groups {
            pulumi_sdk::resolve_input_vec("applicationSecurityGroups", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.custom_dns_configs {
            pulumi_sdk::resolve_input_vec("customDnsConfigs", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.custom_network_interface_name {
            pulumi_sdk::resolve_input("customNetworkInterfaceName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_configurations {
            pulumi_sdk::resolve_input_vec("ipConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.manual_private_link_service_connections {
            pulumi_sdk::resolve_input_vec("manualPrivateLinkServiceConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_endpoint_name {
            pulumi_sdk::resolve_input("privateEndpointName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_link_service_connections {
            pulumi_sdk::resolve_input_vec("privateLinkServiceConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.subnet {
            pulumi_sdk::resolve_input("subnet", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            application_security_groups: registered.outputs.get("applicationSecurityGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_dns_configs: registered.outputs.get("customDnsConfigs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_network_interface_name: registered.outputs.get("customNetworkInterfaceName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            manual_private_link_service_connections: registered.outputs.get("manualPrivateLinkServiceConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_interfaces: registered.outputs.get("networkInterfaces")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_link_service_connections: registered.outputs.get("privateLinkServiceConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            subnet: registered.outputs.get("subnet")
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
