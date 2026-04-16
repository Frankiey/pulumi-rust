use pulumi_random::random_string::{RandomString, RandomStringArgs};
use pulumi_sdk::{run, Input, Result};

fn main() -> Result<()> {
    run(|ctx| async move {
        // --- Part 1: two random strings with an Output dependency ---

        // String A: 8 characters, no special chars
        let string_a = RandomString::new(
            &ctx,
            "resource-a",
            RandomStringArgs {
                length: Input::Value(8),
                special: Some(Input::Value(false)),
                upper: None,
                lower: None,
                numeric: None,
                keepers: None,
                min_lower: None,
                min_numeric: None,
                min_special: None,
                min_upper: None,
                number: None,
                override_special: None,
            },
            None,
        )
        .await?;

        // String B: length = A's result length * 2, with special chars
        let length_b = string_a.result.apply(|val| {
            let s = val.as_str().unwrap_or("");
            s.len() as i64 * 2
        });

        let string_b = RandomString::new(
            &ctx,
            "resource-b",
            RandomStringArgs {
                length: Input::Output(length_b),
                special: Some(Input::Value(true)),
                upper: None,
                lower: None,
                numeric: None,
                keepers: None,
                min_lower: None,
                min_numeric: None,
                min_special: None,
                min_upper: None,
                number: None,
                override_special: None,
            },
            None,
        )
        .await?;

        // Export both results as stack outputs
        ctx.export("stringA", string_a.result).await;
        ctx.export("stringB", string_b.result).await;

        Ok(())
    })
}
