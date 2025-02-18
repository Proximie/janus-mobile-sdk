use crate::apple_log::AppleLog;
use core::fmt;
use tracing_core::field::Visit;
use tracing_core::Event;
use tracing_core::Field;
use tracing_core::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

pub struct AppleTracingSubscriber {
    logger: AppleLog,
}

impl AppleTracingSubscriber {
    pub fn new(subsystem: &str, category: &str) -> Self {
        Self {
            logger: AppleLog::new(subsystem, category),
        }
    }
}

impl<S> Layer<S> for AppleTracingSubscriber
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = EventVisitor::default();
        event.record(&mut visitor);
        self.logger
            .log(&format!("{:#?}", visitor), *event.metadata().level());
    }
}

#[derive(Default)]
struct EventVisitor {
    fields: Vec<String>,
}

impl fmt::Debug for EventVisitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fields.join(", "))
    }
}

impl Visit for EventVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.fields.push(format!("{value:?}"));
        } else {
            self.fields.push(format!("{}= {:?}", field.name(), value));
        }
    }
}
