use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// List all effective security admin rules applied on a virtual network.
/// 
/// Uses Azure REST API version 2021-02-01-preview.
#[derive(Default)]
pub struct ListNetworkManagerEffectiveSecurityAdminRuleArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
    /// The name of the virtual network.
    pub virtual_network_name: String,
}

/// Result of the function invocation.
pub struct ListNetworkManagerEffectiveSecurityAdminRuleResult {
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
    /// Gets a page of NetworkManagerEffectiveSecurityAdminRules
    pub value: Option<Vec<serde_json::Value>>,
}

/// List all effective security admin rules applied on a virtual network.
pub async fn list_network_manager_effective_security_admin_rule(
    ctx: &Context,
    args: ListNetworkManagerEffectiveSecurityAdminRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListNetworkManagerEffectiveSecurityAdminRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.skip_token {
        invoke_args.insert("skipToken".to_string(), json!(v));
    }
    invoke_args.insert("virtualNetworkName".to_string(), json!(args.virtual_network_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:listNetworkManagerEffectiveSecurityAdminRule", invoke_args, &opts).await?;

    Ok(ListNetworkManagerEffectiveSecurityAdminRuleResult {
        skip_token: result.fields.get("skipToken").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
