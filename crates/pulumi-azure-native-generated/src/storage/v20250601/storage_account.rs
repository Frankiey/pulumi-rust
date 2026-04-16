use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The storage account.
pub struct StorageAccountArgs {
    /// Required for storage accounts where kind = BlobStorage. The access tier is used for billing. The 'Premium' access tier is the default value for premium block blobs storage account type and it cannot be changed for the premium block blobs storage account type.
    pub access_tier: Option<Input<storage::v20250601::AccessTierArgs>>,
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Option<Input<String>>,
    /// Allow or disallow public access to all blobs or containers in the storage account. The default interpretation is false for this property.
    pub allow_blob_public_access: Option<Input<bool>>,
    /// Allow or disallow cross AAD tenant object replication. Set this property to true for new or existing accounts only if object replication policies will involve storage accounts in different AAD tenants. The default interpretation is false for new accounts to follow best security practices by default.
    pub allow_cross_tenant_replication: Option<Input<bool>>,
    /// Indicates whether the storage account permits requests to be authorized with the account access key via Shared Key. If false, then all requests, including shared access signatures, must be authorized with Azure Active Directory (Azure AD). The default value is null, which is equivalent to true.
    pub allow_shared_key_access: Option<Input<bool>>,
    /// Restrict copy to and from Storage Accounts within an AAD tenant or with Private Links to the same VNet.
    pub allowed_copy_scope: Option<Input<serde_json::Value>>,
    /// Provides the identity based authentication settings for Azure Files.
    pub azure_files_identity_based_authentication: Option<Input<storage::v20250601::AzureFilesIdentityBasedAuthenticationArgs>>,
    /// User domain assigned to the storage account. Name is the CNAME source. Only one custom domain is supported per storage account at this time. To clear the existing custom domain, use an empty string for the custom domain name property.
    pub custom_domain: Option<Input<storage::v20250601::CustomDomainArgs>>,
    /// A boolean flag which indicates whether the default authentication is OAuth or not. The default interpretation is false for this property.
    pub default_to_o_auth_authentication: Option<Input<bool>>,
    /// Allows you to specify the type of endpoint. Set this to AzureDNSZone to create a large number of accounts in a single subscription, which creates accounts in an Azure DNS Zone and the endpoint URL will have an alphanumeric DNS Zone identifier.
    pub dns_endpoint_type: Option<Input<serde_json::Value>>,
    /// Maintains information about the Internet protocol opted by the user.
    pub dual_stack_endpoint_preference: Option<Input<storage::v20250601::DualStackEndpointPreferenceArgs>>,
    /// Enables extended group support with local users feature, if set to true
    pub enable_extended_groups: Option<Input<bool>>,
    /// Allows https traffic only to storage service if sets to true. The default value is true since API version 2019-04-01.
    pub enable_https_traffic_only: Option<Input<bool>>,
    /// NFS 3.0 protocol support enabled if set to true.
    pub enable_nfs_v3: Option<Input<bool>>,
    /// Encryption settings to be used for server-side encryption for the storage account.
    pub encryption: Option<Input<storage::v20250601::EncryptionArgs>>,
    /// Optional. Set the extended location of the resource. If not set, the storage account will be created in Azure main region. Otherwise it will be created in the specified extended location
    pub extended_location: Option<Input<storage::v20250601::ExtendedLocationArgs>>,
    /// Status indicating whether Geo Priority Replication is enabled for the account.
    pub geo_priority_replication_status: Option<Input<storage::v20250601::GeoPriorityReplicationStatusArgs>>,
    /// The identity of the resource.
    pub identity: Option<Input<storage::v20250601::IdentityArgs>>,
    /// The property is immutable and can only be set to true at the account creation time. When set to true, it enables object level immutability for all the new containers in the account by default.
    pub immutable_storage_with_versioning: Option<Input<storage::v20250601::ImmutableStorageAccountArgs>>,
    /// Account HierarchicalNamespace enabled if sets to true.
    pub is_hns_enabled: Option<Input<bool>>,
    /// Enables local users feature, if set to true
    pub is_local_user_enabled: Option<Input<bool>>,
    /// Enables Secure File Transfer Protocol, if set to true
    pub is_sftp_enabled: Option<Input<bool>>,
    /// KeyPolicy assigned to the storage account.
    pub key_policy: Option<Input<storage::v20250601::KeyPolicyArgs>>,
    /// Required. Indicates the type of storage account.
    pub kind: Input<serde_json::Value>,
    /// Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled.
    pub large_file_shares_state: Option<Input<serde_json::Value>>,
    /// Required. Gets or sets the location of the resource. This will be one of the supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.). The geo region of a resource cannot be changed once it is created, but if an identical geo region is specified on update, the request will succeed.
    pub location: Option<Input<String>>,
    /// Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property.
    pub minimum_tls_version: Option<Input<serde_json::Value>>,
    /// Network rule set
    pub network_rule_set: Option<Input<storage::v20250601::NetworkRuleSetArgs>>,
    /// Optional. Gets or sets the zonal placement details for the storage account.
    pub placement: Option<Input<storage::v20250601::PlacementArgs>>,
    /// Allow, disallow, or let Network Security Perimeter configuration to evaluate public network access to Storage Account. Value is optional but if passed in, must be 'Enabled', 'Disabled' or 'SecuredByPerimeter'.
    pub public_network_access: Option<Input<serde_json::Value>>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// Maintains information about the network routing choice opted by the user for data transfer
    pub routing_preference: Option<Input<storage::v20250601::RoutingPreferenceArgs>>,
    /// SasPolicy assigned to the storage account.
    pub sas_policy: Option<Input<storage::v20250601::SasPolicyArgs>>,
    /// Required. Gets or sets the SKU name.
    pub sku: Input<storage::v20250601::SkuArgs>,
    /// Gets or sets a list of key value pairs that describe the resource. These tags can be used for viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key with a length no greater than 128 characters and a value with a length no greater than 256 characters.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Optional. Gets or sets the pinned logical availability zone for the storage account.
    pub zones: Option<Vec<Input<String>>>,
}

