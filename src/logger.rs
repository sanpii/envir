pub fn init() {
    #[cfg(feature = "env_logger")]
    env_logger::init();

    #[cfg(feature = "tracing")]
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    #[cfg(feature = "tracing")]
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
}

pub fn try_init() -> crate::Result {
    #[cfg(feature = "env_logger")]
    env_logger::try_init().map_err(|e| crate::Error::Logger(e.to_string()))?;

    #[cfg(feature = "tracing")]
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    #[cfg(feature = "tracing")]
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init()
        .map_err(|e| crate::Error::Logger(e.to_string()))?;

    Ok(())
}
