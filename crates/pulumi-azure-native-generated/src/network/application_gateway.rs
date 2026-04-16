use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Application gateway resource.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2018-06-01, 2018-07-01, 2018-08-01, 2018-10-01, 2018-11-01, 2018-12-01, 2019-02-01, 2019-04-01, 2019-06-01, 2019-07-01, 2019-08-01, 2019-09-01, 2019-11-01, 2019-12-01, 2020-03-01, 2020-04-01, 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct ApplicationGatewayArgs {
    /// The name of the application gateway.
    pub application_gateway_name: Option<Input<String>>,
    /// Authentication certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub authentication_certificates: Option<Vec<Input<network::ApplicationGatewayAuthenticationCertificateArgs>>>,
    /// Autoscale Configuration.
    pub autoscale_configuration: Option<Input<network::ApplicationGatewayAutoscaleConfigurationArgs>>,
    /// Backend address pool of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_address_pools: Option<Vec<Input<network::ApplicationGatewayBackendAddressPoolArgs>>>,
    /// Backend http settings of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_http_settings_collection: Option<Vec<Input<network::ApplicationGatewayBackendHttpSettingsArgs>>>,
    /// Backend settings of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_settings_collection: Option<Vec<Input<network::ApplicationGatewayBackendSettingsArgs>>>,
    /// Custom error configurations of the application gateway resource.
    pub custom_error_configurations: Option<Vec<Input<network::ApplicationGatewayCustomErrorArgs>>>,
    /// Whether FIPS is enabled on the application gateway resource.
    pub enable_fips: Option<Input<bool>>,
    /// Whether HTTP2 is enabled on the application gateway resource.
    pub enable_http2: Option<Input<bool>>,
    /// Reference to the FirewallPolicy resource.
    pub firewall_policy: Option<Input<network::SubResourceArgs>>,
    /// If true, associates a firewall policy with an application gateway regardless whether the policy differs from the WAF Config.
    pub force_firewall_policy_association: Option<Input<bool>>,
    /// Frontend IP addresses of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ip_configurations: Option<Vec<Input<network::ApplicationGatewayFrontendIPConfigurationArgs>>>,
    /// Frontend ports of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ports: Option<Vec<Input<network::ApplicationGatewayFrontendPortArgs>>>,
    /// Subnets of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub gateway_ip_configurations: Option<Vec<Input<network::ApplicationGatewayIPConfigurationArgs>>>,
    /// Global Configuration.
    pub global_configuration: Option<Input<network::ApplicationGatewayGlobalConfigurationArgs>>,
    /// Http listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub http_listeners: Option<Vec<Input<network::ApplicationGatewayHttpListenerArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The identity of the application gateway, if configured.
    pub identity: Option<Input<network::ManagedServiceIdentityArgs>>,
    /// Listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub listeners: Option<Vec<Input<network::ApplicationGatewayListenerArgs>>>,
    /// Load distribution policies of the application gateway resource.
    pub load_distribution_policies: Option<Vec<Input<network::ApplicationGatewayLoadDistributionPolicyArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// PrivateLink configurations on application gateway.
    pub private_link_configurations: Option<Vec<Input<network::ApplicationGatewayPrivateLinkConfigurationArgs>>>,
    /// Probes of the application gateway resource.
    pub probes: Option<Vec<Input<network::ApplicationGatewayProbeArgs>>>,
    /// Redirect configurations of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub redirect_configurations: Option<Vec<Input<network::ApplicationGatewayRedirectConfigurationArgs>>>,
    /// Request routing rules of the application gateway resource.
    pub request_routing_rules: Option<Vec<Input<network::ApplicationGatewayRequestRoutingRuleArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Rewrite rules for the application gateway resource.
    pub rewrite_rule_sets: Option<Vec<Input<network::ApplicationGatewayRewriteRuleSetArgs>>>,
    /// Routing rules of the application gateway resource.
    pub routing_rules: Option<Vec<Input<network::ApplicationGatewayRoutingRuleArgs>>>,
    /// SKU of the application gateway resource.
    pub sku: Option<Input<network::ApplicationGatewaySkuArgs>>,
    /// SSL certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_certificates: Option<Vec<Input<network::ApplicationGatewaySslCertificateArgs>>>,
    /// SSL policy of the application gateway resource.
    pub ssl_policy: Option<Input<network::ApplicationGatewaySslPolicyArgs>>,
    /// SSL profiles of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_profiles: Option<Vec<Input<network::ApplicationGatewaySslProfileArgs>>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Trusted client certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub trusted_client_certificates: Option<Vec<Input<network::ApplicationGatewayTrustedClientCertificateArgs>>>,
    /// Trusted Root certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub trusted_root_certificates: Option<Vec<Input<network::ApplicationGatewayTrustedRootCertificateArgs>>>,
    /// URL path map of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub url_path_maps: Option<Vec<Input<network::ApplicationGatewayUrlPathMapArgs>>>,
    /// Web application firewall configuration.
    pub web_application_firewall_configuration: Option<Input<network::ApplicationGatewayWebApplicationFirewallConfigurationArgs>>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Option<Vec<Input<String>>>,
}