/// The storage account.
pub struct StorageAccount {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Required for storage accounts where kind = BlobStorage. The access tier is used for billing. The 'Premium' access tier is the default value for premium block blobs storage account type and it cannot be changed for the premium block blobs storage account type.
    pub access_tier: Output<serde_json::Value>,
    /// If customer initiated account migration is in progress, the value will be true else it will be null.
    pub account_migration_in_progress: Output<serde_json::Value>,
    /// Allow or disallow public access to all blobs or containers in the storage account. The default interpretation is false for this property.
    pub allow_blob_public_access: Output<serde_json::Value>,
    /// Allow or disallow cross AAD tenant object replication. Set this property to true for new or existing accounts only if object replication policies will involve storage accounts in different AAD tenants. The default interpretation is false for new accounts to follow best security practices by default.
    pub allow_cross_tenant_replication: Output<serde_json::Value>,
    /// Indicates whether the storage account permits requests to be authorized with the account access key via Shared Key. If false, then all requests, including shared access signatures, must be authorized with Azure Active Directory (Azure AD). The default value is null, which is equivalent to true.
    pub allow_shared_key_access: Output<serde_json::Value>,
    /// Restrict copy to and from Storage Accounts within an AAD tenant or with Private Links to the same VNet.
    pub allowed_copy_scope: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Provides the identity based authentication settings for Azure Files.
    pub azure_files_identity_based_authentication: Output<serde_json::Value>,
    /// Blob restore status
    pub blob_restore_status: Output<serde_json::Value>,
    /// Gets the creation date and time of the storage account in UTC.
    pub creation_time: Output<serde_json::Value>,
    /// Gets the custom domain the user assigned to this storage account.
    pub custom_domain: Output<serde_json::Value>,
    /// A boolean flag which indicates whether the default authentication is OAuth or not. The default interpretation is false for this property.
    pub default_to_o_auth_authentication: Output<serde_json::Value>,
    /// Allows you to specify the type of endpoint. Set this to AzureDNSZone to create a large number of accounts in a single subscription, which creates accounts in an Azure DNS Zone and the endpoint URL will have an alphanumeric DNS Zone identifier.
    pub dns_endpoint_type: Output<serde_json::Value>,
    /// Maintains information about the Internet protocol opted by the user.
    pub dual_stack_endpoint_preference: Output<serde_json::Value>,
    /// Enables extended group support with local users feature, if set to true
    pub enable_extended_groups: Output<serde_json::Value>,
    /// Allows https traffic only to storage service if sets to true.
    pub enable_https_traffic_only: Output<serde_json::Value>,
    /// NFS 3.0 protocol support enabled if set to true.
    pub enable_nfs_v3: Output<serde_json::Value>,
    /// Encryption settings to be used for server-side encryption for the storage account.
    pub encryption: Output<serde_json::Value>,
    /// The extendedLocation of the resource.
    pub extended_location: Output<serde_json::Value>,
    /// If the failover is in progress, the value will be true, otherwise, it will be null.
    pub failover_in_progress: Output<serde_json::Value>,
    /// Status indicating whether Geo Priority Replication is enabled for the account.
    pub geo_priority_replication_status: Output<serde_json::Value>,
    /// Geo Replication Stats
    pub geo_replication_stats: Output<serde_json::Value>,
    /// The identity of the resource.
    pub identity: Output<serde_json::Value>,
    /// The property is immutable and can only be set to true at the account creation time. When set to true, it enables object level immutability for all the containers in the account by default.
    pub immutable_storage_with_versioning: Output<serde_json::Value>,
    /// Account HierarchicalNamespace enabled if sets to true.
    pub is_hns_enabled: Output<serde_json::Value>,
    /// Enables local users feature, if set to true
    pub is_local_user_enabled: Output<serde_json::Value>,
    /// Enables Secure File Transfer Protocol, if set to true
    pub is_sftp_enabled: Output<serde_json::Value>,
    /// This property will be set to true or false on an event of ongoing migration. Default value is null.
    pub is_sku_conversion_blocked: Output<serde_json::Value>,
    /// Storage account keys creation time.
    pub key_creation_time: Output<serde_json::Value>,
    /// KeyPolicy assigned to the storage account.
    pub key_policy: Output<serde_json::Value>,
    /// Gets the Kind.
    pub kind: Output<serde_json::Value>,
    /// Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled.
    pub large_file_shares_state: Output<serde_json::Value>,
    /// Gets the timestamp of the most recent instance of a failover to the secondary location. Only the most recent timestamp is retained. This element is not returned if there has never been a failover instance. Only available if the accountType is Standard_GRS or Standard_RAGRS.
    pub last_geo_failover_time: Output<serde_json::Value>,
    /// The geo-location where the resource lives
    pub location: Output<serde_json::Value>,
    /// Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property.
    pub minimum_tls_version: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// Network rule set
    pub network_rule_set: Output<serde_json::Value>,
    /// Optional. Gets or sets the zonal placement details for the storage account.
    pub placement: Output<serde_json::Value>,
    /// Gets the URLs that are used to perform a retrieval of a public blob, queue, or table object. Note that Standard_ZRS and Premium_LRS accounts only return the blob endpoint.
    pub primary_endpoints: Output<serde_json::Value>,
    /// Gets the location of the primary data center for the storage account.
    pub primary_location: Output<serde_json::Value>,
    /// List of private endpoint connection associated with the specified storage account
    pub private_endpoint_connections: Output<serde_json::Value>,
    /// Gets the status of the storage account at the time the operation was called.
    pub provisioning_state: Output<serde_json::Value>,
    /// Allow, disallow, or let Network Security Perimeter configuration to evaluate public network access to Storage Account.
    pub public_network_access: Output<serde_json::Value>,
    /// Maintains information about the network routing choice opted by the user for data transfer
    pub routing_preference: Output<serde_json::Value>,
    /// SasPolicy assigned to the storage account.
    pub sas_policy: Output<serde_json::Value>,
    /// Gets the URLs that are used to perform a retrieval of a public blob, queue, or table object from the secondary location of the storage account. Only available if the SKU name is Standard_RAGRS.
    pub secondary_endpoints: Output<serde_json::Value>,
    /// Gets the location of the geo-replicated secondary for the storage account. Only available if the accountType is Standard_GRS or Standard_RAGRS.
    pub secondary_location: Output<serde_json::Value>,
    /// Gets the SKU.
    pub sku: Output<serde_json::Value>,
    /// Gets the status indicating whether the primary location of the storage account is available or unavailable.
    pub status_of_primary: Output<serde_json::Value>,
    /// Gets the status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS.
    pub status_of_secondary: Output<serde_json::Value>,
    /// This property is readOnly and is set by server during asynchronous storage account sku conversion operations.
    pub storage_account_sku_conversion_status: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
    /// The availability zones.
    pub zones: Output<serde_json::Value>,
}

