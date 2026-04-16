use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Information about packet capture session.
pub struct PacketCaptureArgs {
    /// Number of bytes captured per packet, the remaining bytes are truncated.
    pub bytes_to_capture_per_packet: Option<Input<f64>>,
    /// A list of packet capture filters.
    pub filters: Option<Vec<Input<network::v20210801::PacketCaptureFilterArgs>>>,
    /// The name of the network watcher.
    pub network_watcher_name: Input<String>,
    /// The name of the packet capture session.
    pub packet_capture_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The storage location for a packet capture session.
    pub storage_location: Input<network::v20210801::PacketCaptureStorageLocationArgs>,
    /// The ID of the targeted resource, only VM is currently supported.
    pub target: Input<String>,
    /// Maximum duration of the capture session in seconds.
    pub time_limit_in_seconds: Option<Input<i64>>,
    /// Maximum size of the capture output.
    pub total_bytes_per_session: Option<Input<f64>>,
}

/// Information about packet capture session.
pub struct PacketCapture {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Number of bytes captured per packet, the remaining bytes are truncated.
    pub bytes_to_capture_per_packet: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// A list of packet capture filters.
    pub filters: Output<serde_json::Value>,
    /// Name of the packet capture session.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the packet capture session.
    pub provisioning_state: Output<serde_json::Value>,
    /// The storage location for a packet capture session.
    pub storage_location: Output<serde_json::Value>,
    /// The ID of the targeted resource, only VM is currently supported.
    pub target: Output<serde_json::Value>,
    /// Maximum duration of the capture session in seconds.
    pub time_limit_in_seconds: Output<serde_json::Value>,
    /// Maximum size of the capture output.
    pub total_bytes_per_session: Output<serde_json::Value>,
}

impl PacketCapture {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20210801:PacketCapture";

    /// Create a new PacketCapture resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PacketCaptureArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.bytes_to_capture_per_packet {
            pulumi_sdk::resolve_input("bytesToCapturePerPacket", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.filters {
            pulumi_sdk::resolve_input_vec("filters", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkWatcherName", args.network_watcher_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.packet_capture_name {
            pulumi_sdk::resolve_input("packetCaptureName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("storageLocation", args.storage_location, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("target", args.target, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.time_limit_in_seconds {
            pulumi_sdk::resolve_input("timeLimitInSeconds", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.total_bytes_per_session {
            pulumi_sdk::resolve_input("totalBytesPerSession", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            bytes_to_capture_per_packet: registered.outputs.get("bytesToCapturePerPacket")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            filters: registered.outputs.get("filters")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            storage_location: registered.outputs.get("storageLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            target: registered.outputs.get("target")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            time_limit_in_seconds: registered.outputs.get("timeLimitInSeconds")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            total_bytes_per_session: registered.outputs.get("totalBytesPerSession")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
