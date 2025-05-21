use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_tracing(module_log_filters: &str) {
    let fmt_layer = fmt::layer()
        .with_level(true)
        .with_ansi(true);

    let default_env_filter = match std::env::var("RUST_LOG") {
        Ok(_) => EnvFilter::from_env("RUST_LOG"),
        Err(_) => EnvFilter::new(module_log_filters),
    };

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(default_env_filter)
        .init();
}
