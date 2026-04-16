use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the table with the specified table name, under the specified account if it exists.
#[derive(Default)]
pub struct GetTableArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
    /// A table name must be unique within a storage account and must be between 3 and 63 characters.The name must comprise of only alphanumeric characters and it cannot begin with a numeric character.
    pub table_name: String,
}

/// Result of the function invocation.
pub struct GetTableResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// List of stored access policies specified on the table.
    pub signed_identifiers: Option<Vec<storage::v20230501::TableSignedIdentifierResponse>>,
    /// Table name under the specified account
    pub table_name: String,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the table with the specified table name, under the specified account if it exists.
pub async fn get_table(
    ctx: &Context,
    args: GetTableArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetTableResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("tableName".to_string(), json!(args.table_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20230501:getTable", invoke_args, &opts).await?;

    Ok(GetTableResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        signed_identifiers: result.fields.get("signedIdentifiers").cloned().map(serde_json::from_value).transpose()?,
        table_name: serde_json::from_value(result.fields.get("tableName").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
