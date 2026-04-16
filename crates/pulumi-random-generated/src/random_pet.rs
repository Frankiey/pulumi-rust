use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;
/// Input arguments for creating this resource.
///
/// The resource <span pulumi-lang-nodejs="`random.RandomPet`" pulumi-lang-dotnet="`random.RandomPet`" pulumi-lang-go="`RandomPet`" pulumi-lang-python="`RandomPet`" pulumi-lang-yaml="`random.RandomPet`" pulumi-lang-java="`random.RandomPet`">`random.RandomPet`</span> generates random pet names that are intended to be used as unique identifiers for other resources.
///
/// This resource can be used in conjunction with resources that have the <span pulumi-lang-nodejs="`createBeforeDestroy`" pulumi-lang-dotnet="`CreateBeforeDestroy`" pulumi-lang-go="`createBeforeDestroy`" pulumi-lang-python="`create_before_destroy`" pulumi-lang-yaml="`createBeforeDestroy`" pulumi-lang-java="`createBeforeDestroy`">`createBeforeDestroy`</span> lifecycle flag set, to avoid conflicts with unique names during the brief period where both the old and new resources exist concurrently.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as random from "@pulumi/random";
///
/// // The following example shows how to generate a unique pet name
/// // for an AWS EC2 instance that changes each time a new AMI id is
/// // selected.
/// const server = new random.RandomPet("server", {keepers: {
///     ami_id: amiId,
/// }});
/// const serverInstance = new aws.index.Instance("server", {
///     tags: {
///         name: `web-server-${server.id}`,
///     },
///     ami: server.keepers?.amiId,
/// });
/// ```
/// ```python
/// import pulumi
/// import pulumi_aws as aws
/// import pulumi_random as random
///
/// # The following example shows how to generate a unique pet name
/// # for an AWS EC2 instance that changes each time a new AMI id is
/// # selected.
/// server = random.RandomPet("server", keepers={
///     "ami_id": ami_id,
/// })
/// server_instance = aws.index.Instance("server",
///     tags={
///         name: fweb-server-{server.id},
///     },
///     ami=server.keepers.ami_id)
/// ```
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Aws = Pulumi.Aws;
/// using Random = Pulumi.Random;
///
/// return await Deployment.RunAsync(() =>
/// {
///     // The following example shows how to generate a unique pet name
///     // for an AWS EC2 instance that changes each time a new AMI id is
///     // selected.
///     var server = new Random.RandomPet("server", new()
///     {
///         Keepers =
///         {
///             { "ami_id", amiId },
///         },
///     });
///
///     var serverInstance = new Aws.Index.Instance("server", new()
///     {
///         Tags =
///         {
///             { "name", $"web-server-{server.Id}" },
///         },
///         Ami = server.Keepers?.AmiId,
///     });
///
/// });
/// ```
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-aws/sdk/v7/go/aws"
/// 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		// The following example shows how to generate a unique pet name
/// 		// for an AWS EC2 instance that changes each time a new AMI id is
/// 		// selected.
/// 		server, err := random.NewRandomPet(ctx, "server", &random.RandomPetArgs{
/// 			Keepers: pulumi.StringMap{
/// 				"ami_id": pulumi.Any(amiId),
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = aws.NewInstance(ctx, "server", &aws.InstanceArgs{
/// 			Tags: map[string]interface{}{
/// 				"name": pulumi.Sprintf("web-server-%v", server.ID()),
/// 			},
/// 			Ami: server.Keepers.AmiId,
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
/// import com.pulumi.random.RandomPet;
/// import com.pulumi.random.RandomPetArgs;
/// import com.pulumi.aws.Instance;
/// import com.pulumi.aws.InstanceArgs;
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
///         // The following example shows how to generate a unique pet name
///         // for an AWS EC2 instance that changes each time a new AMI id is
///         // selected.
///         var server = new RandomPet("server", RandomPetArgs.builder()
///             .keepers(Map.of("ami_id", amiId))
///             .build());
///
///         var serverInstance = new Instance("serverInstance", InstanceArgs.builder()
///             .tags(Map.of("name", String.format("web-server-%s", server.id())))
///             .ami(server.keepers().amiId())
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   # The following example shows how to generate a unique pet name
///   # for an AWS EC2 instance that changes each time a new AMI id is
///   # selected.
///   server:
///     type: random:RandomPet
///     properties:
///       keepers:
///         ami_id: ${amiId}
///   serverInstance:
///     type: aws:Instance
///     name: server
///     properties:
///       tags:
///         name: web-server-${server.id}
///       ami: ${server.keepers.amiId}
/// ```
/// <!--End PulumiCodeChooser -->
#[derive(Default)]
pub struct RandomPetArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Option<HashMap<String, Input<String>>>,
    /// The length (in words) of the pet name. Defaults to 2
    pub length: Option<Input<i64>>,
    /// A string to prefix the name with.
    pub prefix: Option<Input<String>>,
    /// The character to separate words in the pet name. Defaults to "-"
    pub separator: Option<Input<String>>,
}
/// The resource <span pulumi-lang-nodejs="`random.RandomPet`" pulumi-lang-dotnet="`random.RandomPet`" pulumi-lang-go="`RandomPet`" pulumi-lang-python="`RandomPet`" pulumi-lang-yaml="`random.RandomPet`" pulumi-lang-java="`random.RandomPet`">`random.RandomPet`</span> generates random pet names that are intended to be used as unique identifiers for other resources.
///
/// This resource can be used in conjunction with resources that have the <span pulumi-lang-nodejs="`createBeforeDestroy`" pulumi-lang-dotnet="`CreateBeforeDestroy`" pulumi-lang-go="`createBeforeDestroy`" pulumi-lang-python="`create_before_destroy`" pulumi-lang-yaml="`createBeforeDestroy`" pulumi-lang-java="`createBeforeDestroy`">`createBeforeDestroy`</span> lifecycle flag set, to avoid conflicts with unique names during the brief period where both the old and new resources exist concurrently.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as random from "@pulumi/random";
///
/// // The following example shows how to generate a unique pet name
/// // for an AWS EC2 instance that changes each time a new AMI id is
/// // selected.
/// const server = new random.RandomPet("server", {keepers: {
///     ami_id: amiId,
/// }});
/// const serverInstance = new aws.index.Instance("server", {
///     tags: {
///         name: `web-server-${server.id}`,
///     },
///     ami: server.keepers?.amiId,
/// });
/// ```
/// ```python
/// import pulumi
/// import pulumi_aws as aws
/// import pulumi_random as random
///
/// # The following example shows how to generate a unique pet name
/// # for an AWS EC2 instance that changes each time a new AMI id is
/// # selected.
/// server = random.RandomPet("server", keepers={
///     "ami_id": ami_id,
/// })
/// server_instance = aws.index.Instance("server",
///     tags={
///         name: fweb-server-{server.id},
///     },
///     ami=server.keepers.ami_id)
/// ```
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Aws = Pulumi.Aws;
/// using Random = Pulumi.Random;
///
/// return await Deployment.RunAsync(() =>
/// {
///     // The following example shows how to generate a unique pet name
///     // for an AWS EC2 instance that changes each time a new AMI id is
///     // selected.
///     var server = new Random.RandomPet("server", new()
///     {
///         Keepers =
///         {
///             { "ami_id", amiId },
///         },
///     });
///
///     var serverInstance = new Aws.Index.Instance("server", new()
///     {
///         Tags =
///         {
///             { "name", $"web-server-{server.Id}" },
///         },
///         Ami = server.Keepers?.AmiId,
///     });
///
/// });
/// ```
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-aws/sdk/v7/go/aws"
/// 	"github.com/pulumi/pulumi-random/sdk/v4/go/random"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		// The following example shows how to generate a unique pet name
/// 		// for an AWS EC2 instance that changes each time a new AMI id is
/// 		// selected.
/// 		server, err := random.NewRandomPet(ctx, "server", &random.RandomPetArgs{
/// 			Keepers: pulumi.StringMap{
/// 				"ami_id": pulumi.Any(amiId),
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = aws.NewInstance(ctx, "server", &aws.InstanceArgs{
/// 			Tags: map[string]interface{}{
/// 				"name": pulumi.Sprintf("web-server-%v", server.ID()),
/// 			},
/// 			Ami: server.Keepers.AmiId,
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
/// import com.pulumi.random.RandomPet;
/// import com.pulumi.random.RandomPetArgs;
/// import com.pulumi.aws.Instance;
/// import com.pulumi.aws.InstanceArgs;
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
///         // The following example shows how to generate a unique pet name
///         // for an AWS EC2 instance that changes each time a new AMI id is
///         // selected.
///         var server = new RandomPet("server", RandomPetArgs.builder()
///             .keepers(Map.of("ami_id", amiId))
///             .build());
///
///         var serverInstance = new Instance("serverInstance", InstanceArgs.builder()
///             .tags(Map.of("name", String.format("web-server-%s", server.id())))
///             .ami(server.keepers().amiId())
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   # The following example shows how to generate a unique pet name
///   # for an AWS EC2 instance that changes each time a new AMI id is
///   # selected.
///   server:
///     type: random:RandomPet
///     properties:
///       keepers:
///         ami_id: ${amiId}
///   serverInstance:
///     type: aws:Instance
///     name: server
///     properties:
///       tags:
///         name: web-server-${server.id}
///       ami: ${server.keepers.amiId}
/// ```
/// <!--End PulumiCodeChooser -->
pub struct RandomPet {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Output<serde_json::Value>,
    /// The length (in words) of the pet name. Defaults to 2
    pub length: Output<serde_json::Value>,
    /// A string to prefix the name with.
    pub prefix: Output<serde_json::Value>,
    /// The character to separate words in the pet name. Defaults to "-"
    pub separator: Output<serde_json::Value>,
}
impl RandomPet {
    const TYPE_TOKEN: &'static str = "random:index/randomPet:RandomPet";
    /// Create a new RandomPet resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: RandomPetArgs,
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
        if let Some(v) = args.length {
            pulumi_sdk::resolve_input(
                    "length",
                    v,
                    &mut inputs,
                    &mut deps,
                    &mut prop_deps,
                )
                .await;
        }
        if let Some(v) = args.prefix {
            pulumi_sdk::resolve_input(
                    "prefix",
                    v,
                    &mut inputs,
                    &mut deps,
                    &mut prop_deps,
                )
                .await;
        }
        if let Some(v) = args.separator {
            pulumi_sdk::resolve_input(
                    "separator",
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
            length: registered
                .outputs
                .get("length")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            prefix: registered
                .outputs
                .get("prefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            separator: registered
                .outputs
                .get("separator")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
