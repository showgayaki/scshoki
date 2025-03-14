use std::fs;
use std::sync::Mutex;
use chrono::Local;
use tracing::{info, Event, Subscriber};
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{self, format::Writer},
    layer::{SubscriberExt, Layer},
    EnvFilter, Registry,
    registry::LookupSpan,
};

use crate::config::constants::{LOG_DIR, LOG_FILE_NAME};


pub fn init_logger() {
    // ログディレクトリを作成
    if !LOG_DIR.exists() {
        fs::create_dir_all(&**LOG_DIR).expect("Failed to create log directory");
    }

    let log_writer = rolling::never(&**LOG_DIR, LOG_FILE_NAME);
    let log_writer = Mutex::new(log_writer);

    let file_layer = fmt::layer()
        .with_writer(log_writer)
        .with_ansi(false)
        .event_format(CustomLogFormat)
        .with_filter(EnvFilter::new("info"));

    let subscriber = Registry::default().with(file_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    info!("Logger initialized. Log file: {}/{}", LOG_DIR.display(), LOG_FILE_NAME);
}


// カスタムログフォーマット
struct CustomLogFormat;

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for CustomLogFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: Writer,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S%:z");

        // フォーマット例
        // [2025-03-14 16:38:00+09:00] [INFO] [src/services/logger.rs:31]: message
        write!(
            &mut writer,
            "[{}] [{}] [{}:{}]: ",
            now,
            meta.level(),
            meta.file().unwrap_or("unknown"),
            meta.line().unwrap_or(0)
        )?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
