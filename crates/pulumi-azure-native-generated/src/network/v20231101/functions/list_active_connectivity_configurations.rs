use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Lists active connectivity configurations in a network manager.
#[derive(Default)]
pub struct ListActiveConnectivityConfigurationsArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// List of regions.
    pub regions: Option<Vec<String>>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
    /// An optional query parameter which specifies the maximum number of records to be returned by the server.
    pub top: Option<i64>,
}

/// Result of the function invocation.
pub struct ListActiveConnectivityConfigurationsResult {
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
    /// Gets a page of active connectivity configurations.
    pub value: Option<Vec<network::v20231101::ActiveConnectivityConfigurationResponse>>,
}

/// Lists active connectivity configurations in a network manager.
pub async fn list_active_connectivity_configurations(
    ctx: &Context,
    args: ListActiveConnectivityConfigurationsArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListActiveConnectivityConfigurationsResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    if let Some(v) = args.regions {
        invoke_args.insert("regions".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.skip_token {
        invoke_args.insert("skipToken".to_string(), json!(v));
    }
    if let Some(v) = args.top {
        invoke_args.insert("top".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20231101:listActiveConnectivityConfigurations", invoke_args, &opts).await?;

    Ok(ListActiveConnectivityConfigurationsResult {
        skip_token: result.fields.get("skipToken").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
