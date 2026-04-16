use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;
/// Input arguments for creating this resource.
///
/// The resource <span pulumi-lang-nodejs="`random.RandomUuid`" pulumi-lang-dotnet="`random.RandomUuid`" pulumi-lang-go="`RandomUuid`" pulumi-lang-python="`RandomUuid`" pulumi-lang-yaml="`random.RandomUuid`" pulumi-lang-java="`random.RandomUuid`">`random.RandomUuid`</span> generates a random uuid string that is intended to be used as a unique identifier for other resources.
///
/// This resource uses [hashicorp/go-uuid](https://github.com/hashicorp/go-uuid) to generate a UUID-formatted string for use with services needing a unique string identifier.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as azurerm from "@pulumi/azurerm";
/// import * as random from "@pulumi/random";
///
/// // The following example shows how to generate a unique name for an Azure Resource Group.
/// const test = new random.RandomUuid("test", {});
/// const testResourceGroup = new azurerm.index.ResourceGroup("test", {
///     name: `${test.result}-rg`,
///     location: "Central US",
/// });
/// ```
/// ```python
/// import pulumi
/// import pulumi_azurerm as azurerm
/// import pulumi_random as random
///
/// # The following example shows how to generate a unique name for an Azure Resource Group.
/// test = random.RandomUuid("test")
/// test_resource_group = azurerm.index.ResourceGroup("test",
///     name=f{test.result}-rg,
///     location=Central US)
/// ```
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Azurerm = Pulumi.Azurerm;
/// using Random = Pulumi.Random;
///
/// return await Deployment.RunAsync(() =>
/// {
///     // The following example shows how to generate a unique name for an Azure Resource Group.
///     var test = new Random.RandomUuid("test");
///
///     var testResourceGroup = new Azurerm.Index.ResourceGroup("test", new()
///     {
///         Name = $"{test.Result}-rg",
///         Location = "Central US",
///     });
///
/// });
/// ```
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-azurerm/sdk/go/azurerm"
/// 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		// The following example shows how to generate a unique name for an Azure Resource Group.
/// 		test, err := random.NewRandomUuid(ctx, "test", nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = azurerm.NewResourceGroup(ctx, "test", &azurerm.ResourceGroupArgs{
/// 			Name:     pulumi.Sprintf("%v-rg", test.Result),
/// 			Location: "Central US",
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.random.RandomUuid;
/// import com.pulumi.azurerm.ResourceGroup;
/// import com.pulumi.azurerm.ResourceGroupArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         // The following example shows how to generate a unique name for an Azure Resource Group.
///         var test = new RandomUuid("test");
///
///         var testResourceGroup = new ResourceGroup("testResourceGroup", ResourceGroupArgs.builder()
///             .name(String.format("%s-rg", test.result()))
///             .location("Central US")
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   # The following example shows how to generate a unique name for an Azure Resource Group.
///   test:
///     type: random:RandomUuid
///   testResourceGroup:
///     type: azurerm:ResourceGroup
///     name: test
///     properties:
///       name: ${test.result}-rg
///       location: Central US
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ## Import
///
/// The `pulumi import` command can be used, for example:
///
/// Random UUID's can be imported. This can be used to replace a config
/// value with a value interpolated from the random provider without
/// experiencing diffs.
///
/// ```sh
/// $ pulumi import random:index/randomUuid:RandomUuid main aabbccdd-eeff-0011-2233-445566778899
/// ```
///
#[derive(Default)]
pub struct RandomUuidArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Option<HashMap<String, Input<String>>>,
}
/// The resource <span pulumi-lang-nodejs="`random.RandomUuid`" pulumi-lang-dotnet="`random.RandomUuid`" pulumi-lang-go="`RandomUuid`" pulumi-lang-python="`RandomUuid`" pulumi-lang-yaml="`random.RandomUuid`" pulumi-lang-java="`random.RandomUuid`">`random.RandomUuid`</span> generates a random uuid string that is intended to be used as a unique identifier for other resources.
///
/// This resource uses [hashicorp/go-uuid](https://github.com/hashicorp/go-uuid) to generate a UUID-formatted string for use with services needing a unique string identifier.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as azurerm from "@pulumi/azurerm";
/// import * as random from "@pulumi/random";
///
/// // The following example shows how to generate a unique name for an Azure Resource Group.
/// const test = new random.RandomUuid("test", {});
/// const testResourceGroup = new azurerm.index.ResourceGroup("test", {
///     name: `${test.result}-rg`,
///     location: "Central US",
/// });
/// ```
/// ```python
/// import pulumi
/// import pulumi_azurerm as azurerm
/// import pulumi_random as random
///
/// # The following example shows how to generate a unique name for an Azure Resource Group.
/// test = random.RandomUuid("test")
/// test_resource_group = azurerm.index.ResourceGroup("test",
///     name=f{test.result}-rg,
///     location=Central US)
/// ```
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Azurerm = Pulumi.Azurerm;
/// using Random = Pulumi.Random;
///
/// return await Deployment.RunAsync(() =>
/// {
///     // The following example shows how to generate a unique name for an Azure Resource Group.
///     var test = new Random.RandomUuid("test");
///
///     var testResourceGroup = new Azurerm.Index.ResourceGroup("test", new()
///     {
///         Name = $"{test.Result}-rg",
///         Location = "Central US",
///     });
///
/// });
/// ```
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-azurerm/sdk/go/azurerm"
/// 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		// The following example shows how to generate a unique name for an Azure Resource Group.
/// 		test, err := random.NewRandomUuid(ctx, "test", nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = azurerm.NewResourceGroup(ctx, "test", &azurerm.ResourceGroupArgs{
/// 			Name:     pulumi.Sprintf("%v-rg", test.Result),
/// 			Location: "Central US",
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.random.RandomUuid;
/// import com.pulumi.azurerm.ResourceGroup;
/// import com.pulumi.azurerm.ResourceGroupArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         // The following example shows how to generate a unique name for an Azure Resource Group.
///         var test = new RandomUuid("test");
///
///         var testResourceGroup = new ResourceGroup("testResourceGroup", ResourceGroupArgs.builder()
///             .name(String.format("%s-rg", test.result()))
///             .location("Central US")
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   # The following example shows how to generate a unique name for an Azure Resource Group.
///   test:
///     type: random:RandomUuid
///   testResourceGroup:
///     type: azurerm:ResourceGroup
///     name: test
///     properties:
///       name: ${test.result}-rg
///       location: Central US
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ## Import
///
/// The `pulumi import` command can be used, for example:
///
/// Random UUID's can be imported. This can be used to replace a config
/// value with a value interpolated from the random provider without
/// experiencing diffs.
///
/// ```sh
/// $ pulumi import random:index/randomUuid:RandomUuid main aabbccdd-eeff-0011-2233-445566778899
/// ```
///
pub struct RandomUuid {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Output<serde_json::Value>,
    /// The generated uuid presented in string format.
    pub result: Output<serde_json::Value>,
}
impl RandomUuid {
    const TYPE_TOKEN: &'static str = "random:index/randomUuid:RandomUuid";
    /// Create a new RandomUuid resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: RandomUuidArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();
        if let Some(v) = args.keepers {
            pulumi_sdk::resolve_input_map(
                    "keepers",
                    v,
                    &mut inputs,
                    &mut deps,
                    &mut prop_deps,
                )
                .await;
        }
        let registered = ctx
            .register_resource(Self::TYPE_TOKEN, name, inputs, prop_deps, &opts)
            .await?;
        Ok(Self {
            urn: registered.urn.clone(),
            id: registered
                .outputs
                .get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            keepers: registered
                .outputs
                .get("keepers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            result: registered
                .outputs
                .get("result")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
