use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Public IP address resource.
pub struct PublicIPAddressArgs {
    /// The DDoS protection custom policy associated with the public IP address.
    pub ddos_settings: Option<Input<network::v20240501::DdosSettingsArgs>>,
    /// Specify what happens to the public IP address when the VM using it is deleted
    pub delete_option: Option<Input<serde_json::Value>>,
    /// The FQDN of the DNS record associated with the public IP address.
    pub dns_settings: Option<Input<network::v20240501::PublicIPAddressDnsSettingsArgs>>,
    /// The extended location of the public ip address.
    pub extended_location: Option<Input<network::v20240501::ExtendedLocationArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The idle timeout of the public IP address.
    pub idle_timeout_in_minutes: Option<Input<i64>>,
    /// The IP address associated with the public IP address resource.
    pub ip_address: Option<Input<String>>,
    /// The list of tags associated with the public IP address.
    pub ip_tags: Option<Vec<Input<network::v20240501::IpTagArgs>>>,
    /// The linked public IP address of the public IP address resource.
    pub linked_public_ip_address: Option<Input<network::v20240501::PublicIPAddressArgs>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// Migration phase of Public IP Address.
    pub migration_phase: Option<Input<serde_json::Value>>,
    /// The NatGateway for the Public IP address.
    pub nat_gateway: Option<Input<network::v20240501::NatGatewayArgs>>,
    /// The public IP address version.
    pub public_ip_address_version: Option<Input<serde_json::Value>>,
    /// The public IP address allocation method.
    pub public_ip_allocation_method: Option<Input<serde_json::Value>>,
    /// The Public IP Prefix this Public IP Address should be allocated from.
    pub public_ip_prefix: Option<Input<network::v20240501::SubResourceArgs>>,
    /// The name of the public IP address.
    pub public_ip_address_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The service public IP address of the public IP address resource.
    pub service_public_ip_address: Option<Input<network::v20240501::PublicIPAddressArgs>>,
    /// The public IP address SKU.
    pub sku: Option<Input<network::v20240501::PublicIPAddressSkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Option<Vec<Input<String>>>,
}

/// Public IP address resource.
pub struct PublicIPAddress {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The DDoS protection custom policy associated with the public IP address.
    pub ddos_settings: Output<serde_json::Value>,
    /// Specify what happens to the public IP address when the VM using it is deleted
    pub delete_option: Output<serde_json::Value>,
    /// The FQDN of the DNS record associated with the public IP address.
    pub dns_settings: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the public ip address.
    pub extended_location: Output<serde_json::Value>,
    /// The idle timeout of the public IP address.
    pub idle_timeout_in_minutes: Output<serde_json::Value>,
    /// The IP address associated with the public IP address resource.
    pub ip_address: Output<serde_json::Value>,
    /// The IP configuration associated with the public IP address.
    pub ip_configuration: Output<serde_json::Value>,
    /// The list of tags associated with the public IP address.
    pub ip_tags: Output<serde_json::Value>,
    /// The linked public IP address of the public IP address resource.
    pub linked_public_ip_address: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Migration phase of Public IP Address.
    pub migration_phase: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The NatGateway for the Public IP address.
    pub nat_gateway: Output<serde_json::Value>,
    /// The provisioning state of the public IP address resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The public IP address version.
    pub public_ip_address_version: Output<serde_json::Value>,
    /// The public IP address allocation method.
    pub public_ip_allocation_method: Output<serde_json::Value>,
    /// The Public IP Prefix this Public IP Address should be allocated from.
    pub public_ip_prefix: Output<serde_json::Value>,
    /// The resource GUID property of the public IP address resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The service public IP address of the public IP address resource.
    pub service_public_ip_address: Output<serde_json::Value>,
    /// The public IP address SKU.
    pub sku: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Output<serde_json::Value>,
}

impl PublicIPAddress {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240501:PublicIPAddress";

    /// Create a new PublicIPAddress resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PublicIPAddressArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.ddos_settings {
            pulumi_sdk::resolve_input("ddosSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.delete_option {
            pulumi_sdk::resolve_input("deleteOption", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dns_settings {
            pulumi_sdk::resolve_input("dnsSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.idle_timeout_in_minutes {
            pulumi_sdk::resolve_input("idleTimeoutInMinutes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_address {
            pulumi_sdk::resolve_input("ipAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_tags {
            pulumi_sdk::resolve_input_vec("ipTags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.linked_public_ip_address {
            pulumi_sdk::resolve_input("linkedPublicIPAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.migration_phase {
            pulumi_sdk::resolve_input("migrationPhase", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nat_gateway {
            pulumi_sdk::resolve_input("natGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_address_version {
            pulumi_sdk::resolve_input("publicIPAddressVersion", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_allocation_method {
            pulumi_sdk::resolve_input("publicIPAllocationMethod", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_prefix {
            pulumi_sdk::resolve_input("publicIPPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_address_name {
            pulumi_sdk::resolve_input("publicIpAddressName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.service_public_ip_address {
            pulumi_sdk::resolve_input("servicePublicIPAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ddos_settings: registered.outputs.get("ddosSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            delete_option: registered.outputs.get("deleteOption")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dns_settings: registered.outputs.get("dnsSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            idle_timeout_in_minutes: registered.outputs.get("idleTimeoutInMinutes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_address: registered.outputs.get("ipAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configuration: registered.outputs.get("ipConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_tags: registered.outputs.get("ipTags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            linked_public_ip_address: registered.outputs.get("linkedPublicIPAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            migration_phase: registered.outputs.get("migrationPhase")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            nat_gateway: registered.outputs.get("natGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_address_version: registered.outputs.get("publicIPAddressVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_allocation_method: registered.outputs.get("publicIPAllocationMethod")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_prefix: registered.outputs.get("publicIPPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_public_ip_address: registered.outputs.get("servicePublicIPAddress")
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
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
