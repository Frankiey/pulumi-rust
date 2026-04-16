use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Private Endpoint connection on an application gateway.
pub struct ApplicationGatewayPrivateEndpointConnectionArgs {
    /// The name of the application gateway.
    pub application_gateway_name: Input<String>,
    /// The name of the application gateway private endpoint connection.
    pub connection_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Name of the private endpoint connection on an application gateway.
    pub name: Option<Input<String>>,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Option<Input<network::v20240101::PrivateLinkServiceConnectionStateArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// Private Endpoint connection on an application gateway.
pub struct ApplicationGatewayPrivateEndpointConnection {
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
    /// Name of the private endpoint connection on an application gateway.
    pub name: Output<serde_json::Value>,
    /// The resource of private end point.
    pub private_endpoint: Output<serde_json::Value>,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Output<serde_json::Value>,
    /// The provisioning state of the application gateway private endpoint connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Type of the resource.
    pub type_: Output<serde_json::Value>,
}

impl ApplicationGatewayPrivateEndpointConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240101:ApplicationGatewayPrivateEndpointConnection";

    /// Create a new ApplicationGatewayPrivateEndpointConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ApplicationGatewayPrivateEndpointConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("applicationGatewayName", args.application_gateway_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.connection_name {
            pulumi_sdk::resolve_input("connectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_link_service_connection_state {
            pulumi_sdk::resolve_input("privateLinkServiceConnectionState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
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
