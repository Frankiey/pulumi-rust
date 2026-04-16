use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified application gateway.
#[derive(Default)]
pub struct GetApplicationGatewayArgs {
    /// The name of the application gateway.
    pub application_gateway_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetApplicationGatewayResult {
    /// Authentication certificates of the application gateway resource.
    pub authentication_certificates: Option<Vec<network::v20180701::ApplicationGatewayAuthenticationCertificateResponse>>,
    /// Autoscale Configuration.
    pub autoscale_configuration: Option<network::v20180701::ApplicationGatewayAutoscaleConfigurationResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Backend address pool of the application gateway resource.
    pub backend_address_pools: Option<Vec<network::v20180701::ApplicationGatewayBackendAddressPoolResponse>>,
    /// Backend http settings of the application gateway resource.
    pub backend_http_settings_collection: Option<Vec<network::v20180701::ApplicationGatewayBackendHttpSettingsResponse>>,
    /// Whether FIPS is enabled on the application gateway resource.
    pub enable_fips: Option<bool>,
    /// Whether HTTP2 is enabled on the application gateway resource.
    pub enable_http2: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Frontend IP addresses of the application gateway resource.
    pub frontend_ip_configurations: Option<Vec<network::v20180701::ApplicationGatewayFrontendIPConfigurationResponse>>,
    /// Frontend ports of the application gateway resource.
    pub frontend_ports: Option<Vec<network::v20180701::ApplicationGatewayFrontendPortResponse>>,
    /// Subnets of application the gateway resource.
    pub gateway_ip_configurations: Option<Vec<network::v20180701::ApplicationGatewayIPConfigurationResponse>>,
    /// Http listeners of the application gateway resource.
    pub http_listeners: Option<Vec<network::v20180701::ApplicationGatewayHttpListenerResponse>>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Operational state of the application gateway resource.
    pub operational_state: String,
    /// Probes of the application gateway resource.
    pub probes: Option<Vec<network::v20180701::ApplicationGatewayProbeResponse>>,
    /// Provisioning state of the application gateway resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// Redirect configurations of the application gateway resource.
    pub redirect_configurations: Option<Vec<network::v20180701::ApplicationGatewayRedirectConfigurationResponse>>,
    /// Request routing rules of the application gateway resource.
    pub request_routing_rules: Option<Vec<network::v20180701::ApplicationGatewayRequestRoutingRuleResponse>>,
    /// Resource GUID property of the application gateway resource.
    pub resource_guid: Option<String>,
    /// SKU of the application gateway resource.
    pub sku: Option<network::v20180701::ApplicationGatewaySkuResponse>,
    /// SSL certificates of the application gateway resource.
    pub ssl_certificates: Option<Vec<network::v20180701::ApplicationGatewaySslCertificateResponse>>,
    /// SSL policy of the application gateway resource.
    pub ssl_policy: Option<network::v20180701::ApplicationGatewaySslPolicyResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// URL path map of the application gateway resource.
    pub url_path_maps: Option<Vec<network::v20180701::ApplicationGatewayUrlPathMapResponse>>,
    /// Web application firewall configuration.
    pub web_application_firewall_configuration: Option<network::v20180701::ApplicationGatewayWebApplicationFirewallConfigurationResponse>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Option<Vec<String>>,
}

/// Gets the specified application gateway.
pub async fn get_application_gateway(
    ctx: &Context,
    args: GetApplicationGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetApplicationGatewayResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("applicationGatewayName".to_string(), json!(args.application_gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20180701:getApplicationGateway", invoke_args, &opts).await?;

    Ok(GetApplicationGatewayResult {
        authentication_certificates: result.fields.get("authenticationCertificates").cloned().map(serde_json::from_value).transpose()?,
        autoscale_configuration: result.fields.get("autoscaleConfiguration").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        backend_address_pools: result.fields.get("backendAddressPools").cloned().map(serde_json::from_value).transpose()?,
        backend_http_settings_collection: result.fields.get("backendHttpSettingsCollection").cloned().map(serde_json::from_value).transpose()?,
        enable_fips: result.fields.get("enableFips").cloned().map(serde_json::from_value).transpose()?,
        enable_http2: result.fields.get("enableHttp2").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        frontend_ip_configurations: result.fields.get("frontendIPConfigurations").cloned().map(serde_json::from_value).transpose()?,
        frontend_ports: result.fields.get("frontendPorts").cloned().map(serde_json::from_value).transpose()?,
        gateway_ip_configurations: result.fields.get("gatewayIPConfigurations").cloned().map(serde_json::from_value).transpose()?,
        http_listeners: result.fields.get("httpListeners").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        operational_state: serde_json::from_value(result.fields.get("operationalState").cloned().unwrap_or_default())?
            ,
        probes: result.fields.get("probes").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        redirect_configurations: result.fields.get("redirectConfigurations").cloned().map(serde_json::from_value).transpose()?,
        request_routing_rules: result.fields.get("requestRoutingRules").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: result.fields.get("resourceGuid").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        ssl_certificates: result.fields.get("sslCertificates").cloned().map(serde_json::from_value).transpose()?,
        ssl_policy: result.fields.get("sslPolicy").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        url_path_maps: result.fields.get("urlPathMaps").cloned().map(serde_json::from_value).transpose()?,
        web_application_firewall_configuration: result.fields.get("webApplicationFirewallConfiguration").cloned().map(serde_json::from_value).transpose()?,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
