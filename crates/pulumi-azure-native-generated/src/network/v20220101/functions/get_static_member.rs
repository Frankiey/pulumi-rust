use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified static member.
#[derive(Default)]
pub struct GetStaticMemberArgs {
    /// The name of the network group.
    pub network_group_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the static member.
    pub static_member_name: String,
}

/// Result of the function invocation.
pub struct GetStaticMemberResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the scope assignment resource.
    pub provisioning_state: String,
    /// Resource region.
    pub region: String,
    /// Resource Id.
    pub resource_id: Option<String>,
    /// The system metadata related to this resource.
    pub system_data: network::v20220101::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified static member.
pub async fn get_static_member(
    ctx: &Context,
    args: GetStaticMemberArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetStaticMemberResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkGroupName".to_string(), json!(args.network_group_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("staticMemberName".to_string(), json!(args.static_member_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220101:getStaticMember", invoke_args, &opts).await?;

    Ok(GetStaticMemberResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        region: serde_json::from_value(result.fields.get("region").cloned().unwrap_or_default())?
            ,
        resource_id: result.fields.get("resourceId").cloned().map(serde_json::from_value).transpose()?,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
