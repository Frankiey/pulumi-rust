use std::collections::HashMap;
use std::time::Duration;

use crate::output::Output;

/// Options that control how a resource is managed by the Pulumi engine.
#[derive(Default, Clone)]
pub struct ResourceOptions {
    /// The parent resource URN. Defaults to the stack.
    pub parent: Option<String>,

    /// An explicit provider reference string to use for this resource.
    pub provider: Option<String>,

    /// Additional provider map for component resources (package name → provider ref).
    pub providers: HashMap<String, String>,

    /// URNs this resource depends on (explicit dependencies).
    pub depends_on: Vec<Output<String>>,

    /// If true, the resource is protected from deletion.
    pub protect: Option<bool>,

    /// Properties to ignore during updates.
    pub ignore_changes: Vec<String>,

    /// Properties that trigger replacement when changed.
    pub replace_on_changes: Vec<String>,

    /// If true, delete before creating the replacement.
    pub delete_before_replace: Option<bool>,

    /// Additional output properties to treat as secret.
    pub additional_secret_outputs: Vec<String>,

    /// Custom timeouts for CRUD operations.
    pub custom_timeouts: Option<CustomTimeouts>,

    /// Import an existing resource by provider ID.
    pub import: Option<String>,

    /// If true, retain the resource when removed from the program.
    pub retain_on_delete: Option<bool>,

    /// Resource aliases for migration/renaming.
    pub aliases: Vec<Alias>,

    /// The version of the provider plugin to use.
    pub version: Option<String>,

    /// The download URL for the provider plugin.
    pub plugin_download_url: Option<String>,

    /// If set, resource is deleted when this resource is deleted.
    pub deleted_with: Option<String>,
}

/// Custom timeouts for resource CRUD operations.
#[derive(Clone, Debug)]
pub struct CustomTimeouts {
    pub create: Option<Duration>,
    pub update: Option<Duration>,
    pub delete: Option<Duration>,
}

/// A resource alias for migration/renaming.
#[derive(Clone, Debug)]
pub struct Alias {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub parent: Option<String>,
    pub project: Option<String>,
    pub stack: Option<String>,
    pub no_parent: Option<bool>,
    pub parent_urn: Option<String>,
}

impl ResourceOptions {
    /// Create a new builder for `ResourceOptions`.
    pub fn builder() -> ResourceOptionsBuilder {
        ResourceOptionsBuilder::default()
    }
}

/// Builder for [`ResourceOptions`].
#[derive(Default)]
pub struct ResourceOptionsBuilder {
    opts: ResourceOptions,
}

impl ResourceOptionsBuilder {
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.opts.parent = Some(parent.into());
        self
    }

    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.opts.provider = Some(provider.into());
        self
    }

    pub fn depends_on(mut self, deps: Vec<Output<String>>) -> Self {
        self.opts.depends_on = deps;
        self
    }

    pub fn protect(mut self, protect: bool) -> Self {
        self.opts.protect = Some(protect);
        self
    }

    pub fn ignore_changes(mut self, keys: Vec<String>) -> Self {
        self.opts.ignore_changes = keys;
        self
    }

    pub fn replace_on_changes(mut self, keys: Vec<String>) -> Self {
        self.opts.replace_on_changes = keys;
        self
    }

    pub fn delete_before_replace(mut self, dbr: bool) -> Self {
        self.opts.delete_before_replace = Some(dbr);
        self
    }

    pub fn additional_secret_outputs(mut self, keys: Vec<String>) -> Self {
        self.opts.additional_secret_outputs = keys;
        self
    }

    pub fn custom_timeouts(mut self, timeouts: CustomTimeouts) -> Self {
        self.opts.custom_timeouts = Some(timeouts);
        self
    }

    pub fn import(mut self, id: impl Into<String>) -> Self {
        self.opts.import = Some(id.into());
        self
    }

    pub fn retain_on_delete(mut self, retain: bool) -> Self {
        self.opts.retain_on_delete = Some(retain);
        self
    }

    pub fn aliases(mut self, aliases: Vec<Alias>) -> Self {
        self.opts.aliases = aliases;
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.opts.version = Some(version.into());
        self
    }

    pub fn plugin_download_url(mut self, url: impl Into<String>) -> Self {
        self.opts.plugin_download_url = Some(url.into());
        self
    }

    pub fn deleted_with(mut self, urn: impl Into<String>) -> Self {
        self.opts.deleted_with = Some(urn.into());
        self
    }

    /// Build the `ResourceOptions`.
    pub fn build(self) -> ResourceOptions {
        self.opts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_resource_options() {
        let opts = ResourceOptions::default();
        assert!(opts.parent.is_none());
        assert!(opts.provider.is_none());
        assert!(opts.depends_on.is_empty());
        assert!(opts.protect.is_none());
        assert!(opts.ignore_changes.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let opts = ResourceOptions::builder()
            .parent("urn:pulumi:stack::project::type::parent")
            .protect(true)
            .ignore_changes(vec!["tags".to_string()])
            .version("1.0.0")
            .build();

        assert_eq!(
            opts.parent.as_deref(),
            Some("urn:pulumi:stack::project::type::parent")
        );
        assert_eq!(opts.protect, Some(true));
        assert_eq!(opts.ignore_changes, vec!["tags".to_string()]);
        assert_eq!(opts.version.as_deref(), Some("1.0.0"));
    }

    #[test]
    fn test_builder_delete_before_replace() {
        let opts = ResourceOptions::builder()
            .delete_before_replace(true)
            .build();
        assert_eq!(opts.delete_before_replace, Some(true));
    }

    #[test]
    fn test_builder_import() {
        let opts = ResourceOptions::builder()
            .import("/subscriptions/xxx/resourceGroups/rg")
            .build();
        assert_eq!(
            opts.import.as_deref(),
            Some("/subscriptions/xxx/resourceGroups/rg")
        );
    }

    #[test]
    fn test_builder_custom_timeouts() {
        let opts = ResourceOptions::builder()
            .custom_timeouts(CustomTimeouts {
                create: Some(Duration::from_secs(300)),
                update: None,
                delete: Some(Duration::from_secs(600)),
            })
            .build();
        let ct = opts.custom_timeouts.unwrap();
        assert_eq!(ct.create, Some(Duration::from_secs(300)));
        assert!(ct.update.is_none());
        assert_eq!(ct.delete, Some(Duration::from_secs(600)));
    }
}
