use duct::cmd;
use std::{sync::Once, time::Duration};

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .pretty()
            .init();

        color_eyre::install().unwrap();
    });
}

pub type Result = eyre::Result<()>;

pub const TIMEOUT: Duration = Duration::from_secs(5);

static BUILD: Once = Once::new();

pub fn build_hello_world() -> String {
    BUILD.call_once(|| {
        cmd!(
            "cargo",
            "build",
            "-Z",
            "unstable-options",
            "--out-dir",
            "../.out"
        )
        .dir("samples/hello_world")
        .stdin_null()
        .stdout_null()
        .stderr_null()
        .run()
        .expect("Failed to build sample");
    });
    "samples/.out/hello_world".into()
}