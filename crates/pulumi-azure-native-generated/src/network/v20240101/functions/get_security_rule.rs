use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get the specified network security rule.
#[derive(Default)]
pub struct GetSecurityRuleArgs {
    /// The name of the network security group.
    pub network_security_group_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the security rule.
    pub security_rule_name: String,
}

/// Result of the function invocation.
pub struct GetSecurityRuleResult {
    /// The network traffic is allowed or denied.
    pub access: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description for this rule. Restricted to 140 chars.
    pub description: Option<String>,
    /// The destination address prefix. CIDR or destination IP range. Asterisk '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used.
    pub destination_address_prefix: Option<String>,
    /// The destination address prefixes. CIDR or destination IP ranges.
    pub destination_address_prefixes: Option<Vec<String>>,
    /// The application security group specified as destination.
    pub destination_application_security_groups: Option<Vec<network::v20240101::ApplicationSecurityGroupResponse>>,
    /// The destination port or range. Integer or range between 0 and 65535. Asterisk '*' can also be used to match all ports.
    pub destination_port_range: Option<String>,
    /// The destination port ranges.
    pub destination_port_ranges: Option<Vec<String>>,
    /// The direction of the rule. The direction specifies if rule will be evaluated on incoming or outgoing traffic.
    pub direction: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The priority of the rule. The value can be between 100 and 4096. The priority number must be unique for each rule in the collection. The lower the priority number, the higher the priority of the rule.
    pub priority: i64,
    /// Network protocol this rule applies to.
    pub protocol: String,
    /// The provisioning state of the security rule resource.
    pub provisioning_state: String,
    /// The CIDR or source IP range. Asterisk '*' can also be used to match all source IPs. Default tags such as 'VirtualNetwork', 'AzureLoadBalancer' and 'Internet' can also be used. If this is an ingress rule, specifies where network traffic originates from.
    pub source_address_prefix: Option<String>,
    /// The CIDR or source IP ranges.
    pub source_address_prefixes: Option<Vec<String>>,
    /// The application security group specified as source.
    pub source_application_security_groups: Option<Vec<network::v20240101::ApplicationSecurityGroupResponse>>,
    /// The source port or range. Integer or range between 0 and 65535. Asterisk '*' can also be used to match all ports.
    pub source_port_range: Option<String>,
    /// The source port ranges.
    pub source_port_ranges: Option<Vec<String>>,
    /// The type of the resource.
    pub type_: Option<String>,
}

/// Get the specified network security rule.
pub async fn get_security_rule(
    ctx: &Context,
    args: GetSecurityRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetSecurityRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkSecurityGroupName".to_string(), json!(args.network_security_group_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("securityRuleName".to_string(), json!(args.security_rule_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240101:getSecurityRule", invoke_args, &opts).await?;

    Ok(GetSecurityRuleResult {
        access: serde_json::from_value(result.fields.get("access").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        destination_address_prefix: result.fields.get("destinationAddressPrefix").cloned().map(serde_json::from_value).transpose()?,
        destination_address_prefixes: result.fields.get("destinationAddressPrefixes").cloned().map(serde_json::from_value).transpose()?,
        destination_application_security_groups: result.fields.get("destinationApplicationSecurityGroups").cloned().map(serde_json::from_value).transpose()?,
        destination_port_range: result.fields.get("destinationPortRange").cloned().map(serde_json::from_value).transpose()?,
        destination_port_ranges: result.fields.get("destinationPortRanges").cloned().map(serde_json::from_value).transpose()?,
        direction: serde_json::from_value(result.fields.get("direction").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        priority: serde_json::from_value(result.fields.get("priority").cloned().unwrap_or_default())?
            ,
        protocol: serde_json::from_value(result.fields.get("protocol").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        source_address_prefix: result.fields.get("sourceAddressPrefix").cloned().map(serde_json::from_value).transpose()?,
        source_address_prefixes: result.fields.get("sourceAddressPrefixes").cloned().map(serde_json::from_value).transpose()?,
        source_application_security_groups: result.fields.get("sourceApplicationSecurityGroups").cloned().map(serde_json::from_value).transpose()?,
        source_port_range: result.fields.get("sourcePortRange").cloned().map(serde_json::from_value).transpose()?,
        source_port_ranges: result.fields.get("sourcePortRanges").cloned().map(serde_json::from_value).transpose()?,
        type_: result.fields.get("type").cloned().map(serde_json::from_value).transpose()?,
    })
}
