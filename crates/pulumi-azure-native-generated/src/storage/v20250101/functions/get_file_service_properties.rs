use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the properties of file services in storage accounts, including CORS (Cross-Origin Resource Sharing) rules.
#[derive(Default)]
pub struct GetFileServicePropertiesArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the file Service within the specified storage account. File Service Name must be "default"
    pub file_services_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetFileServicePropertiesResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Specifies CORS rules for the File service. You can include up to five CorsRule elements in the request. If no CorsRule elements are included in the request body, all CORS rules will be deleted, and CORS will be disabled for the File service.
    pub cors: Option<storage::v20250101::CorsRulesResponse>,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// Protocol settings for file service
    pub protocol_settings: Option<storage::v20250101::ProtocolSettingsResponse>,
    /// The file service properties for share soft delete.
    pub share_delete_retention_policy: Option<storage::v20250101::DeleteRetentionPolicyResponse>,
    /// Sku name and tier.
    pub sku: storage::v20250101::SkuResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the properties of file services in storage accounts, including CORS (Cross-Origin Resource Sharing) rules.
pub async fn get_file_service_properties(
    ctx: &Context,
    args: GetFileServicePropertiesArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetFileServicePropertiesResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("fileServicesName".to_string(), json!(args.file_services_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250101:getFileServiceProperties", invoke_args, &opts).await?;

    Ok(GetFileServicePropertiesResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        cors: result.fields.get("cors").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        protocol_settings: result.fields.get("protocolSettings").cloned().map(serde_json::from_value).transpose()?,
        share_delete_retention_policy: result.fields.get("shareDeleteRetentionPolicy").cloned().map(serde_json::from_value).transpose()?,
        sku: serde_json::from_value(result.fields.get("sku").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
