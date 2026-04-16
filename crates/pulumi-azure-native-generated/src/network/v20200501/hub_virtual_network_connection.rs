use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// HubVirtualNetworkConnection Resource.
pub struct HubVirtualNetworkConnectionArgs {
    /// Deprecated: VirtualHub to RemoteVnet transit to enabled or not.
    pub allow_hub_to_remote_vnet_transit: Option<Input<bool>>,
    /// Deprecated: Allow RemoteVnet to use Virtual Hub's gateways.
    pub allow_remote_vnet_to_use_hub_vnet_gateways: Option<Input<bool>>,
    /// The name of the HubVirtualNetworkConnection.
    pub connection_name: Option<Input<String>>,
    /// Enable internet security.
    pub enable_internet_security: Option<Input<bool>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// Reference to the remote virtual network.
    pub remote_virtual_network: Option<Input<network::v20200501::SubResourceArgs>>,
    /// The resource group name of the HubVirtualNetworkConnection.
    pub resource_group_name: Input<String>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Option<Input<network::v20200501::RoutingConfigurationArgs>>,
    /// The name of the VirtualHub.
    pub virtual_hub_name: Input<String>,
}

/// HubVirtualNetworkConnection Resource.
pub struct HubVirtualNetworkConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Deprecated: VirtualHub to RemoteVnet transit to enabled or not.
    pub allow_hub_to_remote_vnet_transit: Output<serde_json::Value>,
    /// Deprecated: Allow RemoteVnet to use Virtual Hub's gateways.
    pub allow_remote_vnet_to_use_hub_vnet_gateways: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Enable internet security.
    pub enable_internet_security: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the hub virtual network connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Reference to the remote virtual network.
    pub remote_virtual_network: Output<serde_json::Value>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Output<serde_json::Value>,
}

impl HubVirtualNetworkConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20200501:HubVirtualNetworkConnection";

    /// Create a new HubVirtualNetworkConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: HubVirtualNetworkConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.allow_hub_to_remote_vnet_transit {
            pulumi_sdk::resolve_input("allowHubToRemoteVnetTransit", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_remote_vnet_to_use_hub_vnet_gateways {
            pulumi_sdk::resolve_input("allowRemoteVnetToUseHubVnetGateways", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.connection_name {
            pulumi_sdk::resolve_input("connectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_internet_security {
            pulumi_sdk::resolve_input("enableInternetSecurity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.remote_virtual_network {
            pulumi_sdk::resolve_input("remoteVirtualNetwork", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.routing_configuration {
            pulumi_sdk::resolve_input("routingConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualHubName", args.virtual_hub_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            allow_hub_to_remote_vnet_transit: registered.outputs.get("allowHubToRemoteVnetTransit")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_remote_vnet_to_use_hub_vnet_gateways: registered.outputs.get("allowRemoteVnetToUseHubVnetGateways")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_internet_security: registered.outputs.get("enableInternetSecurity")
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
            remote_virtual_network: registered.outputs.get("remoteVirtualNetwork")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_configuration: registered.outputs.get("routingConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
