use std::sync::Arc;
use tokio::sync::watch;

/// The state of an output value.
#[derive(Clone, Debug)]
pub enum OutputState<T> {
    /// Not yet resolved.
    Pending,
    /// Resolved with a known value.
    Known(T),
    /// Value won't be known until apply (preview mode).
    Unknown,
}

/// Internal data backing an `Output<T>`.
struct OutputInner<T: Clone + Send + Sync + 'static> {
    rx: watch::Receiver<OutputState<T>>,
    deps: Vec<String>,
    secret: bool,
}

/// The central abstraction in Pulumi's programming model.
///
/// Wraps an async value that may be pending, known, unknown, or secret.
/// Each output carries dependency URNs and supports `.apply()` to derive
/// new outputs.
pub struct Output<T: Clone + Send + Sync + 'static> {
    inner: Arc<OutputInner<T>>,
    tx: Option<Arc<watch::Sender<OutputState<T>>>>,
}

impl<T: Clone + Send + Sync + 'static> Clone for Output<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            tx: self.tx.clone(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Output<T> {
    /// Create a new pending output with the given dependency URNs and secret flag.
    pub fn new(deps: Vec<String>, secret: bool) -> Self {
        let (tx, rx) = watch::channel(OutputState::Pending);
        Self {
            inner: Arc::new(OutputInner { rx, deps, secret }),
            tx: Some(Arc::new(tx)),
        }
    }

    /// Create an output that is immediately resolved with a known value.
    pub fn known(value: T) -> Self {
        let (tx, rx) = watch::channel(OutputState::Known(value));
        Self {
            inner: Arc::new(OutputInner {
                rx,
                deps: Vec::new(),
                secret: false,
            }),
            tx: Some(Arc::new(tx)),
        }
    }

    /// Create an output that is immediately in the unknown state (preview mode).
    pub fn unknown(deps: Vec<String>) -> Self {
        let (tx, rx) = watch::channel(OutputState::Unknown);
        Self {
            inner: Arc::new(OutputInner {
                rx,
                deps,
                secret: false,
            }),
            tx: Some(Arc::new(tx)),
        }
    }

    /// Resolve this output with a known value.
    pub fn resolve(&self, value: T) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(OutputState::Known(value));
        }
    }

    /// Mark this output as unknown (preview mode).
    pub fn resolve_unknown(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(OutputState::Unknown);
        }
    }

    /// Returns the dependency URNs for this output.
    pub fn deps(&self) -> &[String] {
        &self.inner.deps
    }

    /// Returns whether this output is marked as secret.
    pub fn is_secret(&self) -> bool {
        self.inner.secret
    }

    /// Wait for this output to resolve and return the state.
    pub async fn wait(&self) -> OutputState<T> {
        let mut rx = self.inner.rx.clone();
        loop {
            let state = rx.borrow_and_update().clone();
            match state {
                OutputState::Pending => {
                    if rx.changed().await.is_err() {
                        return OutputState::Unknown;
                    }
                }
                other => return other,
            }
        }
    }

    /// Derive a new output by applying a function to this output's value.
    ///
    /// The new output inherits this output's dependencies and secret flag.
    /// If this output resolves to unknown, the derived output is also unknown.
    pub fn apply<U, F>(&self, f: F) -> Output<U>
    where
        U: Clone + Send + Sync + 'static,
        F: FnOnce(T) -> U + Send + 'static,
    {
        let parent = self.clone();
        let deps = self.inner.deps.clone();
        let secret = self.inner.secret;

        let (tx, rx) = watch::channel(OutputState::Pending);
        let tx = Arc::new(tx);
        let tx_clone = Arc::clone(&tx);

        tokio::spawn(async move {
            match parent.wait().await {
                OutputState::Known(val) => {
                    let result = f(val);
                    let _ = tx_clone.send(OutputState::Known(result));
                }
                OutputState::Unknown | OutputState::Pending => {
                    let _ = tx_clone.send(OutputState::Unknown);
                }
            }
        });

        Output {
            inner: Arc::new(OutputInner { rx, deps, secret }),
            tx: Some(tx),
        }
    }

    /// Create an output with secret flag set.
    pub fn with_secret(mut self, secret: bool) -> Self {
        let rx = self.inner.rx.clone();
        let deps = self.inner.deps.clone();
        self.inner = Arc::new(OutputInner { rx, deps, secret });
        self
    }

    /// Create an output with specified dependencies.
    pub fn with_deps(mut self, deps: Vec<String>) -> Self {
        let rx = self.inner.rx.clone();
        let secret = self.inner.secret;
        self.inner = Arc::new(OutputInner { rx, deps, secret });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_output_known() {
        let out = Output::known(42);
        match out.wait().await {
            OutputState::Known(v) => assert_eq!(v, 42),
            _ => panic!("expected Known"),
        }
    }

    #[tokio::test]
    async fn test_output_pending_then_resolved() {
        let out = Output::<String>::new(vec!["urn:a".to_string()], false);
        let out_clone = out.clone();

        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            out_clone.resolve("hello".to_string());
        });

        match out.wait().await {
            OutputState::Known(v) => assert_eq!(v, "hello"),
            _ => panic!("expected Known"),
        }
    }

    #[tokio::test]
    async fn test_output_unknown() {
        let out = Output::<i32>::unknown(vec!["urn:x".to_string()]);
        match out.wait().await {
            OutputState::Unknown => {}
            _ => panic!("expected Unknown"),
        }
    }

    #[tokio::test]
    async fn test_apply_propagates_value() {
        let out = Output::known(10);
        let doubled = out.apply(|v| v * 2);
        match doubled.wait().await {
            OutputState::Known(v) => assert_eq!(v, 20),
            _ => panic!("expected Known"),
        }
    }

    #[tokio::test]
    async fn test_apply_propagates_unknown() {
        let out = Output::<i32>::unknown(vec!["urn:a".to_string()]);
        let mapped = out.apply(|v| v + 1);
        match mapped.wait().await {
            OutputState::Unknown => {}
            _ => panic!("expected Unknown"),
        }
    }

    #[tokio::test]
    async fn test_apply_inherits_deps() {
        let out = Output::<i32>::new(vec!["urn:parent".to_string()], false);
        let child = out.apply(|v| v.to_string());
        assert_eq!(child.deps(), &["urn:parent".to_string()]);
    }

    #[tokio::test]
    async fn test_apply_inherits_secret() {
        let out = Output::<i32>::new(vec![], true);
        let child = out.apply(|v| v.to_string());
        assert!(child.is_secret());
    }

    #[tokio::test]
    async fn test_chained_apply() {
        let out = Output::known(5);
        let result = out.apply(|v| v * 2).apply(|v| format!("result={v}"));
        match result.wait().await {
            OutputState::Known(v) => assert_eq!(v, "result=10"),
            _ => panic!("expected Known"),
        }
    }

    #[tokio::test]
    async fn test_deps_tracking() {
        let out = Output::<i32>::new(vec!["urn:a".to_string(), "urn:b".to_string()], false);
        assert_eq!(out.deps(), &["urn:a", "urn:b"]);
    }
}
