use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Private dns zone group resource.
pub struct PrivateDnsZoneGroupArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// A collection of private dns zone configurations of the private dns zone group.
    pub private_dns_zone_configs: Option<Vec<Input<network::v20200601::PrivateDnsZoneConfigArgs>>>,
    /// The name of the private dns zone group.
    pub private_dns_zone_group_name: Option<Input<String>>,
    /// The name of the private endpoint.
    pub private_endpoint_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// Private dns zone group resource.
pub struct PrivateDnsZoneGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// A collection of private dns zone configurations of the private dns zone group.
    pub private_dns_zone_configs: Output<serde_json::Value>,
    /// The provisioning state of the private dns zone group resource.
    pub provisioning_state: Output<serde_json::Value>,
}

impl PrivateDnsZoneGroup {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20200601:PrivateDnsZoneGroup";

    /// Create a new PrivateDnsZoneGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PrivateDnsZoneGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_dns_zone_configs {
            pulumi_sdk::resolve_input_vec("privateDnsZoneConfigs", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_dns_zone_group_name {
            pulumi_sdk::resolve_input("privateDnsZoneGroupName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("privateEndpointName", args.private_endpoint_name, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_dns_zone_configs: registered.outputs.get("privateDnsZoneConfigs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