/// Application gateway resource.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2018-06-01, 2018-07-01, 2018-08-01, 2018-10-01, 2018-11-01, 2018-12-01, 2019-02-01, 2019-04-01, 2019-06-01, 2019-07-01, 2019-08-01, 2019-09-01, 2019-11-01, 2019-12-01, 2020-03-01, 2020-04-01, 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
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
    /// Backend settings of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub backend_settings_collection: Output<serde_json::Value>,
    /// Custom error configurations of the application gateway resource.
    pub custom_error_configurations: Output<serde_json::Value>,
    /// The default predefined SSL Policy applied on the application gateway resource.
    pub default_predefined_ssl_policy: Output<serde_json::Value>,
    /// Whether FIPS is enabled on the application gateway resource.
    pub enable_fips: Output<serde_json::Value>,
    /// Whether HTTP2 is enabled on the application gateway resource.
    pub enable_http2: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Reference to the FirewallPolicy resource.
    pub firewall_policy: Output<serde_json::Value>,
    /// If true, associates a firewall policy with an application gateway regardless whether the policy differs from the WAF Config.
    pub force_firewall_policy_association: Output<serde_json::Value>,
    /// Frontend IP addresses of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ip_configurations: Output<serde_json::Value>,
    /// Frontend ports of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub frontend_ports: Output<serde_json::Value>,
    /// Subnets of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub gateway_ip_configurations: Output<serde_json::Value>,
    /// Global Configuration.
    pub global_configuration: Output<serde_json::Value>,
    /// Http listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub http_listeners: Output<serde_json::Value>,
    /// The identity of the application gateway, if configured.
    pub identity: Output<serde_json::Value>,
    /// Listeners of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub listeners: Output<serde_json::Value>,
    /// Load distribution policies of the application gateway resource.
    pub load_distribution_policies: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// Operational state of the application gateway resource.
    pub operational_state: Output<serde_json::Value>,
    /// Private Endpoint connections on application gateway.
    pub private_endpoint_connections: Output<serde_json::Value>,
    /// PrivateLink configurations on application gateway.
    pub private_link_configurations: Output<serde_json::Value>,
    /// Probes of the application gateway resource.
    pub probes: Output<serde_json::Value>,
    /// The provisioning state of the application gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Redirect configurations of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub redirect_configurations: Output<serde_json::Value>,
    /// Request routing rules of the application gateway resource.
    pub request_routing_rules: Output<serde_json::Value>,
    /// The resource GUID property of the application gateway resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Rewrite rules for the application gateway resource.
    pub rewrite_rule_sets: Output<serde_json::Value>,
    /// Routing rules of the application gateway resource.
    pub routing_rules: Output<serde_json::Value>,
    /// SKU of the application gateway resource.
    pub sku: Output<serde_json::Value>,
    /// SSL certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_certificates: Output<serde_json::Value>,
    /// SSL policy of the application gateway resource.
    pub ssl_policy: Output<serde_json::Value>,
    /// SSL profiles of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub ssl_profiles: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Trusted client certificates of the application gateway resource. For default limits, see [Application Gateway limits](https://docs.microsoft.com/azure/azure-subscription-service-limits#application-gateway-limits).
    pub trusted_client_certificates: Output<serde_json::Value>,
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
    const TYPE_TOKEN: &'static str = "azure-native:network:ApplicationGateway";

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
        if let Some(v) = args.backend_settings_collection {
            pulumi_sdk::resolve_input_vec("backendSettingsCollection", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.force_firewall_policy_association {
            pulumi_sdk::resolve_input("forceFirewallPolicyAssociation", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.global_configuration {
            pulumi_sdk::resolve_input("globalConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.listeners {
            pulumi_sdk::resolve_input_vec("listeners", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.load_distribution_policies {
            pulumi_sdk::resolve_input_vec("loadDistributionPolicies", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_link_configurations {
            pulumi_sdk::resolve_input_vec("privateLinkConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.probes {
            pulumi_sdk::resolve_input_vec("probes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.redirect_configurations {
            pulumi_sdk::resolve_input_vec("redirectConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.request_routing_rules {
            pulumi_sdk::resolve_input_vec("requestRoutingRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.rewrite_rule_sets {
            pulumi_sdk::resolve_input_vec("rewriteRuleSets", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.routing_rules {
            pulumi_sdk::resolve_input_vec("routingRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.ssl_profiles {
            pulumi_sdk::resolve_input_vec("sslProfiles", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.trusted_client_certificates {
            pulumi_sdk::resolve_input_vec("trustedClientCertificates", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            backend_settings_collection: registered.outputs.get("backendSettingsCollection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_error_configurations: registered.outputs.get("customErrorConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            default_predefined_ssl_policy: registered.outputs.get("defaultPredefinedSslPolicy")
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
            force_firewall_policy_association: registered.outputs.get("forceFirewallPolicyAssociation")
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
            global_configuration: registered.outputs.get("globalConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            http_listeners: registered.outputs.get("httpListeners")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            listeners: registered.outputs.get("listeners")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            load_distribution_policies: registered.outputs.get("loadDistributionPolicies")
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
            private_endpoint_connections: registered.outputs.get("privateEndpointConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_link_configurations: registered.outputs.get("privateLinkConfigurations")
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
            routing_rules: registered.outputs.get("routingRules")
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
            ssl_profiles: registered.outputs.get("sslProfiles")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            trusted_client_certificates: registered.outputs.get("trustedClientCertificates")
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
