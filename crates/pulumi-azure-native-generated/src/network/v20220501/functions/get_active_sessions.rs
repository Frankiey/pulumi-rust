use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Returns the list of currently active sessions on the Bastion.
#[derive(Default)]
pub struct GetActiveSessionsArgs {
    /// The name of the Bastion Host.
    pub bastion_host_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetActiveSessionsResult {
    /// The URL to get the next set of results.
    pub next_link: Option<String>,
    /// List of active sessions on the bastion.
    pub value: Option<Vec<network::v20220501::BastionActiveSessionResponse>>,
}

/// Returns the list of currently active sessions on the Bastion.
pub async fn get_active_sessions(
    ctx: &Context,
    args: GetActiveSessionsArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetActiveSessionsResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("bastionHostName".to_string(), json!(args.bastion_host_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220501:getActiveSessions", invoke_args, &opts).await?;

    Ok(GetActiveSessionsResult {
        next_link: result.fields.get("nextLink").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
