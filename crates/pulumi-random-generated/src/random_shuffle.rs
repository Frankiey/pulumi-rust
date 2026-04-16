use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;
/// Input arguments for creating this resource.
///
/// The resource <span pulumi-lang-nodejs="`random.RandomShuffle`" pulumi-lang-dotnet="`random.RandomShuffle`" pulumi-lang-go="`RandomShuffle`" pulumi-lang-python="`RandomShuffle`" pulumi-lang-yaml="`random.RandomShuffle`" pulumi-lang-java="`random.RandomShuffle`">`random.RandomShuffle`</span> generates a random permutation of a list of strings given as an argument.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as random from "@pulumi/random";
///
/// const az = new random.RandomShuffle("az", {
///     inputs: [
///         "us-west-1a",
///         "us-west-1c",
///         "us-west-1d",
///         "us-west-1e",
///     ],
///     resultCount: 2,
/// });
/// const example = new aws.index.Elb("example", {availabilityZones: az.results});
/// ```
/// ```python
/// import pulumi
/// import pulumi_aws as aws
/// import pulumi_random as random
///
/// az = random.RandomShuffle("az",
///     inputs=[
///         "us-west-1a",
///         "us-west-1c",
///         "us-west-1d",
///         "us-west-1e",
///     ],
///     result_count=2)
/// example = aws.index.Elb("example", availability_zones=az.results)
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
///     var az = new Random.RandomShuffle("az", new()
///     {
///         Inputs = new[]
///         {
///             "us-west-1a",
///             "us-west-1c",
///             "us-west-1d",
///             "us-west-1e",
///         },
///         ResultCount = 2,
///     });
///
///     var example = new Aws.Index.Elb("example", new()
///     {
///         AvailabilityZones = az.Results,
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
/// 		az, err := random.NewRandomShuffle(ctx, "az", &random.RandomShuffleArgs{
/// 			Inputs: pulumi.StringArray{
/// 				pulumi.String("us-west-1a"),
/// 				pulumi.String("us-west-1c"),
/// 				pulumi.String("us-west-1d"),
/// 				pulumi.String("us-west-1e"),
/// 			},
/// 			ResultCount: pulumi.Int(2),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = aws.NewElb(ctx, "example", &aws.ElbArgs{
/// 			AvailabilityZones: az.Results,
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
/// import com.pulumi.random.RandomShuffle;
/// import com.pulumi.random.RandomShuffleArgs;
/// import com.pulumi.aws.Elb;
/// import com.pulumi.aws.ElbArgs;
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
///         var az = new RandomShuffle("az", RandomShuffleArgs.builder()
///             .inputs(
///                 "us-west-1a",
///                 "us-west-1c",
///                 "us-west-1d",
///                 "us-west-1e")
///             .resultCount(2)
///             .build());
///
///         var example = new Elb("example", ElbArgs.builder()
///             .availabilityZones(az.results())
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   az:
///     type: random:RandomShuffle
///     properties:
///       inputs:
///         - us-west-1a
///         - us-west-1c
///         - us-west-1d
///         - us-west-1e
///       resultCount: 2
///   example:
///     type: aws:Elb
///     properties:
///       availabilityZones: ${az.results}
/// ```
/// <!--End PulumiCodeChooser -->
pub struct RandomShuffleArgs {
    /// The list of strings to shuffle.
    pub inputs: Vec<Input<String>>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Option<HashMap<String, Input<String>>>,
    /// The number of results to return. Defaults to the number of items in the <span pulumi-lang-nodejs="`input`" pulumi-lang-dotnet="`Input`" pulumi-lang-go="`input`" pulumi-lang-python="`input`" pulumi-lang-yaml="`input`" pulumi-lang-java="`input`">`input`</span> list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
    pub result_count: Option<Input<i64>>,
    /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
    pub seed: Option<Input<String>>,
}
/// The resource <span pulumi-lang-nodejs="`random.RandomShuffle`" pulumi-lang-dotnet="`random.RandomShuffle`" pulumi-lang-go="`RandomShuffle`" pulumi-lang-python="`RandomShuffle`" pulumi-lang-yaml="`random.RandomShuffle`" pulumi-lang-java="`random.RandomShuffle`">`random.RandomShuffle`</span> generates a random permutation of a list of strings given as an argument.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as random from "@pulumi/random";
///
/// const az = new random.RandomShuffle("az", {
///     inputs: [
///         "us-west-1a",
///         "us-west-1c",
///         "us-west-1d",
///         "us-west-1e",
///     ],
///     resultCount: 2,
/// });
/// const example = new aws.index.Elb("example", {availabilityZones: az.results});
/// ```
/// ```python
/// import pulumi
/// import pulumi_aws as aws
/// import pulumi_random as random
///
/// az = random.RandomShuffle("az",
///     inputs=[
///         "us-west-1a",
///         "us-west-1c",
///         "us-west-1d",
///         "us-west-1e",
///     ],
///     result_count=2)
/// example = aws.index.Elb("example", availability_zones=az.results)
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
///     var az = new Random.RandomShuffle("az", new()
///     {
///         Inputs = new[]
///         {
///             "us-west-1a",
///             "us-west-1c",
///             "us-west-1d",
///             "us-west-1e",
///         },
///         ResultCount = 2,
///     });
///
///     var example = new Aws.Index.Elb("example", new()
///     {
///         AvailabilityZones = az.Results,
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
/// 		az, err := random.NewRandomShuffle(ctx, "az", &random.RandomShuffleArgs{
/// 			Inputs: pulumi.StringArray{
/// 				pulumi.String("us-west-1a"),
/// 				pulumi.String("us-west-1c"),
/// 				pulumi.String("us-west-1d"),
/// 				pulumi.String("us-west-1e"),
/// 			},
/// 			ResultCount: pulumi.Int(2),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = aws.NewElb(ctx, "example", &aws.ElbArgs{
/// 			AvailabilityZones: az.Results,
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
/// import com.pulumi.random.RandomShuffle;
/// import com.pulumi.random.RandomShuffleArgs;
/// import com.pulumi.aws.Elb;
/// import com.pulumi.aws.ElbArgs;
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
///         var az = new RandomShuffle("az", RandomShuffleArgs.builder()
///             .inputs(
///                 "us-west-1a",
///                 "us-west-1c",
///                 "us-west-1d",
///                 "us-west-1e")
///             .resultCount(2)
///             .build());
///
///         var example = new Elb("example", ElbArgs.builder()
///             .availabilityZones(az.results())
///             .build());
///
///     }
/// }
/// ```
/// ```yaml
/// resources:
///   az:
///     type: random:RandomShuffle
///     properties:
///       inputs:
///         - us-west-1a
///         - us-west-1c
///         - us-west-1d
///         - us-west-1e
///       resultCount: 2
///   example:
///     type: aws:Elb
///     properties:
///       availabilityZones: ${az.results}
/// ```
/// <!--End PulumiCodeChooser -->
pub struct RandomShuffle {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The list of strings to shuffle.
    pub inputs: Output<serde_json::Value>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: Output<serde_json::Value>,
    /// The number of results to return. Defaults to the number of items in the <span pulumi-lang-nodejs="`input`" pulumi-lang-dotnet="`Input`" pulumi-lang-go="`input`" pulumi-lang-python="`input`" pulumi-lang-yaml="`input`" pulumi-lang-java="`input`">`input`</span> list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
    pub result_count: Output<serde_json::Value>,
    /// Random permutation of the list of strings given in <span pulumi-lang-nodejs="`input`" pulumi-lang-dotnet="`Input`" pulumi-lang-go="`input`" pulumi-lang-python="`input`" pulumi-lang-yaml="`input`" pulumi-lang-java="`input`">`input`</span>. The number of elements is determined by <span pulumi-lang-nodejs="`resultCount`" pulumi-lang-dotnet="`ResultCount`" pulumi-lang-go="`resultCount`" pulumi-lang-python="`result_count`" pulumi-lang-yaml="`resultCount`" pulumi-lang-java="`resultCount`">`resultCount`</span> if set, or the number of elements in <span pulumi-lang-nodejs="`input`" pulumi-lang-dotnet="`Input`" pulumi-lang-go="`input`" pulumi-lang-python="`input`" pulumi-lang-yaml="`input`" pulumi-lang-java="`input`">`input`</span>.
    pub results: Output<serde_json::Value>,
    /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
    pub seed: Output<serde_json::Value>,
}
impl RandomShuffle {
    const TYPE_TOKEN: &'static str = "random:index/randomShuffle:RandomShuffle";
    /// Create a new RandomShuffle resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: RandomShuffleArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();
        pulumi_sdk::resolve_input_vec(
                "inputs",
                args.inputs,
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
        if let Some(v) = args.result_count {
            pulumi_sdk::resolve_input(
                    "resultCount",
                    v,
                    &mut inputs,
                    &mut deps,
                    &mut prop_deps,
                )
                .await;
        }
        if let Some(v) = args.seed {
            pulumi_sdk::resolve_input("seed", v, &mut inputs, &mut deps, &mut prop_deps)
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
            inputs: registered
                .outputs
                .get("inputs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            keepers: registered
                .outputs
                .get("keepers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            result_count: registered
                .outputs
                .get("resultCount")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            results: registered
                .outputs
                .get("results")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            seed: registered
                .outputs
                .get("seed")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
