use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Returns the properties for the specified storage account including but not limited to name, SKU name, location, and account status. The ListKeys operation should be used to retrieve storage keys.
#[derive(Default)]
pub struct GetStorageAccountArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// May be used to expand the properties within account's properties. By default, data is not included when fetching properties. Currently we only support geoReplicationStats and blobRestoreStatus.
    pub expand: Option<String>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetStorageAccountResult {
    /// Required for storage accounts where kind = BlobStorage. The access tier is used for billing. The 'Premium' access tier is the default value for premium block blobs storage account type and it cannot be changed for the premium block blobs storage account type.
    pub access_tier: String,
    /// If customer initiated account migration is in progress, the value will be true else it will be null.
    pub account_migration_in_progress: bool,
    /// Allow or disallow public access to all blobs or containers in the storage account. The default interpretation is false for this property.
    pub allow_blob_public_access: Option<bool>,
    /// Allow or disallow cross AAD tenant object replication. Set this property to true for new or existing accounts only if object replication policies will involve storage accounts in different AAD tenants. The default interpretation is false for new accounts to follow best security practices by default.
    pub allow_cross_tenant_replication: Option<bool>,
    /// Indicates whether the storage account permits requests to be authorized with the account access key via Shared Key. If false, then all requests, including shared access signatures, must be authorized with Azure Active Directory (Azure AD). The default value is null, which is equivalent to true.
    pub allow_shared_key_access: Option<bool>,
    /// Restrict copy to and from Storage Accounts within an AAD tenant or with Private Links to the same VNet.
    pub allowed_copy_scope: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Provides the identity based authentication settings for Azure Files.
    pub azure_files_identity_based_authentication: Option<storage::v20250101::AzureFilesIdentityBasedAuthenticationResponse>,
    /// Blob restore status
    pub blob_restore_status: storage::v20250101::BlobRestoreStatusResponse,
    /// Gets the creation date and time of the storage account in UTC.
    pub creation_time: String,
    /// Gets the custom domain the user assigned to this storage account.
    pub custom_domain: storage::v20250101::CustomDomainResponse,
    /// A boolean flag which indicates whether the default authentication is OAuth or not. The default interpretation is false for this property.
    pub default_to_o_auth_authentication: Option<bool>,
    /// Allows you to specify the type of endpoint. Set this to AzureDNSZone to create a large number of accounts in a single subscription, which creates accounts in an Azure DNS Zone and the endpoint URL will have an alphanumeric DNS Zone identifier.
    pub dns_endpoint_type: Option<String>,
    /// Maintains information about the Internet protocol opted by the user.
    pub dual_stack_endpoint_preference: Option<storage::v20250101::DualStackEndpointPreferenceResponse>,
    /// Enables extended group support with local users feature, if set to true
    pub enable_extended_groups: Option<bool>,
    /// Allows https traffic only to storage service if sets to true.
    pub enable_https_traffic_only: Option<bool>,
    /// NFS 3.0 protocol support enabled if set to true.
    pub enable_nfs_v3: Option<bool>,
    /// Encryption settings to be used for server-side encryption for the storage account.
    pub encryption: storage::v20250101::EncryptionResponse,
    /// The extendedLocation of the resource.
    pub extended_location: Option<storage::v20250101::ExtendedLocationResponse>,
    /// If the failover is in progress, the value will be true, otherwise, it will be null.
    pub failover_in_progress: bool,
    /// Geo Replication Stats
    pub geo_replication_stats: storage::v20250101::GeoReplicationStatsResponse,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The identity of the resource.
    pub identity: Option<storage::v20250101::IdentityResponse>,
    /// The property is immutable and can only be set to true at the account creation time. When set to true, it enables object level immutability for all the containers in the account by default.
    pub immutable_storage_with_versioning: Option<storage::v20250101::ImmutableStorageAccountResponse>,
    /// Account HierarchicalNamespace enabled if sets to true.
    pub is_hns_enabled: Option<bool>,
    /// Enables local users feature, if set to true
    pub is_local_user_enabled: Option<bool>,
    /// Enables Secure File Transfer Protocol, if set to true
    pub is_sftp_enabled: Option<bool>,
    /// This property will be set to true or false on an event of ongoing migration. Default value is null.
    pub is_sku_conversion_blocked: bool,
    /// Storage account keys creation time.
    pub key_creation_time: storage::v20250101::KeyCreationTimeResponse,
    /// KeyPolicy assigned to the storage account.
    pub key_policy: storage::v20250101::KeyPolicyResponse,
    /// Gets the Kind.
    pub kind: String,
    /// Allow large file shares if sets to Enabled. It cannot be disabled once it is enabled.
    pub large_file_shares_state: Option<String>,
    /// Gets the timestamp of the most recent instance of a failover to the secondary location. Only the most recent timestamp is retained. This element is not returned if there has never been a failover instance. Only available if the accountType is Standard_GRS or Standard_RAGRS.
    pub last_geo_failover_time: String,
    /// The geo-location where the resource lives
    pub location: String,
    /// Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property.
    pub minimum_tls_version: Option<String>,
    /// The name of the resource
    pub name: String,
    /// Network rule set
    pub network_rule_set: storage::v20250101::NetworkRuleSetResponse,
    /// Optional. Gets or sets the zonal placement details for the storage account.
    pub placement: Option<storage::v20250101::PlacementResponse>,
    /// Gets the URLs that are used to perform a retrieval of a public blob, queue, or table object. Note that Standard_ZRS and Premium_LRS accounts only return the blob endpoint.
    pub primary_endpoints: storage::v20250101::EndpointsResponse,
    /// Gets the location of the primary data center for the storage account.
    pub primary_location: String,
    /// List of private endpoint connection associated with the specified storage account
    pub private_endpoint_connections: Vec<storage::v20250101::PrivateEndpointConnectionResponse>,
    /// Gets the status of the storage account at the time the operation was called.
    pub provisioning_state: String,
    /// Allow, disallow, or let Network Security Perimeter configuration to evaluate public network access to Storage Account.
    pub public_network_access: Option<String>,
    /// Maintains information about the network routing choice opted by the user for data transfer
    pub routing_preference: Option<storage::v20250101::RoutingPreferenceResponse>,
    /// SasPolicy assigned to the storage account.
    pub sas_policy: storage::v20250101::SasPolicyResponse,
    /// Gets the URLs that are used to perform a retrieval of a public blob, queue, or table object from the secondary location of the storage account. Only available if the SKU name is Standard_RAGRS.
    pub secondary_endpoints: storage::v20250101::EndpointsResponse,
    /// Gets the location of the geo-replicated secondary for the storage account. Only available if the accountType is Standard_GRS or Standard_RAGRS.
    pub secondary_location: String,
    /// Gets the SKU.
    pub sku: storage::v20250101::SkuResponse,
    /// Gets the status indicating whether the primary location of the storage account is available or unavailable.
    pub status_of_primary: String,
    /// Gets the status indicating whether the secondary location of the storage account is available or unavailable. Only available if the SKU name is Standard_GRS or Standard_RAGRS.
    pub status_of_secondary: String,
    /// This property is readOnly and is set by server during asynchronous storage account sku conversion operations.
    pub storage_account_sku_conversion_status: Option<storage::v20250101::StorageAccountSkuConversionStatusResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
    /// Optional. Gets or sets the pinned logical availability zone for the storage account.
    pub zones: Option<Vec<String>>,
}

