use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieve protection policy with specified name within a resource group.
#[derive(Default)]
pub struct GetWebApplicationFirewallPolicyArgs {
    /// The name of the policy.
    pub policy_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetWebApplicationFirewallPolicyResult {
    /// A collection of references to application gateways.
    pub application_gateways: Vec<network::v20230901::ApplicationGatewayResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The custom rules inside the policy.
    pub custom_rules: Option<Vec<network::v20230901::WebApplicationFirewallCustomRuleResponse>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// A collection of references to application gateway http listeners.
    pub http_listeners: Vec<network::v20230901::SubResourceResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Describes the managedRules structure.
    pub managed_rules: network::v20230901::ManagedRulesDefinitionResponse,
    /// Resource name.
    pub name: String,
    /// A collection of references to application gateway path rules.
    pub path_based_rules: Vec<network::v20230901::SubResourceResponse>,
    /// The PolicySettings for policy.
    pub policy_settings: Option<network::v20230901::PolicySettingsResponse>,
    /// The provisioning state of the web application firewall policy resource.
    pub provisioning_state: String,
    /// Resource status of the policy.
    pub resource_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Retrieve protection policy with specified name within a resource group.
pub async fn get_web_application_firewall_policy(
    ctx: &Context,
    args: GetWebApplicationFirewallPolicyArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetWebApplicationFirewallPolicyResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("policyName".to_string(), json!(args.policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230901:getWebApplicationFirewallPolicy", invoke_args, &opts).await?;

    Ok(GetWebApplicationFirewallPolicyResult {
        application_gateways: serde_json::from_value(result.fields.get("applicationGateways").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        custom_rules: result.fields.get("customRules").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        http_listeners: serde_json::from_value(result.fields.get("httpListeners").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        managed_rules: serde_json::from_value(result.fields.get("managedRules").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        path_based_rules: serde_json::from_value(result.fields.get("pathBasedRules").cloned().unwrap_or_default())?
            ,
        policy_settings: result.fields.get("policySettings").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_state: serde_json::from_value(result.fields.get("resourceState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
