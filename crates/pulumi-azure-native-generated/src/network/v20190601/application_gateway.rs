use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Application gateway resource.
pub struct ApplicationGatewayArgs {
    /// The name of the application gateway.
    pub application_gateway_name: Option<Input<String>>,
    /// Authentication certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub authentication_certificates: Option<Vec<Input<network::v20190601::ApplicationGatewayAuthenticationCertificateArgs>>>,
    /// Autoscale Configuration.
    pub autoscale_configuration: Option<Input<network::v20190601::ApplicationGatewayAutoscaleConfigurationArgs>>,
    /// Backend address pool of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_address_pools: Option<Vec<Input<network::v20190601::ApplicationGatewayBackendAddressPoolArgs>>>,
    /// Backend http settings of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_http_settings_collection: Option<Vec<Input<network::v20190601::ApplicationGatewayBackendHttpSettingsArgs>>>,
    /// Custom error configurations of the application gateway resource.
    pub custom_error_configurations: Option<Vec<Input<network::v20190601::ApplicationGatewayCustomErrorArgs>>>,
    /// Whether FIPS is enabled on the application gateway resource.
    pub enable_fips: Option<Input<bool>>,
    /// Whether HTTP2 is enabled on the application gateway resource.
    pub enable_http2: Option<Input<bool>>,
    /// Reference of the FirewallPolicy resource.
    pub firewall_policy: Option<Input<network::v20190601::SubResourceArgs>>,
    /// Frontend IP addresses of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ip_configurations: Option<Vec<Input<network::v20190601::ApplicationGatewayFrontendIPConfigurationArgs>>>,
    /// Frontend ports of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ports: Option<Vec<Input<network::v20190601::ApplicationGatewayFrontendPortArgs>>>,
    /// Subnets of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub gateway_ip_configurations: Option<Vec<Input<network::v20190601::ApplicationGatewayIPConfigurationArgs>>>,
    /// Http listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub http_listeners: Option<Vec<Input<network::v20190601::ApplicationGatewayHttpListenerArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The identity of the application gateway, if configured.
    pub identity: Option<Input<network::v20190601::ManagedServiceIdentityArgs>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// Probes of the application gateway resource.
    pub probes: Option<Vec<Input<network::v20190601::ApplicationGatewayProbeArgs>>>,
    /// Provisioning state of the application gateway resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<Input<String>>,
    /// Redirect configurations of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub redirect_configurations: Option<Vec<Input<network::v20190601::ApplicationGatewayRedirectConfigurationArgs>>>,
    /// Request routing rules of the application gateway resource.
    pub request_routing_rules: Option<Vec<Input<network::v20190601::ApplicationGatewayRequestRoutingRuleArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource GUID property of the application gateway resource.
    pub resource_guid: Option<Input<String>>,
    /// Rewrite rules for the application gateway resource.
    pub rewrite_rule_sets: Option<Vec<Input<network::v20190601::ApplicationGatewayRewriteRuleSetArgs>>>,
    /// SKU of the application gateway resource.
    pub sku: Option<Input<network::v20190601::ApplicationGatewaySkuArgs>>,
    /// SSL certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_certificates: Option<Vec<Input<network::v20190601::ApplicationGatewaySslCertificateArgs>>>,
    /// SSL policy of the application gateway resource.
    pub ssl_policy: Option<Input<network::v20190601::ApplicationGatewaySslPolicyArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Trusted Root certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub trusted_root_certificates: Option<Vec<Input<network::v20190601::ApplicationGatewayTrustedRootCertificateArgs>>>,
    /// URL path map of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub url_path_maps: Option<Vec<Input<network::v20190601::ApplicationGatewayUrlPathMapArgs>>>,
    /// Web application firewall configuration.
    pub web_application_firewall_configuration: Option<Input<network::v20190601::ApplicationGatewayWebApplicationFirewallConfigurationArgs>>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Option<Vec<Input<String>>>,
}

