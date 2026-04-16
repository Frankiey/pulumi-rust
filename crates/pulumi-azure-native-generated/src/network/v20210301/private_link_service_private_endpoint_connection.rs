use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// PrivateEndpointConnection resource.
pub struct PrivateLinkServicePrivateEndpointConnectionArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the private end point connection.
    pub pe_connection_name: Option<Input<String>>,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Option<Input<network::v20210301::PrivateLinkServiceConnectionStateArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the private link service.
    pub service_name: Input<String>,
}

/// PrivateEndpointConnection resource.
pub struct PrivateLinkServicePrivateEndpointConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The consumer link id.
    pub link_identifier: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The resource of private end point.
    pub private_endpoint: Output<serde_json::Value>,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Output<serde_json::Value>,
    /// The provisioning state of the private endpoint connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource type.
    pub type_: Output<serde_json::Value>,
}

impl PrivateLinkServicePrivateEndpointConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20210301:PrivateLinkServicePrivateEndpointConnection";

    /// Create a new PrivateLinkServicePrivateEndpointConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PrivateLinkServicePrivateEndpointConnectionArgs,
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
        if let Some(v) = args.pe_connection_name {
            pulumi_sdk::resolve_input("peConnectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_link_service_connection_state {
            pulumi_sdk::resolve_input("privateLinkServiceConnectionState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("serviceName", args.service_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            link_identifier: registered.outputs.get("linkIdentifier")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_endpoint: registered.outputs.get("privateEndpoint")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_link_service_connection_state: registered.outputs.get("privateLinkServiceConnectionState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
