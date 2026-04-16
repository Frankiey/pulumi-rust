use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a connection monitor by name.
#[derive(Default)]
pub struct GetConnectionMonitorArgs {
    /// The name of the connection monitor.
    pub connection_monitor_name: String,
    /// The name of the Network Watcher resource.
    pub network_watcher_name: String,
    /// The name of the resource group containing Network Watcher.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetConnectionMonitorResult {
    /// Determines if the connection monitor will start automatically once created.
    pub auto_start: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Type of connection monitor.
    pub connection_monitor_type: String,
    /// Describes the destination of connection monitor.
    pub destination: Option<network::v20191101::ConnectionMonitorDestinationResponse>,
    /// List of connection monitor endpoints.
    pub endpoints: Option<Vec<network::v20191101::ConnectionMonitorEndpointResponse>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// ID of the connection monitor.
    pub id: String,
    /// Connection monitor location.
    pub location: Option<String>,
    /// Monitoring interval in seconds.
    pub monitoring_interval_in_seconds: Option<i64>,
    /// The monitoring status of the connection monitor.
    pub monitoring_status: String,
    /// Name of the connection monitor.
    pub name: String,
    /// Optional notes to be associated with the connection monitor.
    pub notes: Option<String>,
    /// List of connection monitor outputs.
    pub outputs: Option<Vec<network::v20191101::ConnectionMonitorOutputResponse>>,
    /// The provisioning state of the connection monitor.
    pub provisioning_state: String,
    /// Describes the source of connection monitor.
    pub source: Option<network::v20191101::ConnectionMonitorSourceResponse>,
    /// The date and time when the connection monitor was started.
    pub start_time: String,
    /// Connection monitor tags.
    pub tags: Option<HashMap<String, String>>,
    /// List of connection monitor test configurations.
    pub test_configurations: Option<Vec<network::v20191101::ConnectionMonitorTestConfigurationResponse>>,
    /// List of connection monitor test groups.
    pub test_groups: Option<Vec<network::v20191101::ConnectionMonitorTestGroupResponse>>,
    /// Connection monitor type.
    pub type_: String,
}

/// Gets a connection monitor by name.
pub async fn get_connection_monitor(
    ctx: &Context,
    args: GetConnectionMonitorArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetConnectionMonitorResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionMonitorName".to_string(), json!(args.connection_monitor_name));
    invoke_args.insert("networkWatcherName".to_string(), json!(args.network_watcher_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20191101:getConnectionMonitor", invoke_args, &opts).await?;

    Ok(GetConnectionMonitorResult {
        auto_start: result.fields.get("autoStart").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        connection_monitor_type: serde_json::from_value(result.fields.get("connectionMonitorType").cloned().unwrap_or_default())?
            ,
        destination: result.fields.get("destination").cloned().map(serde_json::from_value).transpose()?,
        endpoints: result.fields.get("endpoints").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        monitoring_interval_in_seconds: result.fields.get("monitoringIntervalInSeconds").cloned().map(serde_json::from_value).transpose()?,
        monitoring_status: serde_json::from_value(result.fields.get("monitoringStatus").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        notes: result.fields.get("notes").cloned().map(serde_json::from_value).transpose()?,
        outputs: result.fields.get("outputs").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        source: result.fields.get("source").cloned().map(serde_json::from_value).transpose()?,
        start_time: serde_json::from_value(result.fields.get("startTime").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        test_configurations: result.fields.get("testConfigurations").cloned().map(serde_json::from_value).transpose()?,
        test_groups: result.fields.get("testGroups").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