/// Application gateway resource.
pub struct ApplicationGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Authentication certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub authentication_certificates: Output<serde_json::Value>,
    /// Autoscale Configuration.
    pub autoscale_configuration: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Backend address pool of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_address_pools: Output<serde_json::Value>,
    /// Backend http settings of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_http_settings_collection: Output<serde_json::Value>,
    /// Custom error configurations of the application gateway resource.
    pub custom_error_configurations: Output<serde_json::Value>,
    /// Whether FIPS is enabled on the application gateway resource.
    pub enable_fips: Output<serde_json::Value>,
    /// Whether HTTP2 is enabled on the application gateway resource.
    pub enable_http2: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Reference of the FirewallPolicy resource.
    pub firewall_policy: Output<serde_json::Value>,
    /// Frontend IP addresses of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ip_configurations: Output<serde_json::Value>,
    /// Frontend ports of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ports: Output<serde_json::Value>,
    /// Subnets of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub gateway_ip_configurations: Output<serde_json::Value>,
    /// Http listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub http_listeners: Output<serde_json::Value>,
    /// The identity of the application gateway, if configured.
    pub identity: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// Operational state of the application gateway resource.
    pub operational_state: Output<serde_json::Value>,
    /// Probes of the application gateway resource.
    pub probes: Output<serde_json::Value>,
    /// Provisioning state of the application gateway resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Output<serde_json::Value>,
    /// Redirect configurations of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub redirect_configurations: Output<serde_json::Value>,
    /// Request routing rules of the application gateway resource.
    pub request_routing_rules: Output<serde_json::Value>,
    /// Resource GUID property of the application gateway resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Rewrite rules for the application gateway resource.
    pub rewrite_rule_sets: Output<serde_json::Value>,
    /// SKU of the application gateway resource.
    pub sku: Output<serde_json::Value>,
    /// SSL certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_certificates: Output<serde_json::Value>,
    /// SSL policy of the application gateway resource.
    pub ssl_policy: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Trusted Root certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub trusted_root_certificates: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// URL path map of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub url_path_maps: Output<serde_json::Value>,
    /// Web application firewall configuration.
    pub web_application_firewall_configuration: Output<serde_json::Value>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Output<serde_json::Value>,
}

impl ApplicationGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190601:ApplicationGateway";

    /// Create a new ApplicationGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ApplicationGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.application_gateway_name {
            pulumi_sdk::resolve_input("applicationGatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.authentication_certificates {
            pulumi_sdk::resolve_input_vec("authenticationCertificates", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.autoscale_configuration {
            pulumi_sdk::resolve_input("autoscaleConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.backend_address_pools {
            pulumi_sdk::resolve_input_vec("backendAddressPools", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.backend_http_settings_collection {
            pulumi_sdk::resolve_input_vec("backendHttpSettingsCollection", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.custom_error_configurations {
            pulumi_sdk::resolve_input_vec("customErrorConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_fips {
            pulumi_sdk::resolve_input("enableFips", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_http2 {
            pulumi_sdk::resolve_input("enableHttp2", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.firewall_policy {
            pulumi_sdk::resolve_input("firewallPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.frontend_ip_configurations {
            pulumi_sdk::resolve_input_vec("frontendIPConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.frontend_ports {
            pulumi_sdk::resolve_input_vec("frontendPorts", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_ip_configurations {
            pulumi_sdk::resolve_input_vec("gatewayIPConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.http_listeners {
            pulumi_sdk::resolve_input_vec("httpListeners", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.probes {
            pulumi_sdk::resolve_input_vec("probes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.provisioning_state {
            pulumi_sdk::resolve_input("provisioningState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.redirect_configurations {
            pulumi_sdk::resolve_input_vec("redirectConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.request_routing_rules {
            pulumi_sdk::resolve_input_vec("requestRoutingRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.resource_guid {
            pulumi_sdk::resolve_input("resourceGuid", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.rewrite_rule_sets {
            pulumi_sdk::resolve_input_vec("rewriteRuleSets", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ssl_certificates {
            pulumi_sdk::resolve_input_vec("sslCertificates", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ssl_policy {
            pulumi_sdk::resolve_input("sslPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.trusted_root_certificates {
            pulumi_sdk::resolve_input_vec("trustedRootCertificates", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.url_path_maps {
            pulumi_sdk::resolve_input_vec("urlPathMaps", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.web_application_firewall_configuration {
            pulumi_sdk::resolve_input("webApplicationFirewallConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            authentication_certificates: registered.outputs.get("authenticationCertificates")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            autoscale_configuration: registered.outputs.get("autoscaleConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            backend_address_pools: registered.outputs.get("backendAddressPools")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            backend_http_settings_collection: registered.outputs.get("backendHttpSettingsCollection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_error_configurations: registered.outputs.get("customErrorConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_fips: registered.outputs.get("enableFips")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_http2: registered.outputs.get("enableHttp2")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            firewall_policy: registered.outputs.get("firewallPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            frontend_ip_configurations: registered.outputs.get("frontendIPConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            frontend_ports: registered.outputs.get("frontendPorts")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            gateway_ip_configurations: registered.outputs.get("gatewayIPConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            http_listeners: registered.outputs.get("httpListeners")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            operational_state: registered.outputs.get("operationalState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            probes: registered.outputs.get("probes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            redirect_configurations: registered.outputs.get("redirectConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            request_routing_rules: registered.outputs.get("requestRoutingRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            rewrite_rule_sets: registered.outputs.get("rewriteRuleSets")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ssl_certificates: registered.outputs.get("sslCertificates")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ssl_policy: registered.outputs.get("sslPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            trusted_root_certificates: registered.outputs.get("trustedRootCertificates")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            url_path_maps: registered.outputs.get("urlPathMaps")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            web_application_firewall_configuration: registered.outputs.get("webApplicationFirewallConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
