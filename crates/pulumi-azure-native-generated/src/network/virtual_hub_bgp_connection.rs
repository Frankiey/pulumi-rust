use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Virtual Appliance Site resource.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct VirtualHubBgpConnectionArgs {
    /// The name of the connection.
    pub connection_name: Option<Input<String>>,
    /// The reference to the HubVirtualNetworkConnection resource.
    pub hub_virtual_network_connection: Option<Input<network::SubResourceArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Name of the connection.
    pub name: Option<Input<String>>,
    /// Peer ASN.
    pub peer_asn: Option<Input<f64>>,
    /// Peer IP.
    pub peer_ip: Option<Input<String>>,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: Input<String>,
    /// The name of the VirtualHub.
    pub virtual_hub_name: Input<String>,
}

/// Virtual Appliance Site resource.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct VirtualHubBgpConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The current state of the VirtualHub to Peer.
    pub connection_state: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The reference to the HubVirtualNetworkConnection resource.
    pub hub_virtual_network_connection: Output<serde_json::Value>,
    /// Name of the connection.
    pub name: Output<serde_json::Value>,
    /// Peer ASN.
    pub peer_asn: Output<serde_json::Value>,
    /// Peer IP.
    pub peer_ip: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Connection type.
    pub type_: Output<serde_json::Value>,
}

impl VirtualHubBgpConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network:VirtualHubBgpConnection";

    /// Create a new VirtualHubBgpConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualHubBgpConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.connection_name {
            pulumi_sdk::resolve_input("connectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.hub_virtual_network_connection {
            pulumi_sdk::resolve_input("hubVirtualNetworkConnection", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peer_asn {
            pulumi_sdk::resolve_input("peerAsn", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peer_ip {
            pulumi_sdk::resolve_input("peerIp", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connection_state: registered.outputs.get("connectionState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            hub_virtual_network_connection: registered.outputs.get("hubVirtualNetworkConnection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peer_asn: registered.outputs.get("peerAsn")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peer_ip: registered.outputs.get("peerIp")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
