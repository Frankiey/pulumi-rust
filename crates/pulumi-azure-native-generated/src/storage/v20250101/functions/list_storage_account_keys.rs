use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Lists the access keys or Kerberos keys (if active directory enabled) for the specified storage account.
#[derive(Default)]
pub struct ListStorageAccountKeysArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// Specifies type of the key to be listed. Possible value is kerb.
    pub expand: Option<String>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct ListStorageAccountKeysResult {
    /// Gets the list of storage account keys and their properties for the specified storage account.
    pub keys: Vec<storage::v20250101::StorageAccountKeyResponse>,
}

/// Lists the access keys or Kerberos keys (if active directory enabled) for the specified storage account.
pub async fn list_storage_account_keys(
    ctx: &Context,
    args: ListStorageAccountKeysArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListStorageAccountKeysResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250101:listStorageAccountKeys", invoke_args, &opts).await?;

    Ok(ListStorageAccountKeysResult {
        keys: serde_json::from_value(result.fields.get("keys").cloned().unwrap_or_default())?
            ,
    })
}
