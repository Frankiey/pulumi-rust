use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Post to List of Network Manager Deployment Status.
#[derive(Default)]
pub struct ListNetworkManagerDeploymentStatusArgs {
    /// List of deployment types.
    pub deployment_types: Option<Vec<serde_json::Value>>,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// List of locations.
    pub regions: Option<Vec<String>>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// Continuation token for pagination, capturing the next page size and offset, as well as the context of the query.
    pub skip_token: Option<String>,
    /// An optional query parameter which specifies the maximum number of records to be returned by the server.
    pub top: Option<i64>,
}

/// Result of the function invocation.
pub struct ListNetworkManagerDeploymentStatusResult {
    /// When present, the value can be passed to a subsequent query call (together with the same query and scopes used in the current request) to retrieve the next page of data.
    pub skip_token: Option<String>,
    /// Gets a page of Network Manager Deployment Status
    pub value: Option<Vec<network::v20221101::NetworkManagerDeploymentStatusResponse>>,
}

/// Post to List of Network Manager Deployment Status.
pub async fn list_network_manager_deployment_status(
    ctx: &Context,
    args: ListNetworkManagerDeploymentStatusArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListNetworkManagerDeploymentStatusResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.deployment_types {
        invoke_args.insert("deploymentTypes".to_string(), json!(v));
    }
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
    let result = ctx.invoke("azure-native:network/v20221101:listNetworkManagerDeploymentStatus", invoke_args, &opts).await?;

    Ok(ListNetworkManagerDeploymentStatusResult {
        skip_token: result.fields.get("skipToken").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
