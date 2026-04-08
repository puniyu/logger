use owo_colors::OwoColorize;
use puniyu_logger::{LogLevel, LoggerOptions, init as log_init};

#[test]
fn log_with_options() {
    let options = LoggerOptions::default()
        .with_file_logging(true)
        .with_log_directory("logs")
        .with_retention_days(7);
    log_init(Some(options));
    let msg = "猪咪".fg_rgb::<255, 182, 193>();
    log::info!("{}", msg);
}

#[test]
fn log_info() {
    log_init(None);
    log::info!("{}", "info");
}

#[test]
fn log_error() {
    log_init(None);
    log::error!("{}", "error");
}

#[test]
fn log_warn() {
    log_init(None);
    log::warn!("{}", "warn");
}

#[test]
fn log_debug() {
    log_init(None);
    log::debug!("{}", "debug");
}

#[test]
fn log_trace() {
    log_init(None);
    log::trace!("{}", "trace");
}

#[test]
fn init_is_idempotent() {
    log_init(None);
    log_init(Some(LoggerOptions::default().with_level(LogLevel::Debug)));
    log::info!("{}", "idempotent");
}

#[test]
fn explicit_info_level_works() {
    let options = LoggerOptions::default().with_level(LogLevel::Info);
    log_init(Some(options));
    log::info!("{}", "fallback");
}
