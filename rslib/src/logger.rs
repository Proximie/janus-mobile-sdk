#[cfg(target_os = "android")]
#[uniffi::export]
pub fn init_janus_logger() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("jarust=trace".parse().unwrap()),
        )
        .compact()
        .init();
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_tag("JanusGateway"),
    );
    tracing::info!("Logger started");
    log_panics::init();
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[uniffi::export]
pub fn init_janus_logger(subsystem: &str, category: &str) {
    use apple_tracing_sub::subscriber::AppleTracingSubscriber;
    use tracing_subscriber::layer::SubscriberExt;

    let subscriber =
        tracing_subscriber::registry().with(AppleTracingSubscriber::new(subsystem, category));
    _ = tracing::subscriber::set_global_default(subscriber);
    tracing::info!("Logger started");
    log_panics::init();
}
