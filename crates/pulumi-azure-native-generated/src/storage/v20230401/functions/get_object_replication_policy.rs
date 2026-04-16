use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get the object replication policy of the storage account by policy ID.
#[derive(Default)]
pub struct GetObjectReplicationPolicyArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// For the destination account, provide the value 'default'. Configure the policy on the destination account first. For the source account, provide the value of the policy ID that is returned when you download the policy that was defined on the destination account. The policy is downloaded as a JSON file.
    pub object_replication_policy_id: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetObjectReplicationPolicyResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Required. Destination account name. It should be full resource id if allowCrossTenantReplication set to false.
    pub destination_account: String,
    /// Indicates when the policy is enabled on the source account.
    pub enabled_time: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// A unique id for object replication policy.
    pub policy_id: String,
    /// The storage account object replication rules.
    pub rules: Option<Vec<storage::v20230401::ObjectReplicationPolicyRuleResponse>>,
    /// Required. Source account name. It should be full resource id if allowCrossTenantReplication set to false.
    pub source_account: String,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Get the object replication policy of the storage account by policy ID.
pub async fn get_object_replication_policy(
    ctx: &Context,
    args: GetObjectReplicationPolicyArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetObjectReplicationPolicyResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("objectReplicationPolicyId".to_string(), json!(args.object_replication_policy_id));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20230401:getObjectReplicationPolicy", invoke_args, &opts).await?;

    Ok(GetObjectReplicationPolicyResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        destination_account: serde_json::from_value(result.fields.get("destinationAccount").cloned().unwrap_or_default())?
            ,
        enabled_time: serde_json::from_value(result.fields.get("enabledTime").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        policy_id: serde_json::from_value(result.fields.get("policyId").cloned().unwrap_or_default())?
            ,
        rules: result.fields.get("rules").cloned().map(serde_json::from_value).transpose()?,
        source_account: serde_json::from_value(result.fields.get("sourceAccount").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
