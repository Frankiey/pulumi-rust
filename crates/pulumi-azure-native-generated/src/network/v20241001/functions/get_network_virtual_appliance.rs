use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Network Virtual Appliance.
#[derive(Default)]
pub struct GetNetworkVirtualApplianceArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of Network Virtual Appliance.
    pub network_virtual_appliance_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkVirtualApplianceResult {
    /// Details required for Additional Network Interface. This property is not compatible with the NVA deployed in VNets.
    pub additional_nics: Option<Vec<network::v20241001::VirtualApplianceAdditionalNicPropertiesResponse>>,
    /// Address Prefix.
    pub address_prefix: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// BootStrapConfigurationBlobs storage URLs.
    pub boot_strap_configuration_blobs: Option<Vec<String>>,
    /// CloudInitConfiguration string in plain text.
    pub cloud_init_configuration: Option<String>,
    /// CloudInitConfigurationBlob storage URLs.
    pub cloud_init_configuration_blobs: Option<Vec<String>>,
    /// The delegation for the Virtual Appliance. Only appliable for SaaS NVA.
    pub delegation: Option<network::v20241001::DelegationPropertiesResponse>,
    /// The deployment type. PartnerManaged for the SaaS NVA
    pub deployment_type: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The service principal that has read access to cloud-init and config blob.
    pub identity: Option<network::v20241001::ManagedServiceIdentityResponse>,
    /// List of references to InboundSecurityRules.
    pub inbound_security_rules: Vec<network::v20241001::SubResourceResponse>,
    /// List of Resource Uri of Public IPs for Internet Ingress Scenario.
    pub internet_ingress_public_ips: Option<Vec<network::v20241001::InternetIngressPublicIpsPropertiesResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Network Profile containing configurations for Public and Private NIC.
    pub network_profile: Option<network::v20241001::NetworkVirtualAppliancePropertiesFormatResponseNetworkProfile>,
    /// The NVA in VNet interface configurations
    pub nva_interface_configurations: Option<Vec<network::v20241001::NvaInterfaceConfigurationsPropertiesResponse>>,
    /// Network Virtual Appliance SKU.
    pub nva_sku: Option<network::v20241001::VirtualApplianceSkuPropertiesResponse>,
    /// The delegation for the Virtual Appliance
    pub partner_managed_resource: Option<network::v20241001::PartnerManagedResourcePropertiesResponse>,
    /// A Internal Load Balancer's HA port frontend IP address. Can be used to set routes & UDR to load balance traffic between NVA instances
    pub private_ip_address: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Public key for SSH login.
    pub ssh_public_key: Option<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// VirtualAppliance ASN. Microsoft private, public and IANA reserved ASN are not supported.
    pub virtual_appliance_asn: Option<f64>,
    /// List of references to VirtualApplianceConnections.
    pub virtual_appliance_connections: Vec<network::v20241001::SubResourceResponse>,
    /// List of Virtual Appliance Network Interfaces.
    pub virtual_appliance_nics: Vec<network::v20241001::VirtualApplianceNicPropertiesResponse>,
    /// List of references to VirtualApplianceSite.
    pub virtual_appliance_sites: Vec<network::v20241001::SubResourceResponse>,
    /// The Virtual Hub where Network Virtual Appliance is being deployed.
    pub virtual_hub: Option<network::v20241001::SubResourceResponse>,
}

/// Gets the specified Network Virtual Appliance.
pub async fn get_network_virtual_appliance(
    ctx: &Context,
    args: GetNetworkVirtualApplianceArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkVirtualApplianceResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("networkVirtualApplianceName".to_string(), json!(args.network_virtual_appliance_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20241001:getNetworkVirtualAppliance", invoke_args, &opts).await?;

    Ok(GetNetworkVirtualApplianceResult {
        additional_nics: result.fields.get("additionalNics").cloned().map(serde_json::from_value).transpose()?,
        address_prefix: serde_json::from_value(result.fields.get("addressPrefix").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        boot_strap_configuration_blobs: result.fields.get("bootStrapConfigurationBlobs").cloned().map(serde_json::from_value).transpose()?,
        cloud_init_configuration: result.fields.get("cloudInitConfiguration").cloned().map(serde_json::from_value).transpose()?,
        cloud_init_configuration_blobs: result.fields.get("cloudInitConfigurationBlobs").cloned().map(serde_json::from_value).transpose()?,
        delegation: result.fields.get("delegation").cloned().map(serde_json::from_value).transpose()?,
        deployment_type: serde_json::from_value(result.fields.get("deploymentType").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        inbound_security_rules: serde_json::from_value(result.fields.get("inboundSecurityRules").cloned().unwrap_or_default())?
            ,
        internet_ingress_public_ips: result.fields.get("internetIngressPublicIps").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_profile: result.fields.get("networkProfile").cloned().map(serde_json::from_value).transpose()?,
        nva_interface_configurations: result.fields.get("nvaInterfaceConfigurations").cloned().map(serde_json::from_value).transpose()?,
        nva_sku: result.fields.get("nvaSku").cloned().map(serde_json::from_value).transpose()?,
        partner_managed_resource: result.fields.get("partnerManagedResource").cloned().map(serde_json::from_value).transpose()?,
        private_ip_address: serde_json::from_value(result.fields.get("privateIpAddress").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        ssh_public_key: result.fields.get("sshPublicKey").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_appliance_asn: result.fields.get("virtualApplianceAsn").cloned().map(serde_json::from_value).transpose()?,
        virtual_appliance_connections: serde_json::from_value(result.fields.get("virtualApplianceConnections").cloned().unwrap_or_default())?
            ,
        virtual_appliance_nics: serde_json::from_value(result.fields.get("virtualApplianceNics").cloned().unwrap_or_default())?
            ,
        virtual_appliance_sites: serde_json::from_value(result.fields.get("virtualApplianceSites").cloned().unwrap_or_default())?
            ,
        virtual_hub: result.fields.get("virtualHub").cloned().map(serde_json::from_value).transpose()?,
    })
}
