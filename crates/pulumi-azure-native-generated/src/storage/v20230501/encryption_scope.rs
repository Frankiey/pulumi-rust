use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The Encryption Scope resource.
pub struct EncryptionScopeArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Input<String>,
    /// The name of the encryption scope within the specified storage account. Encryption scope names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub encryption_scope_name: Option<Input<String>>,
    /// The key vault properties for the encryption scope. This is a required field if encryption scope 'source' attribute is set to 'Microsoft.KeyVault'.
    pub key_vault_properties: Option<Input<storage::v20230501::EncryptionScopeKeyVaultPropertiesArgs>>,
    /// A boolean indicating whether or not the service applies a secondary layer of encryption with platform managed keys for data at rest.
    pub require_infrastructure_encryption: Option<Input<bool>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// The provider for the encryption scope. Possible values (case-insensitive):  Microsoft.Storage, Microsoft.KeyVault.
    pub source: Option<Input<serde_json::Value>>,
    /// The state of the encryption scope. Possible values (case-insensitive):  Enabled, Disabled.
    pub state: Option<Input<serde_json::Value>>,
}

/// The Encryption Scope resource.
pub struct EncryptionScope {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Gets the creation date and time of the encryption scope in UTC.
    pub creation_time: Output<serde_json::Value>,
    /// The key vault properties for the encryption scope. This is a required field if encryption scope 'source' attribute is set to 'Microsoft.KeyVault'.
    pub key_vault_properties: Output<serde_json::Value>,
    /// Gets the last modification date and time of the encryption scope in UTC.
    pub last_modified_time: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// A boolean indicating whether or not the service applies a secondary layer of encryption with platform managed keys for data at rest.
    pub require_infrastructure_encryption: Output<serde_json::Value>,
    /// The provider for the encryption scope. Possible values (case-insensitive):  Microsoft.Storage, Microsoft.KeyVault.
    pub source: Output<serde_json::Value>,
    /// The state of the encryption scope. Possible values (case-insensitive):  Enabled, Disabled.
    pub state: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl EncryptionScope {
    const TYPE_TOKEN: &'static str = "azure-native:storage/v20230501:EncryptionScope";

    /// Create a new EncryptionScope resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: EncryptionScopeArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.encryption_scope_name {
            pulumi_sdk::resolve_input("encryptionScopeName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.key_vault_properties {
            pulumi_sdk::resolve_input("keyVaultProperties", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.require_infrastructure_encryption {
            pulumi_sdk::resolve_input("requireInfrastructureEncryption", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.source {
            pulumi_sdk::resolve_input("source", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.state {
            pulumi_sdk::resolve_input("state", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

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
            creation_time: registered.outputs.get("creationTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            key_vault_properties: registered.outputs.get("keyVaultProperties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            last_modified_time: registered.outputs.get("lastModifiedTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            require_infrastructure_encryption: registered.outputs.get("requireInfrastructureEncryption")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            source: registered.outputs.get("source")
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
