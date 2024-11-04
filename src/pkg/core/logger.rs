use chrono::Local;
use config::Config;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::{self, fmt::time::FormatTime};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub fn init(cfg: Option<&Config>) -> WorkerGuard {
    let guard;
    match cfg {
        Some(cfg) => {
            let level = if cfg.get_bool("app.debug").unwrap_or_default() {
                tracing_subscriber::filter::LevelFilter::DEBUG
            } else {
                tracing_subscriber::filter::LevelFilter::INFO
            };
            if cfg.get_string("app.env").unwrap_or(String::from("dev")) == "dev" {}
            let formatting_layer = fmt::layer()
                .pretty()
                .with_writer(std::io::stdout)
                .with_thread_names(true)
                .with_thread_ids(true)
                .compact()
                .with_timer(LocalTimer)
                .with_file(true)
                .with_line_number(true)
                .with_filter(level);

            let file_appender = rolling::daily(
                cfg.get_string("log.path")
                    .unwrap_or(String::from("storage/logs")),
                cfg.get_string("log.filename")
                    .unwrap_or(String::from("server.log")),
            );
            let (non_blocking_appender, _guard) = non_blocking(file_appender);
            let file_layer = fmt::layer()
                .with_thread_names(true)
                .with_thread_ids(true)
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_timer(LocalTimer)
                .json()
                .flatten_event(true)
                .with_ansi(false)
                .with_writer(non_blocking_appender)
                .with_filter(level)
                .boxed();

            Registry::default()
                .with(ErrorLayer::default())
                .with(formatting_layer)
                .with(file_layer)
                .init();

            guard = _guard;
        }
        None => {
            let level = tracing_subscriber::filter::LevelFilter::DEBUG;
            let file_appender = rolling::daily("storage/logs", "server.log");
            let (non_blocking_appender, _guard) = non_blocking(file_appender);
            let file_layer = fmt::layer()
                .with_thread_names(true)
                .with_thread_ids(true)
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_timer(LocalTimer)
                .json()
                .flatten_event(true)
                .with_ansi(false)
                .with_writer(non_blocking_appender)
                .with_filter(level)
                .boxed();
            Registry::default()
                .with(ErrorLayer::default())
                .with(file_layer)
                .init();
            guard = _guard;
        }
    };

    guard
}
