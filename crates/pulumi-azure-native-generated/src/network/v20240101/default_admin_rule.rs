use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Network default admin rule.
pub struct DefaultAdminRuleArgs {
    /// The name of the network manager Security Configuration.
    pub configuration_name: Input<String>,
    /// Default rule flag.
    pub flag: Option<Input<String>>,
    /// Whether the rule is custom or default.
    pub kind: Input<String>,
    /// The name of the network manager.
    pub network_manager_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the network manager security Configuration rule collection.
    pub rule_collection_name: Input<String>,
    /// The name of the rule.
    pub rule_name: Option<Input<String>>,
}

/// Network default admin rule.
pub struct DefaultAdminRule {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Indicates the access allowed for this particular rule
    pub access: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A description for this rule. Restricted to 140 chars.
    pub description: Output<serde_json::Value>,
    /// The destination port ranges.
    pub destination_port_ranges: Output<serde_json::Value>,
    /// The destination address prefixes. CIDR or destination IP ranges.
    pub destinations: Output<serde_json::Value>,
    /// Indicates if the traffic matched against the rule in inbound or outbound.
    pub direction: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Default rule flag.
    pub flag: Output<serde_json::Value>,
    /// Whether the rule is custom or default.
    pub kind: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The priority of the rule. The value can be between 1 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
    pub priority: Output<serde_json::Value>,
    /// Network protocol this rule applies to.
    pub protocol: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Unique identifier for this resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The source port ranges.
    pub source_port_ranges: Output<serde_json::Value>,
    /// The CIDR or source IP ranges.
    pub sources: Output<serde_json::Value>,
    /// The system metadata related to this resource.
    pub system_data: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl DefaultAdminRule {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240101:DefaultAdminRule";

    /// Create a new DefaultAdminRule resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: DefaultAdminRuleArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("configurationName", args.configuration_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.flag {
            pulumi_sdk::resolve_input("flag", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("kind", args.kind, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("networkManagerName", args.network_manager_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("ruleCollectionName", args.rule_collection_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.rule_name {
            pulumi_sdk::resolve_input("ruleName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            access: registered.outputs.get("access")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_port_ranges: registered.outputs.get("destinationPortRanges")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destinations: registered.outputs.get("destinations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            direction: registered.outputs.get("direction")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            flag: registered.outputs.get("flag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            kind: registered.outputs.get("kind")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            priority: registered.outputs.get("priority")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            protocol: registered.outputs.get("protocol")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source_port_ranges: registered.outputs.get("sourcePortRanges")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sources: registered.outputs.get("sources")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
