use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Network security rule.
pub struct SecurityRuleArgs {
    /// The network traffic is allowed or denied.
    pub access: Input<serde_json::Value>,
    /// A description for this rule. Restricted to 140 chars.
    pub description: Option<Input<String>>,
    /// The destination address prefix. CIDR or destination IP range. Asterisk '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used.
    pub destination_address_prefix: Option<Input<String>>,
    /// The destination address prefixes. CIDR or destination IP ranges.
    pub destination_address_prefixes: Option<Vec<Input<String>>>,
    /// The application security group specified as destination.
    pub destination_application_security_groups: Option<Vec<Input<network::v20220501::ApplicationSecurityGroupArgs>>>,
    /// The destination port or range. Integer or range between 0 and 65535. Asterisk '*' can also be used to match all ports.
    pub destination_port_range: Option<Input<String>>,
    /// The destination port ranges.
    pub destination_port_ranges: Option<Vec<Input<String>>>,
    /// The direction of the rule. The direction specifies if rule will be evaluated on incoming or outgoing traffic.
    pub direction: Input<serde_json::Value>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the network security group.
    pub network_security_group_name: Input<String>,
    /// The priority of the rule. The value can be between 100 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
    pub priority: Option<Input<i64>>,
    /// Network protocol this rule applies to.
    pub protocol: Input<serde_json::Value>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the security rule.
    pub security_rule_name: Option<Input<String>>,
    /// The CIDR or source IP range. Asterisk '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used. If this is an ingress rule, specifies where network traffic originates from.
    pub source_address_prefix: Option<Input<String>>,
    /// The CIDR or source IP ranges.
    pub source_address_prefixes: Option<Vec<Input<String>>>,
    /// The application security group specified as source.
    pub source_application_security_groups: Option<Vec<Input<network::v20220501::ApplicationSecurityGroupArgs>>>,
    /// The source port or range. Integer or range between 0 and 65535. Asterisk '*' can also be used to match all ports.
    pub source_port_range: Option<Input<String>>,
    /// The source port ranges.
    pub source_port_ranges: Option<Vec<Input<String>>>,
    /// The type of the resource.
    pub type_: Option<Input<String>>,
}

/// Network security rule.
pub struct SecurityRule {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The network traffic is allowed or denied.
    pub access: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A description for this rule. Restricted to 140 chars.
    pub description: Output<serde_json::Value>,
    /// The destination address prefix. CIDR or destination IP range. Asterisk '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used.
    pub destination_address_prefix: Output<serde_json::Value>,
    /// The destination address prefixes. CIDR or destination IP ranges.
    pub destination_address_prefixes: Output<serde_json::Value>,
    /// The application security group specified as destination.
    pub destination_application_security_groups: Output<serde_json::Value>,
    /// The destination port or range. Integer or range between 0 and 65535. Asterisk '*' can also be used to match all ports.
    pub destination_port_range: Output<serde_json::Value>,
    /// The destination port ranges.
    pub destination_port_ranges: Output<serde_json::Value>,
    /// The direction of the rule. The direction specifies if rule will be evaluated on incoming or outgoing traffic.
    pub direction: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The priority of the rule. The value can be between 100 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
    pub priority: Output<serde_json::Value>,
    /// Network protocol this rule applies to.
    pub protocol: Output<serde_json::Value>,
    /// The provisioning state of the security rule resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The CIDR or source IP range. Asterisk '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used. If this is an ingress rule, specifies where network traffic originates from.
    pub source_address_prefix: Output<serde_json::Value>,
    /// The CIDR or source IP ranges.
    pub source_address_prefixes: Output<serde_json::Value>,
    /// The application security group specified as source.
    pub source_application_security_groups: Output<serde_json::Value>,
    /// The source port or range. Integer or range between 0 and 65535. Asterisk '*' can also be used to match all ports.
    pub source_port_range: Output<serde_json::Value>,
    /// The source port ranges.
    pub source_port_ranges: Output<serde_json::Value>,
    /// The type of the resource.
    pub type_: Output<serde_json::Value>,
}

impl SecurityRule {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220501:SecurityRule";

    /// Create a new SecurityRule resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: SecurityRuleArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("access", args.access, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_address_prefix {
            pulumi_sdk::resolve_input("destinationAddressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_address_prefixes {
            pulumi_sdk::resolve_input_vec("destinationAddressPrefixes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_application_security_groups {
            pulumi_sdk::resolve_input_vec("destinationApplicationSecurityGroups", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_port_range {
            pulumi_sdk::resolve_input("destinationPortRange", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.destination_port_ranges {
            pulumi_sdk::resolve_input_vec("destinationPortRanges", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("direction", args.direction, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkSecurityGroupName", args.network_security_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.priority {
            pulumi_sdk::resolve_input("priority", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("protocol", args.protocol, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.security_rule_name {
            pulumi_sdk::resolve_input("securityRuleName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.source_address_prefix {
            pulumi_sdk::resolve_input("sourceAddressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.source_address_prefixes {
            pulumi_sdk::resolve_input_vec("sourceAddressPrefixes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.source_application_security_groups {
            pulumi_sdk::resolve_input_vec("sourceApplicationSecurityGroups", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.source_port_range {
            pulumi_sdk::resolve_input("sourcePortRange", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.source_port_ranges {
            pulumi_sdk::resolve_input_vec("sourcePortRanges", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.type_ {
            pulumi_sdk::resolve_input("type", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            destination_address_prefix: registered.outputs.get("destinationAddressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_address_prefixes: registered.outputs.get("destinationAddressPrefixes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_application_security_groups: registered.outputs.get("destinationApplicationSecurityGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_port_range: registered.outputs.get("destinationPortRange")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_port_ranges: registered.outputs.get("destinationPortRanges")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            direction: registered.outputs.get("direction")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
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
            source_address_prefix: registered.outputs.get("sourceAddressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source_address_prefixes: registered.outputs.get("sourceAddressPrefixes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source_application_security_groups: registered.outputs.get("sourceApplicationSecurityGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source_port_range: registered.outputs.get("sourcePortRange")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source_port_ranges: registered.outputs.get("sourcePortRanges")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
