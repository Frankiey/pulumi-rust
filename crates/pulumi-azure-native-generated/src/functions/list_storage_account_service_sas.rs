use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// List service SAS credentials of a specific resource.
/// 
/// Uses Azure REST API version 2024-01-01.
#[derive(Default)]
pub struct ListStorageAccountServiceSASArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The response header override for cache control.
    pub cache_control: Option<String>,
    /// The canonical path to the signed resource.
    pub canonicalized_resource: String,
    /// The response header override for content disposition.
    pub content_disposition: Option<String>,
    /// The response header override for content encoding.
    pub content_encoding: Option<String>,
    /// The response header override for content language.
    pub content_language: Option<String>,
    /// The response header override for content type.
    pub content_type: Option<String>,
    /// An IP address or a range of IP addresses from which to accept requests.
    pub i_p_address_or_range: Option<String>,
    /// A unique value up to 64 characters in length that correlates to an access policy specified for the container, queue, or table.
    pub identifier: Option<String>,
    /// The key to sign the account SAS token with.
    pub key_to_sign: Option<String>,
    /// The end of partition key.
    pub partition_key_end: Option<String>,
    /// The start of partition key.
    pub partition_key_start: Option<String>,
    /// The signed permissions for the service SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p).
    pub permissions: Option<serde_json::Value>,
    /// The protocol permitted for a request made with the account SAS.
    pub protocols: Option<storage::HttpProtocol>,
    /// The signed services accessible with the service SAS. Possible values include: Blob (b), Container (c), File (f), Share (s).
    pub resource: Option<serde_json::Value>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
    /// The end of row key.
    pub row_key_end: Option<String>,
    /// The start of row key.
    pub row_key_start: Option<String>,
    /// The time at which the shared access signature becomes invalid.
    pub shared_access_expiry_time: Option<String>,
    /// The time at which the SAS becomes valid.
    pub shared_access_start_time: Option<String>,
}

/// Result of the function invocation.
pub struct ListStorageAccountServiceSASResult {
    /// List service SAS credentials of specific resource.
    pub service_sas_token: String,
}

/// List service SAS credentials of a specific resource.
pub async fn list_storage_account_service_sas(
    ctx: &Context,
    args: ListStorageAccountServiceSASArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListStorageAccountServiceSASResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    if let Some(v) = args.cache_control {
        invoke_args.insert("cacheControl".to_string(), json!(v));
    }
    invoke_args.insert("canonicalizedResource".to_string(), json!(args.canonicalized_resource));
    if let Some(v) = args.content_disposition {
        invoke_args.insert("contentDisposition".to_string(), json!(v));
    }
    if let Some(v) = args.content_encoding {
        invoke_args.insert("contentEncoding".to_string(), json!(v));
    }
    if let Some(v) = args.content_language {
        invoke_args.insert("contentLanguage".to_string(), json!(v));
    }
    if let Some(v) = args.content_type {
        invoke_args.insert("contentType".to_string(), json!(v));
    }
    if let Some(v) = args.i_p_address_or_range {
        invoke_args.insert("iPAddressOrRange".to_string(), json!(v));
    }
    if let Some(v) = args.identifier {
        invoke_args.insert("identifier".to_string(), json!(v));
    }
    if let Some(v) = args.key_to_sign {
        invoke_args.insert("keyToSign".to_string(), json!(v));
    }
    if let Some(v) = args.partition_key_end {
        invoke_args.insert("partitionKeyEnd".to_string(), json!(v));
    }
    if let Some(v) = args.partition_key_start {
        invoke_args.insert("partitionKeyStart".to_string(), json!(v));
    }
    if let Some(v) = args.permissions {
        invoke_args.insert("permissions".to_string(), json!(v));
    }
    if let Some(v) = args.protocols {
        invoke_args.insert("protocols".to_string(), json!(v));
    }
    if let Some(v) = args.resource {
        invoke_args.insert("resource".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.row_key_end {
        invoke_args.insert("rowKeyEnd".to_string(), json!(v));
    }
    if let Some(v) = args.row_key_start {
        invoke_args.insert("rowKeyStart".to_string(), json!(v));
    }
    if let Some(v) = args.shared_access_expiry_time {
        invoke_args.insert("sharedAccessExpiryTime".to_string(), json!(v));
    }
    if let Some(v) = args.shared_access_start_time {
        invoke_args.insert("sharedAccessStartTime".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage:listStorageAccountServiceSAS", invoke_args, &opts).await?;

    Ok(ListStorageAccountServiceSASResult {
        service_sas_token: serde_json::from_value(result.fields.get("serviceSasToken").cloned().unwrap_or_default())?
            ,
    })
}
