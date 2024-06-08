use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn set_logger() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        "my_axum=debug,tower_http=debug".into()
        // std::env::var("RUST_LOG").unwrap_or_else(|_| "error".into())
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
}
