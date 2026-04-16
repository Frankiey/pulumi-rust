use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// List SAS credentials of a storage account.
#[derive(Default)]
pub struct ListStorageAccountSASArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// An IP address or a range of IP addresses from which to accept requests.
    pub i_p_address_or_range: Option<String>,
    /// The key to sign the account SAS token with.
    pub key_to_sign: Option<String>,
    /// The signed permissions for the account SAS. Possible values include: Read (r), Write (w), Delete (d), List (l), Add (a), Create (c), Update (u) and Process (p).
    pub permissions: serde_json::Value,
    /// The protocol permitted for a request made with the account SAS.
    pub protocols: Option<storage::v20230401::HttpProtocol>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
    /// The signed resource types that are accessible with the account SAS. Service (s): Access to service-level APIs; Container (c): Access to container-level APIs; Object (o): Access to object-level APIs for blobs, queue messages, table entities, and files.
    pub resource_types: serde_json::Value,
    /// The signed services accessible with the account SAS. Possible values include: Blob (b), Queue (q), Table (t), File (f).
    pub services: serde_json::Value,
    /// The time at which the shared access signature becomes invalid.
    pub shared_access_expiry_time: String,
    /// The time at which the SAS becomes valid.
    pub shared_access_start_time: Option<String>,
}

/// Result of the function invocation.
pub struct ListStorageAccountSASResult {
    /// List SAS credentials of storage account.
    pub account_sas_token: String,
}

/// List SAS credentials of a storage account.
pub async fn list_storage_account_sas(
    ctx: &Context,
    args: ListStorageAccountSASArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListStorageAccountSASResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    if let Some(v) = args.i_p_address_or_range {
        invoke_args.insert("iPAddressOrRange".to_string(), json!(v));
    }
    if let Some(v) = args.key_to_sign {
        invoke_args.insert("keyToSign".to_string(), json!(v));
    }
    invoke_args.insert("permissions".to_string(), json!(args.permissions));
    if let Some(v) = args.protocols {
        invoke_args.insert("protocols".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("resourceTypes".to_string(), json!(args.resource_types));
    invoke_args.insert("services".to_string(), json!(args.services));
    invoke_args.insert("sharedAccessExpiryTime".to_string(), json!(args.shared_access_expiry_time));
    if let Some(v) = args.shared_access_start_time {
        invoke_args.insert("sharedAccessStartTime".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20230401:listStorageAccountSAS", invoke_args, &opts).await?;

    Ok(ListStorageAccountSASResult {
        account_sas_token: serde_json::from_value(result.fields.get("accountSasToken").cloned().unwrap_or_default())?
            ,
    })
}
