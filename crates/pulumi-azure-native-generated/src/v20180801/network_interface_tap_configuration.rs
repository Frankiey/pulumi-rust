use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Tap configuration in a Network Interface
pub struct NetworkInterfaceTapConfigurationArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the network interface.
    pub network_interface_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the tap configuration.
    pub tap_configuration_name: Option<Input<String>>,
    /// The reference of the Virtual Network Tap resource.
    pub virtual_network_tap: Option<Input<network::v20180801::VirtualNetworkTapArgs>>,
}

/// Tap configuration in a Network Interface
pub struct NetworkInterfaceTapConfiguration {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the network interface tap configuration. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Output<serde_json::Value>,
    /// Sub Resource type.
    pub type_: Output<serde_json::Value>,
    /// The reference of the Virtual Network Tap resource.
    pub virtual_network_tap: Output<serde_json::Value>,
}

impl NetworkInterfaceTapConfiguration {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20180801:NetworkInterfaceTapConfiguration";

    /// Create a new NetworkInterfaceTapConfiguration resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkInterfaceTapConfigurationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkInterfaceName", args.network_interface_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tap_configuration_name {
            pulumi_sdk::resolve_input("tapConfigurationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network_tap {
            pulumi_sdk::resolve_input("virtualNetworkTap", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network_tap: registered.outputs.get("virtualNetworkTap")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
