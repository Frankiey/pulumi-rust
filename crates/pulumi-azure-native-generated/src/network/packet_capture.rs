use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Information about packet capture session.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2018-06-01, 2018-07-01, 2018-08-01, 2018-10-01, 2018-11-01, 2018-12-01, 2019-02-01, 2019-04-01, 2019-06-01, 2019-07-01, 2019-08-01, 2019-09-01, 2019-11-01, 2019-12-01, 2020-03-01, 2020-04-01, 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct PacketCaptureArgs {
    /// Number of bytes captured per packet, the remaining bytes are truncated.
    pub bytes_to_capture_per_packet: Option<Input<f64>>,
    /// The capture setting holds the 'FileCount', 'FileSizeInBytes', 'SessionTimeLimitInSeconds' values.
    pub capture_settings: Option<Input<network::PacketCaptureSettingsArgs>>,
    /// This continuous capture is a nullable boolean, which can hold 'null', 'true' or 'false' value. If we do not pass this parameter, it would be consider as 'null', default value is 'null'.
    pub continuous_capture: Option<Input<bool>>,
    /// A list of packet capture filters.
    pub filters: Option<Vec<Input<network::PacketCaptureFilterArgs>>>,
    /// The name of the network watcher.
    pub network_watcher_name: Input<String>,
    /// The name of the packet capture session.
    pub packet_capture_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// A list of AzureVMSS instances which can be included or excluded to run packet capture. If both included and excluded are empty, then the packet capture will run on all instances of AzureVMSS.
    pub scope: Option<Input<network::PacketCaptureMachineScopeArgs>>,
    /// The storage location for a packet capture session.
    pub storage_location: Input<network::PacketCaptureStorageLocationArgs>,
    /// The ID of the targeted resource, only AzureVM and AzureVMSS as target type are currently supported.
    pub target: Input<String>,
    /// Target type of the resource provided.
    pub target_type: Option<Input<network::PacketCaptureTargetTypeArgs>>,
    /// Maximum duration of the capture session in seconds.
    pub time_limit_in_seconds: Option<Input<i64>>,
    /// Maximum size of the capture output.
    pub total_bytes_per_session: Option<Input<f64>>,
}

/// Information about packet capture session.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2018-06-01, 2018-07-01, 2018-08-01, 2018-10-01, 2018-11-01, 2018-12-01, 2019-02-01, 2019-04-01, 2019-06-01, 2019-07-01, 2019-08-01, 2019-09-01, 2019-11-01, 2019-12-01, 2020-03-01, 2020-04-01, 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct PacketCapture {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Number of bytes captured per packet, the remaining bytes are truncated.
    pub bytes_to_capture_per_packet: Output<serde_json::Value>,
    /// The capture setting holds the 'FileCount', 'FileSizeInBytes', 'SessionTimeLimitInSeconds' values.
    pub capture_settings: Output<serde_json::Value>,
    /// This continuous capture is a nullable boolean, which can hold 'null', 'true' or 'false' value. If we do not pass this parameter, it would be consider as 'null', default value is 'null'.
    pub continuous_capture: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// A list of packet capture filters.
    pub filters: Output<serde_json::Value>,
    /// Name of the packet capture session.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the packet capture session.
    pub provisioning_state: Output<serde_json::Value>,
    /// A list of AzureVMSS instances which can be included or excluded to run packet capture. If both included and excluded are empty, then the packet capture will run on all instances of AzureVMSS.
    pub scope: Output<serde_json::Value>,
    /// The storage location for a packet capture session.
    pub storage_location: Output<serde_json::Value>,
    /// The ID of the targeted resource, only AzureVM and AzureVMSS as target type are currently supported.
    pub target: Output<serde_json::Value>,
    /// Target type of the resource provided.
    pub target_type: Output<serde_json::Value>,
    /// Maximum duration of the capture session in seconds.
    pub time_limit_in_seconds: Output<serde_json::Value>,
    /// Maximum size of the capture output.
    pub total_bytes_per_session: Output<serde_json::Value>,
}

impl PacketCapture {
    const TYPE_TOKEN: &'static str = "azure-native:network:PacketCapture";

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
        if let Some(v) = args.capture_settings {
            pulumi_sdk::resolve_input("captureSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.continuous_capture {
            pulumi_sdk::resolve_input("continuousCapture", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.filters {
            pulumi_sdk::resolve_input_vec("filters", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkWatcherName", args.network_watcher_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.packet_capture_name {
            pulumi_sdk::resolve_input("packetCaptureName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.scope {
            pulumi_sdk::resolve_input("scope", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("storageLocation", args.storage_location, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("target", args.target, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.target_type {
            pulumi_sdk::resolve_input("targetType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
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
            capture_settings: registered.outputs.get("captureSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            continuous_capture: registered.outputs.get("continuousCapture")
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
            scope: registered.outputs.get("scope")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            storage_location: registered.outputs.get("storageLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            target: registered.outputs.get("target")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            target_type: registered.outputs.get("targetType")
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
