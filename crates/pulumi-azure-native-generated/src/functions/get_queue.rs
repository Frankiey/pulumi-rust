use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the queue with the specified queue name, under the specified account if it exists.
/// 
/// Uses Azure REST API version 2024-01-01.
#[derive(Default)]
pub struct GetQueueArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// A queue name must be unique within a storage account and must be between 3 and 63 characters.The name must comprise of lowercase alphanumeric and dash(-) characters only, it should begin and end with an alphanumeric character and it cannot have two consecutive dash(-) characters.
    pub queue_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetQueueResult {
    /// Integer indicating an approximate number of messages in the queue. This number is not lower than the actual number of messages in the queue, but could be higher.
    pub approximate_message_count: i64,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// A name-value pair that represents queue metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the resource
    pub name: String,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the queue with the specified queue name, under the specified account if it exists.
pub async fn get_queue(
    ctx: &Context,
    args: GetQueueArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetQueueResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("queueName".to_string(), json!(args.queue_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage:getQueue", invoke_args, &opts).await?;

    Ok(GetQueueResult {
        approximate_message_count: serde_json::from_value(result.fields.get("approximateMessageCount").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        metadata: result.fields.get("metadata").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
