use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Bastion Host resource.
pub struct BastionHostArgs {
    /// The name of the Bastion Host.
    pub bastion_host_name: Option<Input<String>>,
    /// Enable/Disable Copy/Paste feature of the Bastion Host resource.
    pub disable_copy_paste: Option<Input<bool>>,
    /// FQDN for the endpoint on which bastion host is accessible.
    pub dns_name: Option<Input<String>>,
    /// Enable/Disable File Copy feature of the Bastion Host resource.
    pub enable_file_copy: Option<Input<bool>>,
    /// Enable/Disable IP Connect feature of the Bastion Host resource.
    pub enable_ip_connect: Option<Input<bool>>,
    /// Enable/Disable Kerberos feature of the Bastion Host resource.
    pub enable_kerberos: Option<Input<bool>>,
    /// Enable/Disable Session Recording feature of the Bastion Host resource.
    pub enable_session_recording: Option<Input<bool>>,
    /// Enable/Disable Shareable Link of the Bastion Host resource.
    pub enable_shareable_link: Option<Input<bool>>,
    /// Enable/Disable Tunneling feature of the Bastion Host resource.
    pub enable_tunneling: Option<Input<bool>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// IP configuration of the Bastion Host resource.
    pub ip_configurations: Option<Vec<Input<network::v20240301::BastionHostIPConfigurationArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    pub network_acls: Option<Input<network::v20240301::BastionHostPropertiesFormatNetworkAclsArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The scale units for the Bastion Host resource.
    pub scale_units: Option<Input<i64>>,
    /// The sku of this Bastion Host.
    pub sku: Option<Input<network::v20240301::SkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Reference to an existing virtual network required for Developer Bastion Host only.
    pub virtual_network: Option<Input<network::v20240301::SubResourceArgs>>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Option<Vec<Input<String>>>,
}

/// Bastion Host resource.
pub struct BastionHost {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Enable/Disable Copy/Paste feature of the Bastion Host resource.
    pub disable_copy_paste: Output<serde_json::Value>,
    /// FQDN for the endpoint on which bastion host is accessible.
    pub dns_name: Output<serde_json::Value>,
    /// Enable/Disable File Copy feature of the Bastion Host resource.
    pub enable_file_copy: Output<serde_json::Value>,
    /// Enable/Disable IP Connect feature of the Bastion Host resource.
    pub enable_ip_connect: Output<serde_json::Value>,
    /// Enable/Disable Kerberos feature of the Bastion Host resource.
    pub enable_kerberos: Output<serde_json::Value>,
    /// Enable/Disable Session Recording feature of the Bastion Host resource.
    pub enable_session_recording: Output<serde_json::Value>,
    /// Enable/Disable Shareable Link of the Bastion Host resource.
    pub enable_shareable_link: Output<serde_json::Value>,
    /// Enable/Disable Tunneling feature of the Bastion Host resource.
    pub enable_tunneling: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// IP configuration of the Bastion Host resource.
    pub ip_configurations: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    pub network_acls: Output<serde_json::Value>,
    /// The provisioning state of the bastion host resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The scale units for the Bastion Host resource.
    pub scale_units: Output<serde_json::Value>,
    /// The sku of this Bastion Host.
    pub sku: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// Reference to an existing virtual network required for Developer Bastion Host only.
    pub virtual_network: Output<serde_json::Value>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Output<serde_json::Value>,
}

impl BastionHost {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240301:BastionHost";

    /// Create a new BastionHost resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: BastionHostArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.bastion_host_name {
            pulumi_sdk::resolve_input("bastionHostName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.disable_copy_paste {
            pulumi_sdk::resolve_input("disableCopyPaste", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dns_name {
            pulumi_sdk::resolve_input("dnsName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_file_copy {
            pulumi_sdk::resolve_input("enableFileCopy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_ip_connect {
            pulumi_sdk::resolve_input("enableIpConnect", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_kerberos {
            pulumi_sdk::resolve_input("enableKerberos", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_session_recording {
            pulumi_sdk::resolve_input("enableSessionRecording", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_shareable_link {
            pulumi_sdk::resolve_input("enableShareableLink", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_tunneling {
            pulumi_sdk::resolve_input("enableTunneling", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_configurations {
            pulumi_sdk::resolve_input_vec("ipConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_acls {
            pulumi_sdk::resolve_input("networkAcls", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.scale_units {
            pulumi_sdk::resolve_input("scaleUnits", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network {
            pulumi_sdk::resolve_input("virtualNetwork", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            disable_copy_paste: registered.outputs.get("disableCopyPaste")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dns_name: registered.outputs.get("dnsName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_file_copy: registered.outputs.get("enableFileCopy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_ip_connect: registered.outputs.get("enableIpConnect")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_kerberos: registered.outputs.get("enableKerberos")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_session_recording: registered.outputs.get("enableSessionRecording")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_shareable_link: registered.outputs.get("enableShareableLink")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_tunneling: registered.outputs.get("enableTunneling")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_acls: registered.outputs.get("networkAcls")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            scale_units: registered.outputs.get("scaleUnits")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network: registered.outputs.get("virtualNetwork")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
