use anyhow::Result;
use pulumi_random::{Input, RandomString, RandomStringArgs};
use pulumi_sdk::run;

fn main() -> Result<()> {
    run(|ctx| {
        let rt = tokio::runtime::Handle::current();

        // --- Part 1: two random strings with an Output dependency ---

        // String A: 8 characters, no special chars
        let string_a = rt.block_on(RandomString::new(
            ctx,
            "resource-a",
            RandomStringArgs {
                length: Input::Value(8),
                special: Some(false),
                upper: None,
                lower: None,
                numeric: None,
            },
        ))?;

        // String B: length = A's result length * 2, with special chars
        let length_b = string_a.result.apply(|val| {
            let s = val.as_str().unwrap_or("");
            serde_json::json!(s.len() as i64 * 2)
        });

        let string_b = rt.block_on(RandomString::new(
            ctx,
            "resource-b",
            RandomStringArgs {
                length: Input::Output(length_b.apply(|v| v.as_i64().unwrap_or(16))),
                special: Some(true),
                upper: None,
                lower: None,
                numeric: None,
            },
        ))?;

        // Export both results as stack outputs
        rt.block_on(ctx.export("stringA", string_a.result));
        rt.block_on(ctx.export("stringB", string_b.result));

        Ok(())
    })
}
