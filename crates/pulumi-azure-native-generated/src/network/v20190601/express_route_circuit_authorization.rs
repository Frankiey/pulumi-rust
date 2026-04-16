use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Authorization in an ExpressRouteCircuit resource.
pub struct ExpressRouteCircuitAuthorizationArgs {
    /// The authorization key.
    pub authorization_key: Option<Input<String>>,
    /// The name of the authorization.
    pub authorization_name: Option<Input<String>>,
    /// The authorization use status.
    pub authorization_use_status: Option<Input<serde_json::Value>>,
    /// The name of the express route circuit.
    pub circuit_name: Input<String>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Gets name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// Authorization in an ExpressRouteCircuit resource.
pub struct ExpressRouteCircuitAuthorization {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The authorization key.
    pub authorization_key: Output<serde_json::Value>,
    /// The authorization use status.
    pub authorization_use_status: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Gets name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Output<serde_json::Value>,
    /// Type of the resource.
    pub type_: Output<serde_json::Value>,
}

impl ExpressRouteCircuitAuthorization {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190601:ExpressRouteCircuitAuthorization";

    /// Create a new ExpressRouteCircuitAuthorization resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ExpressRouteCircuitAuthorizationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.authorization_key {
            pulumi_sdk::resolve_input("authorizationKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.authorization_name {
            pulumi_sdk::resolve_input("authorizationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.authorization_use_status {
            pulumi_sdk::resolve_input("authorizationUseStatus", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("circuitName", args.circuit_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.provisioning_state {
            pulumi_sdk::resolve_input("provisioningState", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            authorization_key: registered.outputs.get("authorizationKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            authorization_use_status: registered.outputs.get("authorizationUseStatus")
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
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