impl StorageAccount {
    const TYPE_TOKEN: &'static str = "azure-native:storage/v20250601:StorageAccount";

    /// Create a new StorageAccount resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: StorageAccountArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.access_tier {
            pulumi_sdk::resolve_input("accessTier", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.account_name {
            pulumi_sdk::resolve_input("accountName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_blob_public_access {
            pulumi_sdk::resolve_input("allowBlobPublicAccess", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_cross_tenant_replication {
            pulumi_sdk::resolve_input("allowCrossTenantReplication", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_shared_key_access {
            pulumi_sdk::resolve_input("allowSharedKeyAccess", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allowed_copy_scope {
            pulumi_sdk::resolve_input("allowedCopyScope", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.azure_files_identity_based_authentication {
            pulumi_sdk::resolve_input("azureFilesIdentityBasedAuthentication", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.custom_domain {
            pulumi_sdk::resolve_input("customDomain", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.default_to_o_auth_authentication {
            pulumi_sdk::resolve_input("defaultToOAuthAuthentication", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dns_endpoint_type {
            pulumi_sdk::resolve_input("dnsEndpointType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dual_stack_endpoint_preference {
            pulumi_sdk::resolve_input("dualStackEndpointPreference", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_extended_groups {
            pulumi_sdk::resolve_input("enableExtendedGroups", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_https_traffic_only {
            pulumi_sdk::resolve_input("enableHttpsTrafficOnly", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_nfs_v3 {
            pulumi_sdk::resolve_input("enableNfsV3", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.encryption {
            pulumi_sdk::resolve_input("encryption", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.geo_priority_replication_status {
            pulumi_sdk::resolve_input("geoPriorityReplicationStatus", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.immutable_storage_with_versioning {
            pulumi_sdk::resolve_input("immutableStorageWithVersioning", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_hns_enabled {
            pulumi_sdk::resolve_input("isHnsEnabled", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_local_user_enabled {
            pulumi_sdk::resolve_input("isLocalUserEnabled", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_sftp_enabled {
            pulumi_sdk::resolve_input("isSftpEnabled", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.key_policy {
            pulumi_sdk::resolve_input("keyPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("kind", args.kind, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.large_file_shares_state {
            pulumi_sdk::resolve_input("largeFileSharesState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.minimum_tls_version {
            pulumi_sdk::resolve_input("minimumTlsVersion", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_rule_set {
            pulumi_sdk::resolve_input("networkRuleSet", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.placement {
            pulumi_sdk::resolve_input("placement", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_network_access {
            pulumi_sdk::resolve_input("publicNetworkAccess", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.routing_preference {
            pulumi_sdk::resolve_input("routingPreference", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sas_policy {
            pulumi_sdk::resolve_input("sasPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("sku", args.sku, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.zones {
            pulumi_sdk::resolve_input_vec("zones", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            access_tier: registered.outputs.get("accessTier")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            account_migration_in_progress: registered.outputs.get("accountMigrationInProgress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_blob_public_access: registered.outputs.get("allowBlobPublicAccess")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_cross_tenant_replication: registered.outputs.get("allowCrossTenantReplication")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_shared_key_access: registered.outputs.get("allowSharedKeyAccess")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allowed_copy_scope: registered.outputs.get("allowedCopyScope")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_files_identity_based_authentication: registered.outputs.get("azureFilesIdentityBasedAuthentication")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            blob_restore_status: registered.outputs.get("blobRestoreStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            creation_time: registered.outputs.get("creationTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_domain: registered.outputs.get("customDomain")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            default_to_o_auth_authentication: registered.outputs.get("defaultToOAuthAuthentication")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dns_endpoint_type: registered.outputs.get("dnsEndpointType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dual_stack_endpoint_preference: registered.outputs.get("dualStackEndpointPreference")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_extended_groups: registered.outputs.get("enableExtendedGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_https_traffic_only: registered.outputs.get("enableHttpsTrafficOnly")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_nfs_v3: registered.outputs.get("enableNfsV3")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            encryption: registered.outputs.get("encryption")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            failover_in_progress: registered.outputs.get("failoverInProgress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            geo_priority_replication_status: registered.outputs.get("geoPriorityReplicationStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            geo_replication_stats: registered.outputs.get("geoReplicationStats")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            immutable_storage_with_versioning: registered.outputs.get("immutableStorageWithVersioning")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_hns_enabled: registered.outputs.get("isHnsEnabled")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_local_user_enabled: registered.outputs.get("isLocalUserEnabled")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_sftp_enabled: registered.outputs.get("isSftpEnabled")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_sku_conversion_blocked: registered.outputs.get("isSkuConversionBlocked")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            key_creation_time: registered.outputs.get("keyCreationTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            key_policy: registered.outputs.get("keyPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            kind: registered.outputs.get("kind")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            large_file_shares_state: registered.outputs.get("largeFileSharesState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            last_geo_failover_time: registered.outputs.get("lastGeoFailoverTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            minimum_tls_version: registered.outputs.get("minimumTlsVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_rule_set: registered.outputs.get("networkRuleSet")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            placement: registered.outputs.get("placement")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            primary_endpoints: registered.outputs.get("primaryEndpoints")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            primary_location: registered.outputs.get("primaryLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_endpoint_connections: registered.outputs.get("privateEndpointConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_network_access: registered.outputs.get("publicNetworkAccess")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_preference: registered.outputs.get("routingPreference")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sas_policy: registered.outputs.get("sasPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            secondary_endpoints: registered.outputs.get("secondaryEndpoints")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            secondary_location: registered.outputs.get("secondaryLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            status_of_primary: registered.outputs.get("statusOfPrimary")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            status_of_secondary: registered.outputs.get("statusOfSecondary")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            storage_account_sku_conversion_status: registered.outputs.get("storageAccountSkuConversionStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