/// Returns the properties for the specified storage account including but not limited to name, SKU name, location, and account status. The ListKeys operation should be used to retrieve storage keys.
pub async fn get_storage_account(
    ctx: &Context,
    args: GetStorageAccountArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetStorageAccountResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250101:getStorageAccount", invoke_args, &opts).await?;

    Ok(GetStorageAccountResult {
        access_tier: serde_json::from_value(result.fields.get("accessTier").cloned().unwrap_or_default())?
            ,
        account_migration_in_progress: serde_json::from_value(result.fields.get("accountMigrationInProgress").cloned().unwrap_or_default())?
            ,
        allow_blob_public_access: result.fields.get("allowBlobPublicAccess").cloned().map(serde_json::from_value).transpose()?,
        allow_cross_tenant_replication: result.fields.get("allowCrossTenantReplication").cloned().map(serde_json::from_value).transpose()?,
        allow_shared_key_access: result.fields.get("allowSharedKeyAccess").cloned().map(serde_json::from_value).transpose()?,
        allowed_copy_scope: result.fields.get("allowedCopyScope").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        azure_files_identity_based_authentication: result.fields.get("azureFilesIdentityBasedAuthentication").cloned().map(serde_json::from_value).transpose()?,
        blob_restore_status: serde_json::from_value(result.fields.get("blobRestoreStatus").cloned().unwrap_or_default())?
            ,
        creation_time: serde_json::from_value(result.fields.get("creationTime").cloned().unwrap_or_default())?
            ,
        custom_domain: serde_json::from_value(result.fields.get("customDomain").cloned().unwrap_or_default())?
            ,
        default_to_o_auth_authentication: result.fields.get("defaultToOAuthAuthentication").cloned().map(serde_json::from_value).transpose()?,
        dns_endpoint_type: result.fields.get("dnsEndpointType").cloned().map(serde_json::from_value).transpose()?,
        dual_stack_endpoint_preference: result.fields.get("dualStackEndpointPreference").cloned().map(serde_json::from_value).transpose()?,
        enable_extended_groups: result.fields.get("enableExtendedGroups").cloned().map(serde_json::from_value).transpose()?,
        enable_https_traffic_only: result.fields.get("enableHttpsTrafficOnly").cloned().map(serde_json::from_value).transpose()?,
        enable_nfs_v3: result.fields.get("enableNfsV3").cloned().map(serde_json::from_value).transpose()?,
        encryption: serde_json::from_value(result.fields.get("encryption").cloned().unwrap_or_default())?
            ,
        extended_location: result.fields.get("extendedLocation").cloned().map(serde_json::from_value).transpose()?,
        failover_in_progress: serde_json::from_value(result.fields.get("failoverInProgress").cloned().unwrap_or_default())?
            ,
        geo_replication_stats: serde_json::from_value(result.fields.get("geoReplicationStats").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        immutable_storage_with_versioning: result.fields.get("immutableStorageWithVersioning").cloned().map(serde_json::from_value).transpose()?,
        is_hns_enabled: result.fields.get("isHnsEnabled").cloned().map(serde_json::from_value).transpose()?,
        is_local_user_enabled: result.fields.get("isLocalUserEnabled").cloned().map(serde_json::from_value).transpose()?,
        is_sftp_enabled: result.fields.get("isSftpEnabled").cloned().map(serde_json::from_value).transpose()?,
        is_sku_conversion_blocked: serde_json::from_value(result.fields.get("isSkuConversionBlocked").cloned().unwrap_or_default())?
            ,
        key_creation_time: serde_json::from_value(result.fields.get("keyCreationTime").cloned().unwrap_or_default())?
            ,
        key_policy: serde_json::from_value(result.fields.get("keyPolicy").cloned().unwrap_or_default())?
            ,
        kind: serde_json::from_value(result.fields.get("kind").cloned().unwrap_or_default())?
            ,
        large_file_shares_state: result.fields.get("largeFileSharesState").cloned().map(serde_json::from_value).transpose()?,
        last_geo_failover_time: serde_json::from_value(result.fields.get("lastGeoFailoverTime").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        minimum_tls_version: result.fields.get("minimumTlsVersion").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_rule_set: serde_json::from_value(result.fields.get("networkRuleSet").cloned().unwrap_or_default())?
            ,
        placement: result.fields.get("placement").cloned().map(serde_json::from_value).transpose()?,
        primary_endpoints: serde_json::from_value(result.fields.get("primaryEndpoints").cloned().unwrap_or_default())?
            ,
        primary_location: serde_json::from_value(result.fields.get("primaryLocation").cloned().unwrap_or_default())?
            ,
        private_endpoint_connections: serde_json::from_value(result.fields.get("privateEndpointConnections").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        public_network_access: result.fields.get("publicNetworkAccess").cloned().map(serde_json::from_value).transpose()?,
        routing_preference: result.fields.get("routingPreference").cloned().map(serde_json::from_value).transpose()?,
        sas_policy: serde_json::from_value(result.fields.get("sasPolicy").cloned().unwrap_or_default())?
            ,
        secondary_endpoints: serde_json::from_value(result.fields.get("secondaryEndpoints").cloned().unwrap_or_default())?
            ,
        secondary_location: serde_json::from_value(result.fields.get("secondaryLocation").cloned().unwrap_or_default())?
            ,
        sku: serde_json::from_value(result.fields.get("sku").cloned().unwrap_or_default())?
            ,
        status_of_primary: serde_json::from_value(result.fields.get("statusOfPrimary").cloned().unwrap_or_default())?
            ,
        status_of_secondary: serde_json::from_value(result.fields.get("statusOfSecondary").cloned().unwrap_or_default())?
            ,
        storage_account_sku_conversion_status: result.fields.get("storageAccountSkuConversionStatus").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
