use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;
/// Input arguments for creating this resource.
///
/// The resource <span pulumi-lang-nodejs="`random.RandomId`" pulumi-lang-dotnet="`random.RandomId`" pulumi-lang-go="`RandomId`" pulumi-lang-python="`RandomId`" pulumi-lang-yaml="`random.RandomId`" pulumi-lang-java="`random.RandomId`">`random.RandomId`</span> generates random numbers that are intended to be
/// used as unique identifiers for other resources. If the output is considered
/// sensitive, and should not be displayed in the CLI, use <span pulumi-lang-nodejs="`random.RandomBytes`" pulumi-lang-dotnet="`random.RandomBytes`" pulumi-lang-go="`RandomBytes`" pulumi-lang-python="`RandomBytes`" pulumi-lang-yaml="`random.RandomBytes`" pulumi-lang-java="`random.RandomBytes`">`random.RandomBytes`</span>
/// instead.
///
/// This resource *does* use a cryptographic random number generator in order
/// to minimize the chance of collisions, making the results of this resource
/// when a 16-byte identifier is requested of equivalent uniqueness to a
/// type-4 UUID.
///
/// This resource can be used in conjunction with resources that have
/// the <span pulumi-lang-nodejs="`createBeforeDestroy`" pulumi-lang-dotnet="`CreateBeforeDestroy`" pulumi-lang-go="`createBeforeDestroy`" pulumi-lang-python="`create_before_destroy`" pulumi-lang-yaml="`createBeforeDestroy`" pulumi-lang-java="`createBeforeDestroy`">`createBeforeDestroy`</span> lifecycle flag set to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as random from "@pulumi/random";
///
/// // The following example shows how to generate a unique name for an AWS EC2
/// // instance that changes each time a new AMI id is selected.
/// const server = new random.RandomId("server", {
///     keepers: {
///         ami_id: amiId,
///     },
///     byteLength: 8,
/// });
/// const serverInstance = new aws.index.Instance("server", {
///     tags: {
///         name: `web-server ${server.hex}`,
///     },
///     ami: server.keepers?.amiId,
/// });
/// ```
/// ```python
/// import pulumi
/// import pulumi_aws as aws
/// import pulumi_random as random
///
/// # The following example shows how to generate a unique name for an AWS EC2
/// # instance that changes each time a new AMI id is selected.
/// server = random.RandomId("server",
///     keepers={
///         "ami_id": ami_id,
///     },
///     byte_length=8)
/// server_instance = aws.index.Instance("server",
///     tags={
///         name: fweb-server {server.hex},
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
///     // The following example shows how to generate a unique name for an AWS EC2
///     // instance that changes each time a new AMI id is selected.
///     var server = new Random.RandomId("server", new()
///     {
///         Keepers =
///         {
///             { "ami_id", amiId },
///         },
///         ByteLength = 8,
///     });
///
///     var serverInstance = new Aws.Index.Instance("server", new()
///     {
///         Tags =
///         {
///             { "name", $"web-server {server.Hex}" },
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
/// 		// The following example shows how to generate a unique name for an AWS EC2
/// 		// instance that changes each time a new AMI id is selected.
/// 		server, err := random.NewRandomId(ctx, "server", &random.RandomIdArgs{
/// 			Keepers: pulumi.StringMap{
/// 				"ami_id": pulumi.Any(amiId),
/// 			},
/// 			ByteLength: pulumi.Int(8),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = aws.NewInstance(ctx, "server", &aws.InstanceArgs{
/// 			Tags: map[string]interface{}{
/// 				"name": pulumi.Sprintf("web-server %v", server.Hex),
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
/// import com.pulumi.random.RandomId;
/// import com.pulumi.random.RandomIdArgs;
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
///         // The following example shows how to generate a unique name for an AWS EC2
///         // instance that changes each time a new AMI id is selected.
///         var server = new RandomId("server", RandomIdArgs.builder()
///             .keepers(Map.of("ami_id", amiId))
///             .byteLength(8)
///             .build());
///
///         var serverInstance = new Instance("serverInstance", InstanceArgs.builder()
///             .tags(Map.of("name", String.format("web-server %s", server.hex())))
///             .ami(server.keepers().amiId())
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   # The following example shows how to generate a unique name for an AWS EC2
///   # instance that changes each time a new AMI id is selected.
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         ami_id: ${amiId}
///       byteLength: 8
///   serverInstance:
///     type: aws:Instance
///     name: server
///     properties:
///       tags:
///         name: web-server ${server.hex}
///       ami: ${server.keepers.amiId}
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ## Import
///
/// The `pulumi import` command can be used, for example:
///
/// Random IDs can be imported using the<span pulumi-lang-nodejs=" b64Url " pulumi-lang-dotnet=" B64Url " pulumi-lang-go=" b64Url " pulumi-lang-python=" b64_url " pulumi-lang-yaml=" b64Url " pulumi-lang-java=" b64Url "> b64Url </span>with an optional prefix. This
/// can be used to replace a config value with a value interpolated from the
/// random provider without experiencing diffs.
///
/// Example with no prefix:
///
/// ```sh
/// $ pulumi import random:index/randomId:RandomId server p-9hUg
/// ```
///
/// Example with prefix (prefix is separated by a ,):
///
/// ```sh
/// $ pulumi import random:index/randomId:RandomId server my-prefix-,p-9hUg
/// ```
///
pub struct RandomIdArgs {
    /// The number of random bytes to produce. The minimum value is 1, which produces eight bits of randomness.
    pub byte_length: Input<i64>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Option<HashMap<String, Input<String>>>,
    /// Arbitrary string to prefix the output value with. This string is supplied as-is, meaning it is not guaranteed to be URL-safe or base64 encoded.
    pub prefix: Option<Input<String>>,
}
/// The resource <span pulumi-lang-nodejs="`random.RandomId`" pulumi-lang-dotnet="`random.RandomId`" pulumi-lang-go="`RandomId`" pulumi-lang-python="`RandomId`" pulumi-lang-yaml="`random.RandomId`" pulumi-lang-java="`random.RandomId`">`random.RandomId`</span> generates random numbers that are intended to be
/// used as unique identifiers for other resources. If the output is considered
/// sensitive, and should not be displayed in the CLI, use <span pulumi-lang-nodejs="`random.RandomBytes`" pulumi-lang-dotnet="`random.RandomBytes`" pulumi-lang-go="`RandomBytes`" pulumi-lang-python="`RandomBytes`" pulumi-lang-yaml="`random.RandomBytes`" pulumi-lang-java="`random.RandomBytes`">`random.RandomBytes`</span>
/// instead.
///
/// This resource *does* use a cryptographic random number generator in order
/// to minimize the chance of collisions, making the results of this resource
/// when a 16-byte identifier is requested of equivalent uniqueness to a
/// type-4 UUID.
///
/// This resource can be used in conjunction with resources that have
/// the <span pulumi-lang-nodejs="`createBeforeDestroy`" pulumi-lang-dotnet="`CreateBeforeDestroy`" pulumi-lang-go="`createBeforeDestroy`" pulumi-lang-python="`create_before_destroy`" pulumi-lang-yaml="`createBeforeDestroy`" pulumi-lang-java="`createBeforeDestroy`">`createBeforeDestroy`</span> lifecycle flag set to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as random from "@pulumi/random";
///
/// // The following example shows how to generate a unique name for an AWS EC2
/// // instance that changes each time a new AMI id is selected.
/// const server = new random.RandomId("server", {
///     keepers: {
///         ami_id: amiId,
///     },
///     byteLength: 8,
/// });
/// const serverInstance = new aws.index.Instance("server", {
///     tags: {
///         name: `web-server ${server.hex}`,
///     },
///     ami: server.keepers?.amiId,
/// });
/// ```
/// ```python
/// import pulumi
/// import pulumi_aws as aws
/// import pulumi_random as random
///
/// # The following example shows how to generate a unique name for an AWS EC2
/// # instance that changes each time a new AMI id is selected.
/// server = random.RandomId("server",
///     keepers={
///         "ami_id": ami_id,
///     },
///     byte_length=8)
/// server_instance = aws.index.Instance("server",
///     tags={
///         name: fweb-server {server.hex},
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
///     // The following example shows how to generate a unique name for an AWS EC2
///     // instance that changes each time a new AMI id is selected.
///     var server = new Random.RandomId("server", new()
///     {
///         Keepers =
///         {
///             { "ami_id", amiId },
///         },
///         ByteLength = 8,
///     });
///
///     var serverInstance = new Aws.Index.Instance("server", new()
///     {
///         Tags =
///         {
///             { "name", $"web-server {server.Hex}" },
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
/// 		// The following example shows how to generate a unique name for an AWS EC2
/// 		// instance that changes each time a new AMI id is selected.
/// 		server, err := random.NewRandomId(ctx, "server", &random.RandomIdArgs{
/// 			Keepers: pulumi.StringMap{
/// 				"ami_id": pulumi.Any(amiId),
/// 			},
/// 			ByteLength: pulumi.Int(8),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = aws.NewInstance(ctx, "server", &aws.InstanceArgs{
/// 			Tags: map[string]interface{}{
/// 				"name": pulumi.Sprintf("web-server %v", server.Hex),
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
/// import com.pulumi.random.RandomId;
/// import com.pulumi.random.RandomIdArgs;
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
///         // The following example shows how to generate a unique name for an AWS EC2
///         // instance that changes each time a new AMI id is selected.
///         var server = new RandomId("server", RandomIdArgs.builder()
///             .keepers(Map.of("ami_id", amiId))
///             .byteLength(8)
///             .build());
///
///         var serverInstance = new Instance("serverInstance", InstanceArgs.builder()
///             .tags(Map.of("name", String.format("web-server %s", server.hex())))
///             .ami(server.keepers().amiId())
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   # The following example shows how to generate a unique name for an AWS EC2
///   # instance that changes each time a new AMI id is selected.
///   server:
///     type: random:RandomId
///     properties:
///       keepers:
///         ami_id: ${amiId}
///       byteLength: 8
///   serverInstance:
///     type: aws:Instance
///     name: server
///     properties:
///       tags:
///         name: web-server ${server.hex}
///       ami: ${server.keepers.amiId}
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ## Import
///
/// The `pulumi import` command can be used, for example:
///
/// Random IDs can be imported using the<span pulumi-lang-nodejs=" b64Url " pulumi-lang-dotnet=" B64Url " pulumi-lang-go=" b64Url " pulumi-lang-python=" b64_url " pulumi-lang-yaml=" b64Url " pulumi-lang-java=" b64Url "> b64Url </span>with an optional prefix. This
/// can be used to replace a config value with a value interpolated from the
/// random provider without experiencing diffs.
///
/// Example with no prefix:
///
/// ```sh
/// $ pulumi import random:index/randomId:RandomId server p-9hUg
/// ```
///
/// Example with prefix (prefix is separated by a ,):
///
/// ```sh
/// $ pulumi import random:index/randomId:RandomId server my-prefix-,p-9hUg
/// ```
///
pub struct RandomId {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The generated id presented in base64 without additional transformations.
    pub b64std: Output<serde_json::Value>,
    /// The generated id presented in base64, using the URL-friendly character set: case-sensitive letters, digits and the characters `_` and `-`.
    pub b64url: Output<serde_json::Value>,
    /// The number of random bytes to produce. The minimum value is 1, which produces eight bits of randomness.
    pub byte_length: Output<serde_json::Value>,
    /// The generated id presented in non-padded decimal digits.
    pub dec: Output<serde_json::Value>,
    /// The generated id presented in padded hexadecimal digits. This result will always be twice as long as the requested byte length.
    pub hex: Output<serde_json::Value>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Output<serde_json::Value>,
    /// Arbitrary string to prefix the output value with. This string is supplied as-is, meaning it is not guaranteed to be URL-safe or base64 encoded.
    pub prefix: Output<serde_json::Value>,
}
impl RandomId {
    const TYPE_TOKEN: &'static str = "random:index/randomId:RandomId";
    /// Create a new RandomId resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: RandomIdArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();
        pulumi_sdk::resolve_input(
                "byteLength",
                args.byte_length,
                &mut inputs,
                &mut deps,
                &mut prop_deps,
            )
            .await;
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
            b64std: registered
                .outputs
                .get("b64Std")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            b64url: registered
                .outputs
                .get("b64Url")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            byte_length: registered
                .outputs
                .get("byteLength")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dec: registered
                .outputs
                .get("dec")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            hex: registered
                .outputs
                .get("hex")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            keepers: registered
                .outputs
                .get("keepers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            prefix: registered
                .outputs
                .get("prefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
