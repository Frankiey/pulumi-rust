use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Information about the connection monitor.
pub struct ConnectionMonitorArgs {
    /// Determines if the connection monitor will start automatically once created.
    pub auto_start: Option<Input<bool>>,
    /// The name of the connection monitor.
    pub connection_monitor_name: Option<Input<String>>,
    /// Describes the destination of connection monitor.
    pub destination: Option<Input<network::v20220501::ConnectionMonitorDestinationArgs>>,
    /// List of connection monitor endpoints.
    pub endpoints: Option<Vec<Input<network::v20220501::ConnectionMonitorEndpointArgs>>>,
    /// Connection monitor location.
    pub location: Option<Input<String>>,
    /// Value indicating whether connection monitor V1 should be migrated to V2 format.
    pub migrate: Option<Input<String>>,
    /// Monitoring interval in seconds.
    pub monitoring_interval_in_seconds: Option<Input<i64>>,
    /// The name of the Network Watcher resource.
    pub network_watcher_name: Input<String>,
    /// Optional notes to be associated with the connection monitor.
    pub notes: Option<Input<String>>,
    /// List of connection monitor outputs.
    pub outputs: Option<Vec<Input<network::v20220501::ConnectionMonitorOutputArgs>>>,
    /// The name of the resource group containing Network Watcher.
    pub resource_group_name: Input<String>,
    /// Describes the source of connection monitor.
    pub source: Option<Input<network::v20220501::ConnectionMonitorSourceArgs>>,
    /// Connection monitor tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// List of connection monitor test configurations.
    pub test_configurations: Option<Vec<Input<network::v20220501::ConnectionMonitorTestConfigurationArgs>>>,
    /// List of connection monitor test groups.
    pub test_groups: Option<Vec<Input<network::v20220501::ConnectionMonitorTestGroupArgs>>>,
}

/// Information about the connection monitor.
pub struct ConnectionMonitor {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Determines if the connection monitor will start automatically once created.
    pub auto_start: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Type of connection monitor.
    pub connection_monitor_type: Output<serde_json::Value>,
    /// Describes the destination of connection monitor.
    pub destination: Output<serde_json::Value>,
    /// List of connection monitor endpoints.
    pub endpoints: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Connection monitor location.
    pub location: Output<serde_json::Value>,
    /// Monitoring interval in seconds.
    pub monitoring_interval_in_seconds: Output<serde_json::Value>,
    /// The monitoring status of the connection monitor.
    pub monitoring_status: Output<serde_json::Value>,
    /// Name of the connection monitor.
    pub name: Output<serde_json::Value>,
    /// Optional notes to be associated with the connection monitor.
    pub notes: Output<serde_json::Value>,
    /// List of connection monitor outputs.
    pub outputs: Output<serde_json::Value>,
    /// The provisioning state of the connection monitor.
    pub provisioning_state: Output<serde_json::Value>,
    /// Describes the source of connection monitor.
    pub source: Output<serde_json::Value>,
    /// The date and time when the connection monitor was started.
    pub start_time: Output<serde_json::Value>,
    /// Connection monitor tags.
    pub tags: Output<serde_json::Value>,
    /// List of connection monitor test configurations.
    pub test_configurations: Output<serde_json::Value>,
    /// List of connection monitor test groups.
    pub test_groups: Output<serde_json::Value>,
    /// Connection monitor type.
    pub type_: Output<serde_json::Value>,
}

impl ConnectionMonitor {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220501:ConnectionMonitor";

    /// Create a new ConnectionMonitor resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ConnectionMonitorArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.auto_start {
            pulumi_sdk::resolve_input("autoStart", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.connection_monitor_name {
            pulumi_sdk::resolve_input("connectionMonitorName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination {
            pulumi_sdk::resolve_input("destination", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.endpoints {
            pulumi_sdk::resolve_input_vec("endpoints", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.migrate {
            pulumi_sdk::resolve_input("migrate", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.monitoring_interval_in_seconds {
            pulumi_sdk::resolve_input("monitoringIntervalInSeconds", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkWatcherName", args.network_watcher_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.notes {
            pulumi_sdk::resolve_input("notes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.outputs {
            pulumi_sdk::resolve_input_vec("outputs", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.source {
            pulumi_sdk::resolve_input("source", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.test_configurations {
            pulumi_sdk::resolve_input_vec("testConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.test_groups {
            pulumi_sdk::resolve_input_vec("testGroups", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            auto_start: registered.outputs.get("autoStart")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connection_monitor_type: registered.outputs.get("connectionMonitorType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination: registered.outputs.get("destination")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            endpoints: registered.outputs.get("endpoints")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            monitoring_interval_in_seconds: registered.outputs.get("monitoringIntervalInSeconds")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            monitoring_status: registered.outputs.get("monitoringStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            notes: registered.outputs.get("notes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            outputs: registered.outputs.get("outputs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source: registered.outputs.get("source")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            start_time: registered.outputs.get("startTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            test_configurations: registered.outputs.get("testConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            test_groups: registered.outputs.get("testGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
