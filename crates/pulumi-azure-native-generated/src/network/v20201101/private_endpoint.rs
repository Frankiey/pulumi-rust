use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Private endpoint resource.
pub struct PrivateEndpointArgs {
    /// An array of custom dns configurations.
    pub custom_dns_configs: Option<Vec<Input<network::v20201101::CustomDnsConfigPropertiesFormatArgs>>>,
    /// The extended location of the load balancer.
    pub extended_location: Option<Input<network::v20201101::ExtendedLocationArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// A grouping of information about the connection to the remote resource. Used when the network admin does not have access to approve connections to the remote resource.
    pub manual_private_link_service_connections: Option<Vec<Input<network::v20201101::PrivateLinkServiceConnectionArgs>>>,
    /// The name of the private endpoint.
    pub private_endpoint_name: Option<Input<String>>,
    /// A grouping of information about the connection to the remote resource.
    pub private_link_service_connections: Option<Vec<Input<network::v20201101::PrivateLinkServiceConnectionArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The ID of the subnet from which the private IP will be allocated.
    pub subnet: Option<Input<network::v20201101::SubnetArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Private endpoint resource.
pub struct PrivateEndpoint {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// An array of custom dns configurations.
    pub custom_dns_configs: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the load balancer.
    pub extended_location: Output<serde_json::Value>,
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
    const TYPE_TOKEN: &'static str = "azure-native:network/v20201101:PrivateEndpoint";

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

        if let Some(v) = args.custom_dns_configs {
            pulumi_sdk::resolve_input_vec("customDnsConfigs", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_dns_configs: registered.outputs.get("customDnsConfigs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
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
