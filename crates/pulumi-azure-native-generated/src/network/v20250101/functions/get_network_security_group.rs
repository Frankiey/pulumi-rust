use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified network security group.
#[derive(Default)]
pub struct GetNetworkSecurityGroupArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the network security group.
    pub network_security_group_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkSecurityGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The default security rules of network security group.
    pub default_security_rules: Vec<network::v20250101::SecurityRuleResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// A collection of references to flow log resources.
    pub flow_logs: Vec<network::v20250101::FlowLogResponse>,
    /// When enabled, flows created from Network Security Group connections will be re-evaluated when rules are updates. Initial enablement will trigger re-evaluation.
    pub flush_connection: Option<bool>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// A collection of references to network interfaces.
    pub network_interfaces: Vec<network::v20250101::NetworkInterfaceResponse>,
    /// The provisioning state of the network security group resource.
    pub provisioning_state: String,
    /// The resource GUID property of the network security group resource.
    pub resource_guid: String,
    /// A collection of security rules of the network security group.
    pub security_rules: Option<Vec<network::v20250101::SecurityRuleResponse>>,
    /// A collection of references to subnets.
    pub subnets: Vec<network::v20250101::SubnetResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified network security group.
pub async fn get_network_security_group(
    ctx: &Context,
    args: GetNetworkSecurityGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkSecurityGroupResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("networkSecurityGroupName".to_string(), json!(args.network_security_group_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250101:getNetworkSecurityGroup", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        default_security_rules: serde_json::from_value(result.fields.get("defaultSecurityRules").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        flow_logs: serde_json::from_value(result.fields.get("flowLogs").cloned().unwrap_or_default())?
            ,
        flush_connection: result.fields.get("flushConnection").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_interfaces: serde_json::from_value(result.fields.get("networkInterfaces").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        security_rules: result.fields.get("securityRules").cloned().map(serde_json::from_value).transpose()?,
        subnets: serde_json::from_value(result.fields.get("subnets").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
