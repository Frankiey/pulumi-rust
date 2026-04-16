use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The Private Endpoint Connection resource.
pub struct PrivateEndpointConnectionArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Input<String>,
    /// The name of the private endpoint connection associated with the Azure resource
    pub private_endpoint_connection_name: Option<Input<String>>,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Input<storage::v20230101::PrivateLinkServiceConnectionStateArgs>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
}

/// The Private Endpoint Connection resource.
pub struct PrivateEndpointConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// The resource of private end point.
    pub private_endpoint: Output<serde_json::Value>,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Output<serde_json::Value>,
    /// The provisioning state of the private endpoint connection resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl PrivateEndpointConnection {
    const TYPE_TOKEN: &'static str = "azure-native:storage/v20230101:PrivateEndpointConnection";

    /// Create a new PrivateEndpointConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PrivateEndpointConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.private_endpoint_connection_name {
            pulumi_sdk::resolve_input("privateEndpointConnectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("privateLinkServiceConnectionState", args.private_link_service_connection_state, &mut inputs, &mut deps, &mut prop_deps).await;
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
