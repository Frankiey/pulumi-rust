use pulumi_azure_native::resources::resource_group::{ResourceGroup, ResourceGroupArgs};
use pulumi_sdk::{run, Input, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    run(|ctx| async move {
        let mut tags = HashMap::new();
        tags.insert("environment".into(), Input::Value("dev".into()));
        tags.insert("managed-by".into(), Input::Value("pulumi-rust".into()));

        let rg = ResourceGroup::new(
            &ctx,
            "my-resource-group",
            ResourceGroupArgs {
                location: Some(Input::Value("westeurope".into())),
                tags: Some(tags),
                ..Default::default()
            },
            None,
        )
        .await?;

        ctx.export("resourceGroupName", rg.name).await;

        Ok(())
    })
}
