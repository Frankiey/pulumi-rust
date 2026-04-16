use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// NetworkVirtualAppliance Resource.
pub struct NetworkVirtualApplianceArgs {
    /// BootStrapConfigurationBlobs storage URLs.
    pub boot_strap_configuration_blobs: Option<Vec<Input<String>>>,
    /// CloudInitConfiguration string in plain text.
    pub cloud_init_configuration: Option<Input<String>>,
    /// CloudInitConfigurationBlob storage URLs.
    pub cloud_init_configuration_blobs: Option<Vec<Input<String>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The service principal that has read access to cloud-init and config blob.
    pub identity: Option<Input<network::v20210301::ManagedServiceIdentityArgs>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of Network Virtual Appliance.
    pub network_virtual_appliance_name: Option<Input<String>>,
    /// Network Virtual Appliance SKU.
    pub nva_sku: Option<Input<network::v20210301::VirtualApplianceSkuPropertiesArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Public key for SSH login.
    pub ssh_public_key: Option<Input<String>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// VirtualAppliance ASN.
    pub virtual_appliance_asn: Option<Input<f64>>,
    /// The Virtual Hub where Network Virtual Appliance is being deployed.
    pub virtual_hub: Option<Input<network::v20210301::SubResourceArgs>>,
}

/// NetworkVirtualAppliance Resource.
pub struct NetworkVirtualAppliance {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Address Prefix.
    pub address_prefix: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// BootStrapConfigurationBlobs storage URLs.
    pub boot_strap_configuration_blobs: Output<serde_json::Value>,
    /// CloudInitConfiguration string in plain text.
    pub cloud_init_configuration: Output<serde_json::Value>,
    /// CloudInitConfigurationBlob storage URLs.
    pub cloud_init_configuration_blobs: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The service principal that has read access to cloud-init and config blob.
    pub identity: Output<serde_json::Value>,
    /// List of references to InboundSecurityRules.
    pub inbound_security_rules: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// Network Virtual Appliance SKU.
    pub nva_sku: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Public key for SSH login.
    pub ssh_public_key: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// VirtualAppliance ASN.
    pub virtual_appliance_asn: Output<serde_json::Value>,
    /// List of Virtual Appliance Network Interfaces.
    pub virtual_appliance_nics: Output<serde_json::Value>,
    /// List of references to VirtualApplianceSite.
    pub virtual_appliance_sites: Output<serde_json::Value>,
    /// The Virtual Hub where Network Virtual Appliance is being deployed.
    pub virtual_hub: Output<serde_json::Value>,
}

impl NetworkVirtualAppliance {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20210301:NetworkVirtualAppliance";

    /// Create a new NetworkVirtualAppliance resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkVirtualApplianceArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.boot_strap_configuration_blobs {
            pulumi_sdk::resolve_input_vec("bootStrapConfigurationBlobs", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.cloud_init_configuration {
            pulumi_sdk::resolve_input("cloudInitConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.cloud_init_configuration_blobs {
            pulumi_sdk::resolve_input_vec("cloudInitConfigurationBlobs", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_virtual_appliance_name {
            pulumi_sdk::resolve_input("networkVirtualApplianceName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nva_sku {
            pulumi_sdk::resolve_input("nvaSku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.ssh_public_key {
            pulumi_sdk::resolve_input("sshPublicKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_appliance_asn {
            pulumi_sdk::resolve_input("virtualApplianceAsn", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_hub {
            pulumi_sdk::resolve_input("virtualHub", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            address_prefix: registered.outputs.get("addressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            boot_strap_configuration_blobs: registered.outputs.get("bootStrapConfigurationBlobs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            cloud_init_configuration: registered.outputs.get("cloudInitConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            cloud_init_configuration_blobs: registered.outputs.get("cloudInitConfigurationBlobs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            inbound_security_rules: registered.outputs.get("inboundSecurityRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            nva_sku: registered.outputs.get("nvaSku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ssh_public_key: registered.outputs.get("sshPublicKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_appliance_asn: registered.outputs.get("virtualApplianceAsn")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_appliance_nics: registered.outputs.get("virtualApplianceNics")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_appliance_sites: registered.outputs.get("virtualApplianceSites")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_hub: registered.outputs.get("virtualHub")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
