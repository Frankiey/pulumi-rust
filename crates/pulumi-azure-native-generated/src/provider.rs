use pulumi_sdk::{Context, Input, Output, ProviderResource, ResourceOptions, Result};
use std::collections::HashMap;

/// Configuration arguments for the provider resource.
#[derive(Default)]
pub struct ProviderArgs {
    /// Any additional Tenant IDs which should be used for authentication.
    pub auxiliary_tenant_ids: Option<Vec<Input<String>>>,
    /// The password associated with the Client Certificate. For use when authenticating as a Service Principal using a Client Certificate
    pub client_certificate_password: Option<Input<String>>,
    /// The path to the Client Certificate associated with the Service Principal for use when authenticating as a Service Principal using a Client Certificate.
    pub client_certificate_path: Option<Input<String>>,
    /// The Client ID which should be used.
    pub client_id: Option<Input<String>>,
    /// The Client Secret which should be used. For use when authenticating as a Service Principal using a Client Secret.
    pub client_secret: Option<Input<String>>,
    /// Determines whether or not instance discovery is performed when attempting to authenticate. Setting this to true will completely disable both instance discovery and authority validation. This functionality is intended for use in scenarios where the metadata endpoint cannot be reached, such as in private clouds or Azure Stack.
    pub disable_instance_discovery: Option<Input<bool>>,
    /// This will disable the Pulumi Partner ID which is used if a custom `partnerId` isn't specified.
    pub disable_pulumi_partner_id: Option<Input<bool>>,
    /// The Cloud Environment which should be used. Possible values are public, usgovernment, and china. Defaults to public. Not used when metadataHost is specified or when ARM_METADATA_HOSTNAME is set.
    pub environment: Option<Input<String>>,
    /// The location to use. ResourceGroups will consult this property for a default location, if one was not supplied explicitly when defining the resource.
    pub location: Option<Input<String>>,
    /// The Hostname of the Azure Metadata Service.
    pub metadata_host: Option<Input<String>>,
    /// The path to a custom endpoint for Managed Service Identity - in most circumstances this should be detected automatically.
    pub msi_endpoint: Option<Input<String>>,
    /// Your cloud service or provider's bearer token to exchange for an OIDC ID token.
    pub oidc_request_token: Option<Input<String>>,
    /// The URL to initiate the OIDC token exchange. 
    pub oidc_request_url: Option<Input<String>>,
    /// The OIDC token to exchange for an Azure token.
    pub oidc_token: Option<Input<String>>,
    /// The path to a file containing an OIDC token to exchange for an Azure token.
    pub oidc_token_file_path: Option<Input<String>>,
    /// A GUID/UUID that is registered with Microsoft to facilitate partner resource usage attribution.
    pub partner_id: Option<Input<String>>,
    /// The Subscription ID which should be used.
    pub subscription_id: Option<Input<String>>,
    /// The Tenant ID which should be used.
    pub tenant_id: Option<Input<String>>,
    /// Use the default credential chain of the Azure SDK (see https://learn.microsoft.com/en-us/azure/developer/go/sdk/authentication/credential-chains#defaultazurecredential-overview).
    pub use_default_azure_credential: Option<Input<bool>>,
    /// Allow Managed Service Identity be used for Authentication.
    pub use_msi: Option<Input<bool>>,
    /// Allow OpenID Connect (OIDC) to be used for Authentication.
    pub use_oidc: Option<Input<bool>>,
}

/// The provider resource for the azure-native package.
pub struct Provider {
    /// The URN of the provider resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
}

impl Provider {
    const TYPE_TOKEN: &'static str = "pulumi:providers:azure-native";

    /// Create a new provider resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ProviderArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.auxiliary_tenant_ids {
            pulumi_sdk::resolve_input("auxiliaryTenantIds", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.client_certificate_password {
            pulumi_sdk::resolve_input("clientCertificatePassword", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.client_certificate_path {
            pulumi_sdk::resolve_input("clientCertificatePath", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.client_id {
            pulumi_sdk::resolve_input("clientId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.client_secret {
            pulumi_sdk::resolve_input("clientSecret", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.disable_instance_discovery {
            pulumi_sdk::resolve_input("disableInstanceDiscovery", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.disable_pulumi_partner_id {
            pulumi_sdk::resolve_input("disablePulumiPartnerId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.environment {
            pulumi_sdk::resolve_input("environment", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.metadata_host {
            pulumi_sdk::resolve_input("metadataHost", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.msi_endpoint {
            pulumi_sdk::resolve_input("msiEndpoint", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.oidc_request_token {
            pulumi_sdk::resolve_input("oidcRequestToken", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.oidc_request_url {
            pulumi_sdk::resolve_input("oidcRequestUrl", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.oidc_token {
            pulumi_sdk::resolve_input("oidcToken", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.oidc_token_file_path {
            pulumi_sdk::resolve_input("oidcTokenFilePath", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.partner_id {
            pulumi_sdk::resolve_input("partnerId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.subscription_id {
            pulumi_sdk::resolve_input("subscriptionId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tenant_id {
            pulumi_sdk::resolve_input("tenantId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.use_default_azure_credential {
            pulumi_sdk::resolve_input("useDefaultAzureCredential", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.use_msi {
            pulumi_sdk::resolve_input("useMsi", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.use_oidc {
            pulumi_sdk::resolve_input("useOidc", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        })
    }

    /// Convert to a ProviderResource for use in ResourceOptions.
    pub fn as_provider_resource(&self) -> ProviderResource {
        ProviderResource::new(
            self.urn.clone(),
            self.id.clone().apply(|v| v.as_str().unwrap_or_default().to_string()),
        )
    }
}
