use std::path::PathBuf;
use log::info;
use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
        RollingFileAppender,
    },
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};

use crate::config::constants::{
    LOG_DIR,
    LOG_FILE_NAME,
    LOG_ROTATE_BASE,
    LOG_ROTATE_COUNT,
    LOG_ROTATE_SIZE,
};


pub fn init_logger() {
    // ログのフォーマット
    const LOG_PATTERN: &str = "[{d(%Y-%m-%d %H:%M:%S%:z)}] [{l}] [{f}:{L}]: {m}{n}";

    // `scshoki.log` を `scshoki-{}.log` に変換
    let log_pattern = {
        let path = PathBuf::from(LOG_FILE_NAME);
        if let Some(ext) = path.extension() {
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            format!("{}/{}-{{}}.{}", LOG_DIR.display(), stem, ext.to_string_lossy()) // scshoki-{}.log
        } else {
            format!("{}/{}-{{}}", LOG_DIR.display(), LOG_FILE_NAME) // scshoki-{}
        }
    };

    let stdout = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            LOG_PATTERN,
        )))
        .build();

    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            LOG_PATTERN,
        )))
        .build(
            &LOG_DIR.join(LOG_FILE_NAME),
            Box::new(CompoundPolicy::new(
                Box::new(SizeTrigger::new(LOG_ROTATE_SIZE)),
                Box::new(
                    FixedWindowRoller::builder()
                        .base(LOG_ROTATE_BASE)
                        .build(&log_pattern, LOG_ROTATE_COUNT)
                        .unwrap(),
                ),
            )),
        )
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(
            Logger::builder()
                .appender("file")
                .build("default_logger", log::LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(log::LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("Logger initialized. Log file: {}", LOG_DIR.join(LOG_FILE_NAME).display());
}
