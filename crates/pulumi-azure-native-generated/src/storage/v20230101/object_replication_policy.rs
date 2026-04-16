use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The replication policy between two storage accounts. Multiple rules can be defined in one policy.
pub struct ObjectReplicationPolicyArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Input<String>,
    /// Required. Destination account name. It should be full resource id if allowCrossTenantReplication set to false.
    pub destination_account: Input<String>,
    /// For the destination account, provide the value 'default'. Configure the policy on the destination account first. For the source account, provide the value of the policy ID that is returned when you download the policy that was defined on the destination account. The policy is downloaded as a JSON file.
    pub object_replication_policy_id: Option<Input<String>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// The storage account object replication rules.
    pub rules: Option<Vec<Input<storage::v20230101::ObjectReplicationPolicyRuleArgs>>>,
    /// Required. Source account name. It should be full resource id if allowCrossTenantReplication set to false.
    pub source_account: Input<String>,
}

/// The replication policy between two storage accounts. Multiple rules can be defined in one policy.
pub struct ObjectReplicationPolicy {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Required. Destination account name. It should be full resource id if allowCrossTenantReplication set to false.
    pub destination_account: Output<serde_json::Value>,
    /// Indicates when the policy is enabled on the source account.
    pub enabled_time: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// A unique id for object replication policy.
    pub policy_id: Output<serde_json::Value>,
    /// The storage account object replication rules.
    pub rules: Output<serde_json::Value>,
    /// Required. Source account name. It should be full resource id if allowCrossTenantReplication set to false.
    pub source_account: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl ObjectReplicationPolicy {
    const TYPE_TOKEN: &'static str = "azure-native:storage/v20230101:ObjectReplicationPolicy";

    /// Create a new ObjectReplicationPolicy resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ObjectReplicationPolicyArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("destinationAccount", args.destination_account, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.object_replication_policy_id {
            pulumi_sdk::resolve_input("objectReplicationPolicyId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.rules {
            pulumi_sdk::resolve_input_vec("rules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("sourceAccount", args.source_account, &mut inputs, &mut deps, &mut prop_deps).await;

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            destination_account: registered.outputs.get("destinationAccount")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enabled_time: registered.outputs.get("enabledTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            policy_id: registered.outputs.get("policyId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            rules: registered.outputs.get("rules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source_account: registered.outputs.get("sourceAccount")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
