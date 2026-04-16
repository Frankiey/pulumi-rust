use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Lists Active Security User Rules in a network manager.
#[derive(Default)]
pub struct ListActiveSecurityUserRulesArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// List of regions.
    pub regions: Option<Vec<String>>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
}

/// Result of the function invocation.
pub struct ListActiveSecurityUserRulesResult {
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
    /// Gets a page of active security user rules.
    pub value: Option<Vec<serde_json::Value>>,
}

/// Lists Active Security User Rules in a network manager.
pub async fn list_active_security_user_rules(
    ctx: &Context,
    args: ListActiveSecurityUserRulesArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListActiveSecurityUserRulesResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    if let Some(v) = args.regions {
        invoke_args.insert("regions".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.skip_token {
        invoke_args.insert("skipToken".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220401preview:listActiveSecurityUserRules", invoke_args, &opts).await?;

    Ok(ListActiveSecurityUserRulesResult {
        skip_token: result.fields.get("skipToken").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
