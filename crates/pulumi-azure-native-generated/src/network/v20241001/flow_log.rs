use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// A flow log resource.
pub struct FlowLogArgs {
    /// Flag to enable/disable flow logging.
    pub enabled: Option<Input<bool>>,
    /// Optional field to filter network traffic logs based on SrcIP, SrcPort, DstIP, DstPort, Protocol, Encryption, Direction and Action. If not specified, all network traffic will be logged.
    pub enabled_filtering_criteria: Option<Input<String>>,
    /// Parameters that define the configuration of traffic analytics.
    pub flow_analytics_configuration: Option<Input<network::v20241001::TrafficAnalyticsPropertiesArgs>>,
    /// The name of the flow log.
    pub flow_log_name: Option<Input<String>>,
    /// Parameters that define the flow log format.
    pub format: Option<Input<network::v20241001::FlowLogFormatParametersArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// FlowLog resource Managed Identity
    pub identity: Option<Input<network::v20241001::ManagedServiceIdentityArgs>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the network watcher.
    pub network_watcher_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Parameters that define the retention policy for flow log.
    pub retention_policy: Option<Input<network::v20241001::RetentionPolicyParametersArgs>>,
    /// ID of the storage account which is used to store the flow log.
    pub storage_id: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// ID of network security group to which flow log will be applied.
    pub target_resource_id: Input<String>,
}

/// A flow log resource.
pub struct FlowLog {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Flag to enable/disable flow logging.
    pub enabled: Output<serde_json::Value>,
    /// Optional field to filter network traffic logs based on SrcIP, SrcPort, DstIP, DstPort, Protocol, Encryption, Direction and Action. If not specified, all network traffic will be logged.
    pub enabled_filtering_criteria: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Parameters that define the configuration of traffic analytics.
    pub flow_analytics_configuration: Output<serde_json::Value>,
    /// Parameters that define the flow log format.
    pub format: Output<serde_json::Value>,
    /// FlowLog resource Managed Identity
    pub identity: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the flow log.
    pub provisioning_state: Output<serde_json::Value>,
    /// Parameters that define the retention policy for flow log.
    pub retention_policy: Output<serde_json::Value>,
    /// ID of the storage account which is used to store the flow log.
    pub storage_id: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Guid of network security group to which flow log will be applied.
    pub target_resource_guid: Output<serde_json::Value>,
    /// ID of network security group to which flow log will be applied.
    pub target_resource_id: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl FlowLog {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20241001:FlowLog";

    /// Create a new FlowLog resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: FlowLogArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.enabled {
            pulumi_sdk::resolve_input("enabled", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enabled_filtering_criteria {
            pulumi_sdk::resolve_input("enabledFilteringCriteria", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.flow_analytics_configuration {
            pulumi_sdk::resolve_input("flowAnalyticsConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.flow_log_name {
            pulumi_sdk::resolve_input("flowLogName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.format {
            pulumi_sdk::resolve_input("format", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkWatcherName", args.network_watcher_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.retention_policy {
            pulumi_sdk::resolve_input("retentionPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("storageId", args.storage_id, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("targetResourceId", args.target_resource_id, &mut inputs, &mut deps, &mut prop_deps).await;

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
            enabled: registered.outputs.get("enabled")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enabled_filtering_criteria: registered.outputs.get("enabledFilteringCriteria")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            flow_analytics_configuration: registered.outputs.get("flowAnalyticsConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            format: registered.outputs.get("format")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            retention_policy: registered.outputs.get("retentionPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            storage_id: registered.outputs.get("storageId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            target_resource_guid: registered.outputs.get("targetResourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            target_resource_id: registered.outputs.get("targetResourceId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
