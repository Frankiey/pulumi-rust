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
    /// Describes the destination of connection monitor.
    pub destination: network::v20181101::ConnectionMonitorDestinationResponse,
    pub etag: Option<String>,
    /// ID of the connection monitor.
    pub id: String,
    /// Connection monitor location.
    pub location: Option<String>,
    /// Monitoring interval in seconds.
    pub monitoring_interval_in_seconds: Option<i64>,
    /// The monitoring status of the connection monitor.
    pub monitoring_status: Option<String>,
    /// Name of the connection monitor.
    pub name: String,
    /// The provisioning state of the connection monitor.
    pub provisioning_state: Option<String>,
    /// Describes the source of connection monitor.
    pub source: network::v20181101::ConnectionMonitorSourceResponse,
    /// The date and time when the connection monitor was started.
    pub start_time: Option<String>,
    /// Connection monitor tags.
    pub tags: Option<HashMap<String, String>>,
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
    let result = ctx.invoke("azure-native:network/v20181101:getConnectionMonitor", invoke_args, &opts).await?;

    Ok(GetConnectionMonitorResult {
        auto_start: result.fields.get("autoStart").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        destination: serde_json::from_value(result.fields.get("destination").cloned().unwrap_or_default())?
            ,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        monitoring_interval_in_seconds: result.fields.get("monitoringIntervalInSeconds").cloned().map(serde_json::from_value).transpose()?,
        monitoring_status: result.fields.get("monitoringStatus").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        source: serde_json::from_value(result.fields.get("source").cloned().unwrap_or_default())?
            ,
        start_time: result.fields.get("startTime").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
