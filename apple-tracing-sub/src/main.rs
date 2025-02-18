use apple_tracing_sub::subscriber::AppleTracingSubscriber;
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let subscriber = tracing_subscriber::registry()
        .with(AppleTracingSubscriber::new("com.ghamza.idkman", "jarust"));

    tracing::subscriber::set_global_default(subscriber).unwrap();
    let span = tracing::info_span!("example_span");

    tracing::trace!("This is a trace message");
    tracing::debug!("This is a debug message");
    tracing::info!("This is an info message");
    tracing::warn!("This is a warning message");
    tracing::error!("This is an error message");

    let _guard = span.enter();
    tracing::warn!("Inside a span!");
}
