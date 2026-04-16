use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the current status of IDPS signatures for the relevant policy. Maximal amount of returned signatures is 1000.
#[derive(Default)]
pub struct ListFirewallPolicyIdpsSignatureArgs {
    /// Contain all filters names and values
    pub filters: Option<Vec<network::v20230601::FilterItems>>,
    /// The name of the Firewall Policy.
    pub firewall_policy_name: String,
    /// Column to sort response by
    pub order_by: Option<network::v20230601::OrderBy>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The number of the results to return in each page
    pub results_per_page: Option<i64>,
    /// Search term in all columns
    pub search: Option<String>,
    /// The number of records matching the filter to skip
    pub skip: Option<i64>,
}

/// Result of the function invocation.
pub struct ListFirewallPolicyIdpsSignatureResult {
    /// Number of total records matching the query.
    pub matching_records_count: Option<f64>,
    /// Array containing the results of the query
    pub signatures: Option<Vec<network::v20230601::SingleQueryResultResponse>>,
}

/// Retrieves the current status of IDPS signatures for the relevant policy. Maximal amount of returned signatures is 1000.
pub async fn list_firewall_policy_idps_signature(
    ctx: &Context,
    args: ListFirewallPolicyIdpsSignatureArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListFirewallPolicyIdpsSignatureResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.filters {
        invoke_args.insert("filters".to_string(), json!(v));
    }
    invoke_args.insert("firewallPolicyName".to_string(), json!(args.firewall_policy_name));
    if let Some(v) = args.order_by {
        invoke_args.insert("orderBy".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.results_per_page {
        invoke_args.insert("resultsPerPage".to_string(), json!(v));
    }
    if let Some(v) = args.search {
        invoke_args.insert("search".to_string(), json!(v));
    }
    if let Some(v) = args.skip {
        invoke_args.insert("skip".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230601:listFirewallPolicyIdpsSignature", invoke_args, &opts).await?;

    Ok(ListFirewallPolicyIdpsSignatureResult {
        matching_records_count: result.fields.get("matchingRecordsCount").cloned().map(serde_json::from_value).transpose()?,
        signatures: result.fields.get("signatures").cloned().map(serde_json::from_value).transpose()?,
    })
}
