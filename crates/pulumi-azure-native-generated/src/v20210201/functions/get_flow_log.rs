use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a flow log resource by name.
#[derive(Default)]
pub struct GetFlowLogArgs {
    /// The name of the flow log resource.
    pub flow_log_name: String,
    /// The name of the network watcher.
    pub network_watcher_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetFlowLogResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Flag to enable/disable flow logging.
    pub enabled: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Parameters that define the configuration of traffic analytics.
    pub flow_analytics_configuration: Option<network::v20210201::TrafficAnalyticsPropertiesResponse>,
    /// Parameters that define the flow log format.
    pub format: Option<network::v20210201::FlowLogFormatParametersResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the flow log.
    pub provisioning_state: String,
    /// Parameters that define the retention policy for flow log.
    pub retention_policy: Option<network::v20210201::RetentionPolicyParametersResponse>,
    /// ID of the storage account which is used to store the flow log.
    pub storage_id: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Guid of network security group to which flow log will be applied.
    pub target_resource_guid: String,
    /// ID of network security group to which flow log will be applied.
    pub target_resource_id: String,
    /// Resource type.
    pub type_: String,
}

/// Gets a flow log resource by name.
pub async fn get_flow_log(
    ctx: &Context,
    args: GetFlowLogArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetFlowLogResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("flowLogName".to_string(), json!(args.flow_log_name));
    invoke_args.insert("networkWatcherName".to_string(), json!(args.network_watcher_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20210201:getFlowLog", invoke_args, &opts).await?;

    Ok(GetFlowLogResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        enabled: result.fields.get("enabled").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        flow_analytics_configuration: result.fields.get("flowAnalyticsConfiguration").cloned().map(serde_json::from_value).transpose()?,
        format: result.fields.get("format").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        retention_policy: result.fields.get("retentionPolicy").cloned().map(serde_json::from_value).transpose()?,
        storage_id: serde_json::from_value(result.fields.get("storageId").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        target_resource_guid: serde_json::from_value(result.fields.get("targetResourceGuid").cloned().unwrap_or_default())?
            ,
        target_resource_id: serde_json::from_value(result.fields.get("targetResourceId").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
