use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The ImmutabilityPolicy property of a blob container, including Id, resource name, resource type, Etag.
/// 
/// Uses Azure REST API version 2024-01-01. In version 2.x of the Azure Native provider, it used API version 2022-09-01.
/// 
/// Other available API versions: 2022-09-01, 2023-01-01, 2023-04-01, 2023-05-01, 2025-01-01, 2025-06-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native storage [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct BlobContainerImmutabilityPolicyArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Input<String>,
    /// This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to an append blob while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API.
    pub allow_protected_append_writes: Option<Input<bool>>,
    /// This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API. The 'allowProtectedAppendWrites' and 'allowProtectedAppendWritesAll' properties are mutually exclusive.
    pub allow_protected_append_writes_all: Option<Input<bool>>,
    /// The name of the blob container within the specified storage account. Blob container names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub container_name: Input<String>,
    /// The immutability period for the blobs in the container since the policy creation, in days.
    pub immutability_period_since_creation_in_days: Option<Input<i64>>,
    /// The name of the blob container immutabilityPolicy within the specified storage account. ImmutabilityPolicy Name must be 'default'
    pub immutability_policy_name: Option<Input<String>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
}

/// The ImmutabilityPolicy property of a blob container, including Id, resource name, resource type, Etag.
/// 
/// Uses Azure REST API version 2024-01-01. In version 2.x of the Azure Native provider, it used API version 2022-09-01.
/// 
/// Other available API versions: 2022-09-01, 2023-01-01, 2023-04-01, 2023-05-01, 2025-01-01, 2025-06-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native storage [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct BlobContainerImmutabilityPolicy {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to an append blob while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API.
    pub allow_protected_append_writes: Output<serde_json::Value>,
    /// This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API. The 'allowProtectedAppendWrites' and 'allowProtectedAppendWritesAll' properties are mutually exclusive.
    pub allow_protected_append_writes_all: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Resource Etag.
    pub etag: Output<serde_json::Value>,
    /// The immutability period for the blobs in the container since the policy creation, in days.
    pub immutability_period_since_creation_in_days: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// The ImmutabilityPolicy state of a blob container, possible values include: Locked and Unlocked.
    pub state: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl BlobContainerImmutabilityPolicy {
    const TYPE_TOKEN: &'static str = "azure-native:storage:BlobContainerImmutabilityPolicy";

    /// Create a new BlobContainerImmutabilityPolicy resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: BlobContainerImmutabilityPolicyArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.allow_protected_append_writes {
            pulumi_sdk::resolve_input("allowProtectedAppendWrites", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_protected_append_writes_all {
            pulumi_sdk::resolve_input("allowProtectedAppendWritesAll", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("containerName", args.container_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.immutability_period_since_creation_in_days {
            pulumi_sdk::resolve_input("immutabilityPeriodSinceCreationInDays", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.immutability_policy_name {
            pulumi_sdk::resolve_input("immutabilityPolicyName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            allow_protected_append_writes: registered.outputs.get("allowProtectedAppendWrites")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_protected_append_writes_all: registered.outputs.get("allowProtectedAppendWritesAll")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            immutability_period_since_creation_in_days: registered.outputs.get("immutabilityPeriodSinceCreationInDays")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            state: registered.outputs.get("state")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
