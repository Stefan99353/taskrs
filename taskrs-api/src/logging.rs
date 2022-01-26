use crate::config::LogConfig;
use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{reload, EnvFilter, Registry};

pub fn get_subscriber(
    log_config: &LogConfig,
) -> (
    impl Subscriber + Sync + Send,
    reload::Handle<EnvFilter, Registry>,
    Vec<WorkerGuard>,
) {
    let mut guards = vec![];

    // EnvFilter Layer
    let (env_filter, reload_handle) = reload::Layer::new(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&log_config.rust_log)),
    );

    // File Layer
    let file_layer = match &log_config.log_to_file {
        true => {
            let file_appender =
                tracing_appender::rolling::daily(&log_config.log_dir, &log_config.log_prefix);
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            guards.push(guard);

            let file_layer = Layer::new()
                .with_writer(non_blocking)
                .event_format(tracing_subscriber::fmt::format().json())
                .fmt_fields(tracing_subscriber::fmt::format::JsonFields::new());
            Some(file_layer)
        }
        false => None,
    };

    // StdOut Layer
    let stdout_layer = match &log_config.log_to_stdout {
        true => Some(Layer::new()),
        false => None,
    };

    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer);

    (subscriber, reload_handle, guards)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber")
}
