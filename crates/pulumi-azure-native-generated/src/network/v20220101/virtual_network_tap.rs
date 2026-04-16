use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Virtual Network Tap resource.
pub struct VirtualNetworkTapArgs {
    /// The reference to the private IP address on the internal Load Balancer that will receive the tap.
    pub destination_load_balancer_front_end_ip_configuration: Option<Input<network::v20220101::FrontendIPConfigurationArgs>>,
    /// The reference to the private IP Address of the collector nic that will receive the tap.
    pub destination_network_interface_ip_configuration: Option<Input<network::v20220101::NetworkInterfaceIPConfigurationArgs>>,
    /// The VXLAN destination port that will receive the tapped traffic.
    pub destination_port: Option<Input<i64>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The name of the virtual network tap.
    pub tap_name: Option<Input<String>>,
}

/// Virtual Network Tap resource.
pub struct VirtualNetworkTap {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The reference to the private IP address on the internal Load Balancer that will receive the tap.
    pub destination_load_balancer_front_end_ip_configuration: Output<serde_json::Value>,
    /// The reference to the private IP Address of the collector nic that will receive the tap.
    pub destination_network_interface_ip_configuration: Output<serde_json::Value>,
    /// The VXLAN destination port that will receive the tapped traffic.
    pub destination_port: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// Specifies the list of resource IDs for the network interface IP configuration that needs to be tapped.
    pub network_interface_tap_configurations: Output<serde_json::Value>,
    /// The provisioning state of the virtual network tap resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the virtual network tap resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl VirtualNetworkTap {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220101:VirtualNetworkTap";

    /// Create a new VirtualNetworkTap resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualNetworkTapArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.destination_load_balancer_front_end_ip_configuration {
            pulumi_sdk::resolve_input("destinationLoadBalancerFrontEndIPConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_network_interface_ip_configuration {
            pulumi_sdk::resolve_input("destinationNetworkInterfaceIPConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_port {
            pulumi_sdk::resolve_input("destinationPort", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tap_name {
            pulumi_sdk::resolve_input("tapName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            destination_load_balancer_front_end_ip_configuration: registered.outputs.get("destinationLoadBalancerFrontEndIPConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_network_interface_ip_configuration: registered.outputs.get("destinationNetworkInterfaceIPConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_port: registered.outputs.get("destinationPort")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_interface_tap_configurations: registered.outputs.get("networkInterfaceTapConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
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
