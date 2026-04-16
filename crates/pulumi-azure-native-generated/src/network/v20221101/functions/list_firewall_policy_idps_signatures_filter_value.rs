use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the current filter values for the signatures overrides
#[derive(Default)]
pub struct ListFirewallPolicyIdpsSignaturesFilterValueArgs {
    /// Describes the name of the column which values will be returned
    pub filter_name: Option<String>,
    /// The name of the Firewall Policy.
    pub firewall_policy_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct ListFirewallPolicyIdpsSignaturesFilterValueResult {
    /// Describes the possible values
    pub filter_values: Option<Vec<String>>,
}

/// Retrieves the current filter values for the signatures overrides
pub async fn list_firewall_policy_idps_signatures_filter_value(
    ctx: &Context,
    args: ListFirewallPolicyIdpsSignaturesFilterValueArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListFirewallPolicyIdpsSignaturesFilterValueResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.filter_name {
        invoke_args.insert("filterName".to_string(), json!(v));
    }
    invoke_args.insert("firewallPolicyName".to_string(), json!(args.firewall_policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20221101:listFirewallPolicyIdpsSignaturesFilterValue", invoke_args, &opts).await?;

    Ok(ListFirewallPolicyIdpsSignaturesFilterValueResult {
        filter_values: result.fields.get("filterValues").cloned().map(serde_json::from_value).transpose()?,
    })
}
