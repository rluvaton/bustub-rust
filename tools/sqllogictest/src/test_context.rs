use std::path::Path;
use bustub_instance::BustubInstance;
use tempfile::TempDir;

/// Context for running tests
pub struct TestContext {
    /// Context for running queries
    ctx: BustubInstance,
    /// Temporary directory created and cleared at the end of the test
    test_dir: Option<TempDir>,
}

impl TestContext {
    pub fn new(ctx: BustubInstance) -> Self {
        Self {
            ctx,
            test_dir: None,
        }
    }

    /// Enables the test directory feature. If not enabled,
    /// calling `testdir_path` will result in a panic.
    pub fn enable_testdir(&mut self) {
        if self.test_dir.is_none() {
            self.test_dir = Some(TempDir::new().expect("failed to create testdir"));
        }
    }

    /// Returns the path to the test directory. Panics if the test
    /// directory feature is not enabled via `enable_testdir`.
    pub fn testdir_path(&self) -> &Path {
        self.test_dir.as_ref().expect("testdir not enabled").path()
    }

    /// Returns a reference to the internal SessionContext
    pub fn session_ctx(&self) -> &BustubInstance {
        &self.ctx
    }
    /// Returns a reference to the internal SessionContext
    pub fn ctx(self) -> BustubInstance {
        self.ctx
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            ctx: BustubInstance::in_memory(None),
            test_dir: None
        }
    }
}
