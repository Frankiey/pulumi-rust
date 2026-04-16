use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Lists all effective virtual networks by specified network group.
/// 
/// Uses Azure REST API version 2021-02-01-preview.
#[derive(Default)]
pub struct ListEffectiveVirtualNetworkByNetworkGroupArgs {
    /// The name of the network group to get.
    pub network_group_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
}

/// Result of the function invocation.
pub struct ListEffectiveVirtualNetworkByNetworkGroupResult {
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
    /// Gets a page of EffectiveVirtualNetwork
    pub value: Option<Vec<network::EffectiveVirtualNetworkResponse>>,
}

/// Lists all effective virtual networks by specified network group.
pub async fn list_effective_virtual_network_by_network_group(
    ctx: &Context,
    args: ListEffectiveVirtualNetworkByNetworkGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListEffectiveVirtualNetworkByNetworkGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkGroupName".to_string(), json!(args.network_group_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.skip_token {
        invoke_args.insert("skipToken".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:listEffectiveVirtualNetworkByNetworkGroup", invoke_args, &opts).await?;

    Ok(ListEffectiveVirtualNetworkByNetworkGroupResult {
        skip_token: result.fields.get("skipToken").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
