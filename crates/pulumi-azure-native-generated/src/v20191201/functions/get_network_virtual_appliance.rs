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
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// BootStrapConfigurationBlob storage URLs.
    pub boot_strap_configuration_blob: Option<Vec<String>>,
    /// CloudInitConfigurationBlob storage URLs.
    pub cloud_init_configuration_blob: Option<Vec<String>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The service principal that has read access to cloud-init and config blob.
    pub identity: Option<network::v20191201::ManagedServiceIdentityResponse>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Network Virtual Appliance SKU.
    pub sku: Option<network::v20191201::VirtualApplianceSkuPropertiesResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// VirtualAppliance ASN.
    pub virtual_appliance_asn: Option<f64>,
    /// List of Virtual Appliance Network Interfaces.
    pub virtual_appliance_nics: Vec<network::v20191201::VirtualApplianceNicPropertiesResponse>,
    /// The Virtual Hub where Network Virtual Appliance is being deployed.
    pub virtual_hub: Option<network::v20191201::SubResourceResponse>,
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
    let result = ctx.invoke("azure-native:network/v20191201:getNetworkVirtualAppliance", invoke_args, &opts).await?;

    Ok(GetNetworkVirtualApplianceResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        boot_strap_configuration_blob: result.fields.get("bootStrapConfigurationBlob").cloned().map(serde_json::from_value).transpose()?,
        cloud_init_configuration_blob: result.fields.get("cloudInitConfigurationBlob").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_appliance_asn: result.fields.get("virtualApplianceAsn").cloned().map(serde_json::from_value).transpose()?,
        virtual_appliance_nics: serde_json::from_value(result.fields.get("virtualApplianceNics").cloned().unwrap_or_default())?
            ,
        virtual_hub: result.fields.get("virtualHub").cloned().map(serde_json::from_value).transpose()?,
    })
}
