use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// List of PoolAssociation
#[derive(Default)]
pub struct ListIpamPoolAssociatedResourcesArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// Pool resource name.
    pub pool_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct ListIpamPoolAssociatedResourcesResult {
    /// The link used to get the next page of operations.
    pub next_link: Option<String>,
    pub value: Option<Vec<network::v20240101preview::PoolAssociationResponse>>,
}

/// List of PoolAssociation
pub async fn list_ipam_pool_associated_resources(
    ctx: &Context,
    args: ListIpamPoolAssociatedResourcesArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListIpamPoolAssociatedResourcesResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("poolName".to_string(), json!(args.pool_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240101preview:listIpamPoolAssociatedResources", invoke_args, &opts).await?;

    Ok(ListIpamPoolAssociatedResourcesResult {
        next_link: result.fields.get("nextLink").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
