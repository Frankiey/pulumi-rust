use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a packet capture session by name.
#[derive(Default)]
pub struct GetPacketCaptureArgs {
    /// The name of the network watcher.
    pub network_watcher_name: String,
    /// The name of the packet capture session.
    pub packet_capture_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetPacketCaptureResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Number of bytes captured per packet, the remaining bytes are truncated.
    pub bytes_to_capture_per_packet: Option<i64>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// A list of packet capture filters.
    pub filters: Option<Vec<network::v20200301::PacketCaptureFilterResponse>>,
    /// ID of the packet capture operation.
    pub id: String,
    /// Name of the packet capture session.
    pub name: String,
    /// The provisioning state of the packet capture session.
    pub provisioning_state: String,
    /// The storage location for a packet capture session.
    pub storage_location: network::v20200301::PacketCaptureStorageLocationResponse,
    /// The ID of the targeted resource, only VM is currently supported.
    pub target: String,
    /// Maximum duration of the capture session in seconds.
    pub time_limit_in_seconds: Option<i64>,
    /// Maximum size of the capture output.
    pub total_bytes_per_session: Option<i64>,
}

/// Gets a packet capture session by name.
pub async fn get_packet_capture(
    ctx: &Context,
    args: GetPacketCaptureArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPacketCaptureResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkWatcherName".to_string(), json!(args.network_watcher_name));
    invoke_args.insert("packetCaptureName".to_string(), json!(args.packet_capture_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200301:getPacketCapture", invoke_args, &opts).await?;

    Ok(GetPacketCaptureResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bytes_to_capture_per_packet: result.fields.get("bytesToCapturePerPacket").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        filters: result.fields.get("filters").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        storage_location: serde_json::from_value(result.fields.get("storageLocation").cloned().unwrap_or_default())?
            ,
        target: serde_json::from_value(result.fields.get("target").cloned().unwrap_or_default())?
            ,
        time_limit_in_seconds: result.fields.get("timeLimitInSeconds").cloned().map(serde_json::from_value).transpose()?,
        total_bytes_per_session: result.fields.get("totalBytesPerSession").cloned().map(serde_json::from_value).transpose()?,
    })
}
