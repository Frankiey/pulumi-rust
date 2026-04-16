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
    /// Authentication certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub authentication_certificates: Option<Vec<network::v20230201::ApplicationGatewayAuthenticationCertificateResponse>>,
    /// Autoscale Configuration.
    pub autoscale_configuration: Option<network::v20230201::ApplicationGatewayAutoscaleConfigurationResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Backend address pool of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_address_pools: Option<Vec<network::v20230201::ApplicationGatewayBackendAddressPoolResponse>>,
    /// Backend http settings of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_http_settings_collection: Option<Vec<network::v20230201::ApplicationGatewayBackendHttpSettingsResponse>>,
    /// Backend settings of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_settings_collection: Option<Vec<network::v20230201::ApplicationGatewayBackendSettingsResponse>>,
    /// Custom error configurations of the application gateway resource.
    pub custom_error_configurations: Option<Vec<network::v20230201::ApplicationGatewayCustomErrorResponse>>,
    /// The default predefined SSL Policy applied on the application gateway resource.
    pub default_predefined_ssl_policy: String,
    /// Whether FIPS is enabled on the application gateway resource.
    pub enable_fips: Option<bool>,
    /// Whether HTTP2 is enabled on the application gateway resource.
    pub enable_http2: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Reference to the FirewallPolicy resource.
    pub firewall_policy: Option<network::v20230201::SubResourceResponse>,
    /// If true, associates a firewall policy with an application gateway regardless whether the policy differs from the WAF Config.
    pub force_firewall_policy_association: Option<bool>,
    /// Frontend IP addresses of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ip_configurations: Option<Vec<network::v20230201::ApplicationGatewayFrontendIPConfigurationResponse>>,
    /// Frontend ports of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ports: Option<Vec<network::v20230201::ApplicationGatewayFrontendPortResponse>>,
    /// Subnets of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub gateway_ip_configurations: Option<Vec<network::v20230201::ApplicationGatewayIPConfigurationResponse>>,
    /// Global Configuration.
    pub global_configuration: Option<network::v20230201::ApplicationGatewayGlobalConfigurationResponse>,
    /// Http listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub http_listeners: Option<Vec<network::v20230201::ApplicationGatewayHttpListenerResponse>>,
    /// Resource ID.
    pub id: Option<String>,
    /// The identity of the application gateway, if configured.
    pub identity: Option<network::v20230201::ManagedServiceIdentityResponse>,
    /// Listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub listeners: Option<Vec<network::v20230201::ApplicationGatewayListenerResponse>>,
    /// Load distribution policies of the application gateway resource.
    pub load_distribution_policies: Option<Vec<network::v20230201::ApplicationGatewayLoadDistributionPolicyResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Operational state of the application gateway resource.
    pub operational_state: String,
    /// Private Endpoint connections on application gateway.
    pub private_endpoint_connections: Vec<network::v20230201::ApplicationGatewayPrivateEndpointConnectionResponse>,
    /// PrivateLink configurations on application gateway.
    pub private_link_configurations: Option<Vec<network::v20230201::ApplicationGatewayPrivateLinkConfigurationResponse>>,
    /// Probes of the application gateway resource.
    pub probes: Option<Vec<network::v20230201::ApplicationGatewayProbeResponse>>,
    /// The provisioning state of the application gateway resource.
    pub provisioning_state: String,
    /// Redirect configurations of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub redirect_configurations: Option<Vec<network::v20230201::ApplicationGatewayRedirectConfigurationResponse>>,
    /// Request routing rules of the application gateway resource.
    pub request_routing_rules: Option<Vec<network::v20230201::ApplicationGatewayRequestRoutingRuleResponse>>,
    /// The resource GUID property of the application gateway resource.
    pub resource_guid: String,
    /// Rewrite rules for the application gateway resource.
    pub rewrite_rule_sets: Option<Vec<network::v20230201::ApplicationGatewayRewriteRuleSetResponse>>,
    /// Routing rules of the application gateway resource.
    pub routing_rules: Option<Vec<network::v20230201::ApplicationGatewayRoutingRuleResponse>>,
    /// SKU of the application gateway resource.
    pub sku: Option<network::v20230201::ApplicationGatewaySkuResponse>,
    /// SSL certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_certificates: Option<Vec<network::v20230201::ApplicationGatewaySslCertificateResponse>>,
    /// SSL policy of the application gateway resource.
    pub ssl_policy: Option<network::v20230201::ApplicationGatewaySslPolicyResponse>,
    /// SSL profiles of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_profiles: Option<Vec<network::v20230201::ApplicationGatewaySslProfileResponse>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Trusted client certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub trusted_client_certificates: Option<Vec<network::v20230201::ApplicationGatewayTrustedClientCertificateResponse>>,
    /// Trusted Root certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub trusted_root_certificates: Option<Vec<network::v20230201::ApplicationGatewayTrustedRootCertificateResponse>>,
    /// Resource type.
    pub type_: String,
    /// URL path map of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub url_path_maps: Option<Vec<network::v20230201::ApplicationGatewayUrlPathMapResponse>>,
    /// Web application firewall configuration.
    pub web_application_firewall_configuration: Option<network::v20230201::ApplicationGatewayWebApplicationFirewallConfigurationResponse>,
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
    let result = ctx.invoke("azure-native:network/v20230201:getApplicationGateway", invoke_args, &opts).await?;

    Ok(GetApplicationGatewayResult {
        authentication_certificates: result.fields.get("authenticationCertificates").cloned().map(serde_json::from_value).transpose()?,
        autoscale_configuration: result.fields.get("autoscaleConfiguration").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        backend_address_pools: result.fields.get("backendAddressPools").cloned().map(serde_json::from_value).transpose()?,
        backend_http_settings_collection: result.fields.get("backendHttpSettingsCollection").cloned().map(serde_json::from_value).transpose()?,
        backend_settings_collection: result.fields.get("backendSettingsCollection").cloned().map(serde_json::from_value).transpose()?,
        custom_error_configurations: result.fields.get("customErrorConfigurations").cloned().map(serde_json::from_value).transpose()?,
        default_predefined_ssl_policy: serde_json::from_value(result.fields.get("defaultPredefinedSslPolicy").cloned().unwrap_or_default())?
            ,
        enable_fips: result.fields.get("enableFips").cloned().map(serde_json::from_value).transpose()?,
        enable_http2: result.fields.get("enableHttp2").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        firewall_policy: result.fields.get("firewallPolicy").cloned().map(serde_json::from_value).transpose()?,
        force_firewall_policy_association: result.fields.get("forceFirewallPolicyAssociation").cloned().map(serde_json::from_value).transpose()?,
        frontend_ip_configurations: result.fields.get("frontendIPConfigurations").cloned().map(serde_json::from_value).transpose()?,
        frontend_ports: result.fields.get("frontendPorts").cloned().map(serde_json::from_value).transpose()?,
        gateway_ip_configurations: result.fields.get("gatewayIPConfigurations").cloned().map(serde_json::from_value).transpose()?,
        global_configuration: result.fields.get("globalConfiguration").cloned().map(serde_json::from_value).transpose()?,
        http_listeners: result.fields.get("httpListeners").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        listeners: result.fields.get("listeners").cloned().map(serde_json::from_value).transpose()?,
        load_distribution_policies: result.fields.get("loadDistributionPolicies").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        operational_state: serde_json::from_value(result.fields.get("operationalState").cloned().unwrap_or_default())?
            ,
        private_endpoint_connections: serde_json::from_value(result.fields.get("privateEndpointConnections").cloned().unwrap_or_default())?
            ,
        private_link_configurations: result.fields.get("privateLinkConfigurations").cloned().map(serde_json::from_value).transpose()?,
        probes: result.fields.get("probes").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        redirect_configurations: result.fields.get("redirectConfigurations").cloned().map(serde_json::from_value).transpose()?,
        request_routing_rules: result.fields.get("requestRoutingRules").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        rewrite_rule_sets: result.fields.get("rewriteRuleSets").cloned().map(serde_json::from_value).transpose()?,
        routing_rules: result.fields.get("routingRules").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        ssl_certificates: result.fields.get("sslCertificates").cloned().map(serde_json::from_value).transpose()?,
        ssl_policy: result.fields.get("sslPolicy").cloned().map(serde_json::from_value).transpose()?,
        ssl_profiles: result.fields.get("sslProfiles").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        trusted_client_certificates: result.fields.get("trustedClientCertificates").cloned().map(serde_json::from_value).transpose()?,
        trusted_root_certificates: result.fields.get("trustedRootCertificates").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        url_path_maps: result.fields.get("urlPathMaps").cloned().map(serde_json::from_value).transpose()?,
        web_application_firewall_configuration: result.fields.get("webApplicationFirewallConfiguration").cloned().map(serde_json::from_value).transpose()?,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
