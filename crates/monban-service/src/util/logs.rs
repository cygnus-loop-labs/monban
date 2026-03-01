use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!(
        target: "Logger",
        "Logger initialized (RUST_LOG={:?})",
        std::env::var("RUST_LOG")
    );
}
