use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Return the Bastion Shareable Links for all the VMs specified in the request.
#[derive(Default)]
pub struct GetBastionShareableLinkArgs {
    /// The name of the Bastion Host.
    pub bastion_host_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// List of VM references.
    pub vms: Option<Vec<network::v20250501::BastionShareableLink>>,
}

/// Result of the function invocation.
pub struct GetBastionShareableLinkResult {
    /// The URL to get the next set of results.
    pub next_link: Option<String>,
    /// List of Bastion Shareable Links for the request.
    pub value: Option<Vec<network::v20250501::BastionShareableLinkResponse>>,
}

/// Return the Bastion Shareable Links for all the VMs specified in the request.
pub async fn get_bastion_shareable_link(
    ctx: &Context,
    args: GetBastionShareableLinkArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetBastionShareableLinkResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("bastionHostName".to_string(), json!(args.bastion_host_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.vms {
        invoke_args.insert("vms".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250501:getBastionShareableLink", invoke_args, &opts).await?;

    Ok(GetBastionShareableLinkResult {
        next_link: result.fields.get("nextLink").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
