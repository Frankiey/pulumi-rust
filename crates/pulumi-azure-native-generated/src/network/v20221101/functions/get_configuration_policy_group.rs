use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a ConfigurationPolicyGroup.
#[derive(Default)]
pub struct GetConfigurationPolicyGroupArgs {
    /// The name of the ConfigurationPolicyGroup being retrieved.
    pub configuration_policy_group_name: String,
    /// The resource group name of the VpnServerConfiguration.
    pub resource_group_name: String,
    /// The name of the VpnServerConfiguration.
    pub vpn_server_configuration_name: String,
}

/// Result of the function invocation.
pub struct GetConfigurationPolicyGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Shows if this is a Default VpnServerConfigurationPolicyGroup or not.
    pub is_default: Option<bool>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// List of references to P2SConnectionConfigurations.
    pub p2s_connection_configurations: Vec<network::v20221101::SubResourceResponse>,
    /// Multiple PolicyMembers for VpnServerConfigurationPolicyGroup.
    pub policy_members: Option<Vec<network::v20221101::VpnServerConfigurationPolicyGroupMemberResponse>>,
    /// Priority for VpnServerConfigurationPolicyGroup.
    pub priority: Option<i64>,
    /// The provisioning state of the VpnServerConfigurationPolicyGroup resource.
    pub provisioning_state: String,
    /// Resource type.
    pub type_: String,
}

/// Retrieves the details of a ConfigurationPolicyGroup.
pub async fn get_configuration_policy_group(
    ctx: &Context,
    args: GetConfigurationPolicyGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetConfigurationPolicyGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationPolicyGroupName".to_string(), json!(args.configuration_policy_group_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("vpnServerConfigurationName".to_string(), json!(args.vpn_server_configuration_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20221101:getConfigurationPolicyGroup", invoke_args, &opts).await?;

    Ok(GetConfigurationPolicyGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        is_default: result.fields.get("isDefault").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        p2s_connection_configurations: serde_json::from_value(result.fields.get("p2SConnectionConfigurations").cloned().unwrap_or_default())?
            ,
        policy_members: result.fields.get("policyMembers").cloned().map(serde_json::from_value).transpose()?,
        priority: result.fields.get("priority").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
