use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Returns the properties for the specified encryption scope.
#[derive(Default)]
pub struct GetEncryptionScopeArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the encryption scope within the specified storage account. Encryption scope names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub encryption_scope_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetEncryptionScopeResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Gets the creation date and time of the encryption scope in UTC.
    pub creation_time: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The key vault properties for the encryption scope. This is a required field if encryption scope 'source' attribute is set to 'Microsoft.KeyVault'.
    pub key_vault_properties: Option<storage::v20250101::EncryptionScopeKeyVaultPropertiesResponse>,
    /// Gets the last modification date and time of the encryption scope in UTC.
    pub last_modified_time: String,
    /// The name of the resource
    pub name: String,
    /// A boolean indicating whether or not the service applies a secondary layer of encryption with platform managed keys for data at rest.
    pub require_infrastructure_encryption: Option<bool>,
    /// The provider for the encryption scope. Possible values (case-insensitive):  Microsoft.Storage, Microsoft.KeyVault.
    pub source: Option<String>,
    /// The state of the encryption scope. Possible values (case-insensitive):  Enabled, Disabled.
    pub state: Option<String>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Returns the properties for the specified encryption scope.
pub async fn get_encryption_scope(
    ctx: &Context,
    args: GetEncryptionScopeArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetEncryptionScopeResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("encryptionScopeName".to_string(), json!(args.encryption_scope_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250101:getEncryptionScope", invoke_args, &opts).await?;

    Ok(GetEncryptionScopeResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        creation_time: serde_json::from_value(result.fields.get("creationTime").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        key_vault_properties: result.fields.get("keyVaultProperties").cloned().map(serde_json::from_value).transpose()?,
        last_modified_time: serde_json::from_value(result.fields.get("lastModifiedTime").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        require_infrastructure_encryption: result.fields.get("requireInfrastructureEncryption").cloned().map(serde_json::from_value).transpose()?,
        source: result.fields.get("source").cloned().map(serde_json::from_value).transpose()?,
        state: result.fields.get("state").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
