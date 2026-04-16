use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Interface endpoint resource.
pub struct InterfaceEndpointArgs {
    /// A reference to the service being brought into the virtual network.
    pub endpoint_service: Option<Input<network::v20181101::EndpointServiceArgs>>,
    /// A first-party service's FQDN that is mapped to the private IP allocated via this interface endpoint.
    pub fqdn: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the interface endpoint.
    pub interface_endpoint_name: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The ID of the subnet from which the private IP will be allocated.
    pub subnet: Option<Input<network::v20181101::SubnetArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Interface endpoint resource.
pub struct InterfaceEndpoint {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A reference to the service being brought into the virtual network.
    pub endpoint_service: Output<serde_json::Value>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// A first-party service's FQDN that is mapped to the private IP allocated via this interface endpoint.
    pub fqdn: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// Gets an array of references to the network interfaces created for this interface endpoint.
    pub network_interfaces: Output<serde_json::Value>,
    /// A read-only property that identifies who created this interface endpoint.
    pub owner: Output<serde_json::Value>,
    /// The provisioning state of the interface endpoint. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Output<serde_json::Value>,
    /// The ID of the subnet from which the private IP will be allocated.
    pub subnet: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl InterfaceEndpoint {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20181101:InterfaceEndpoint";

    /// Create a new InterfaceEndpoint resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: InterfaceEndpointArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.endpoint_service {
            pulumi_sdk::resolve_input("endpointService", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.fqdn {
            pulumi_sdk::resolve_input("fqdn", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.interface_endpoint_name {
            pulumi_sdk::resolve_input("interfaceEndpointName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            endpoint_service: registered.outputs.get("endpointService")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            fqdn: registered.outputs.get("fqdn")
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
            owner: registered.outputs.get("owner")
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
