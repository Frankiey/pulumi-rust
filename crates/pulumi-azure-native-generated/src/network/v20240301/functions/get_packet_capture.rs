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
    pub bytes_to_capture_per_packet: Option<f64>,
    /// The capture setting holds the 'FileCount', 'FileSizeInBytes', 'SessionTimeLimitInSeconds' values.
    pub capture_settings: Option<network::v20240301::PacketCaptureSettingsResponse>,
    /// This continuous capture is a nullable boolean, which can hold 'null', 'true' or 'false' value. If we do not pass this parameter, it would be consider as 'null', default value is 'null'.
    pub continuous_capture: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// A list of packet capture filters.
    pub filters: Option<Vec<network::v20240301::PacketCaptureFilterResponse>>,
    /// ID of the packet capture operation.
    pub id: String,
    /// Name of the packet capture session.
    pub name: String,
    /// The provisioning state of the packet capture session.
    pub provisioning_state: String,
    /// A list of AzureVMSS instances which can be included or excluded to run packet capture. If both included and excluded are empty, then the packet capture will run on all instances of AzureVMSS.
    pub scope: Option<network::v20240301::PacketCaptureMachineScopeResponse>,
    /// The storage location for a packet capture session.
    pub storage_location: network::v20240301::PacketCaptureStorageLocationResponse,
    /// The ID of the targeted resource, only AzureVM and AzureVMSS as target type are currently supported.
    pub target: String,
    /// Target type of the resource provided.
    pub target_type: Option<String>,
    /// Maximum duration of the capture session in seconds.
    pub time_limit_in_seconds: Option<i64>,
    /// Maximum size of the capture output.
    pub total_bytes_per_session: Option<f64>,
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
    let result = ctx.invoke("azure-native:network/v20240301:getPacketCapture", invoke_args, &opts).await?;

    Ok(GetPacketCaptureResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bytes_to_capture_per_packet: result.fields.get("bytesToCapturePerPacket").cloned().map(serde_json::from_value).transpose()?,
        capture_settings: result.fields.get("captureSettings").cloned().map(serde_json::from_value).transpose()?,
        continuous_capture: result.fields.get("continuousCapture").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        filters: result.fields.get("filters").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        scope: result.fields.get("scope").cloned().map(serde_json::from_value).transpose()?,
        storage_location: serde_json::from_value(result.fields.get("storageLocation").cloned().unwrap_or_default())?
            ,
        target: serde_json::from_value(result.fields.get("target").cloned().unwrap_or_default())?
            ,
        target_type: result.fields.get("targetType").cloned().map(serde_json::from_value).transpose()?,
        time_limit_in_seconds: result.fields.get("timeLimitInSeconds").cloned().map(serde_json::from_value).transpose()?,
        total_bytes_per_session: result.fields.get("totalBytesPerSession").cloned().map(serde_json::from_value).transpose()?,
    })
}
