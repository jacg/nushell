use crate::commands::WholeStreamCommand;
use crate::context::CommandRegistry;
use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::{Signature, SyntaxShape, UntaggedValue};
use nu_source::Tagged;

pub struct Skip;

#[derive(Deserialize)]
pub struct SkipArgs {
    rows: Option<Tagged<usize>>,
}

#[async_trait]
impl WholeStreamCommand for Skip {
    fn name(&self) -> &str {
        "skip"
    }

    fn signature(&self) -> Signature {
        Signature::build("skip").optional("rows", SyntaxShape::Int, "how many rows to skip")
    }

    fn usage(&self) -> &str {
        "Skip some number of rows."
    }

    async fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        skip(args, registry).await
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Skip the first 5 rows",
            example: "echo [1 2 3 4 5 6 7] | skip 5",
            result: Some(vec![
                UntaggedValue::int(6).into(),
                UntaggedValue::int(7).into(),
            ]),
        }]
    }
}

async fn skip(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let registry = registry.clone();
    let (SkipArgs { rows }, input) = args.process(&registry).await?;
    let rows_desired = if let Some(quantity) = rows {
        *quantity
    } else {
        1
    };

    Ok(input.skip(rows_desired).to_output_stream())
}

#[cfg(test)]
mod tests {
    use super::Skip;

    #[test]
    fn examples_work_as_expected() {
        use crate::examples::test as test_examples;

        test_examples(Skip {})
    }
}
