use crate::output::Output;
use serde::{Serialize, Serializer};

/// An input that can be either a plain value or an [`Output`].
///
/// This is the primary type used for resource arguments. Generated provider
/// crates use `Input<T>` for required properties and `Option<Input<T>>` for
/// optional ones.
#[derive(Clone)]
pub enum Input<T: Clone + Send + Sync + 'static> {
    /// A plain, immediately-known value.
    Value(T),
    /// An output that may still be pending resolution.
    Output(Output<T>),
}

impl<T: Clone + Send + Sync + 'static> Input<T> {
    /// Convert this input into an [`Output`], lifting plain values into known outputs.
    pub fn into_output(self) -> Output<T> {
        match self {
            Input::Value(v) => Output::known(v),
            Input::Output(o) => o,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> From<T> for Input<T> {
    fn from(value: T) -> Self {
        Input::Value(value)
    }
}

impl<T: Clone + Send + Sync + 'static> From<Output<T>> for Input<T> {
    fn from(output: Output<T>) -> Self {
        Input::Output(output)
    }
}

impl<T: Clone + Send + Sync + Serialize + 'static> Serialize for Input<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Input::Value(v) => v.serialize(serializer),
            Input::Output(_) => Err(serde::ser::Error::custom(
                "cannot serialize an unresolved Output; resolve inputs before serializing",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::OutputState;

    #[tokio::test]
    async fn test_input_from_value() {
        let input: Input<i64> = 42.into();
        let output = input.into_output();
        match output.wait().await {
            OutputState::Known(v) => assert_eq!(v, 42),
            _ => panic!("expected Known"),
        }
    }

    #[tokio::test]
    async fn test_input_from_output() {
        let out = Output::known("hello".to_string());
        let input: Input<String> = out.into();
        let output = input.into_output();
        match output.wait().await {
            OutputState::Known(v) => assert_eq!(v, "hello"),
            _ => panic!("expected Known"),
        }
    }

    #[tokio::test]
    async fn test_input_into_output_preserves_deps() {
        let out = Output::<i32>::new(vec!["urn:dep".to_string()], true);
        out.resolve(99);
        let input = Input::Output(out);
        let output = input.into_output();
        assert_eq!(output.deps(), &["urn:dep".to_string()]);
        assert!(output.is_secret());
    }
}
