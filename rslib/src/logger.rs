#[uniffi::export]
pub fn raw_init_logger(subsystem: &str, category: &str) {
    #[cfg(target_os = "android")]
    {
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
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        use apple_tracing_sub::subscriber::AppleTracingSubscriber;
        use tracing_subscriber::layer::SubscriberExt;

        let subscriber =
            tracing_subscriber::registry().with(AppleTracingSubscriber::new(subsystem, category));
        let logger = tracing::subscriber::set_global_default(subscriber);
        match logger {
            Ok(()) => {}
            Err(why) => tracing::error!("{why}"),
        };
    }

    tracing::info!("JanusGateway started logging");
    log_panics::init();
}
