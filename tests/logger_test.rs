use log::LevelFilter;
use owo_colors::OwoColorize;
use puniyu_logger::{LoggerOptions, get_level, init as log_init, set_level};
use serial_test::serial;

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
    log_init(Some(
        LoggerOptions::default().with_level(LevelFilter::Debug),
    ));
    log::info!("{}", "idempotent");
}

#[test]
fn explicit_info_level_works() {
    let options = LoggerOptions::default().with_level(LevelFilter::Info);
    log_init(Some(options));
    log::info!("{}", "fallback");
}

#[test]
#[serial]
fn runtime_level_can_be_updated() {
    log_init(None);
    let saved = get_level();
    assert_eq!(saved, LevelFilter::Info);
    set_level(LevelFilter::Debug);
    assert_eq!(get_level(), LevelFilter::Debug);
    log::debug!("{}", "runtime updated");
    set_level(saved);
    assert_eq!(get_level(), saved);
}

#[test]
#[serial]
fn runtime_level_increase_is_effective() {
    log_init(Some(LoggerOptions::default().with_level(LevelFilter::Info)));

    set_level(LevelFilter::Trace);
    assert_eq!(get_level(), LevelFilter::Trace);
    assert!(log::log_enabled!(log::Level::Trace));

    set_level(LevelFilter::Warn);
    assert_eq!(get_level(), LevelFilter::Warn);
    assert!(!log::log_enabled!(log::Level::Info));
    assert!(log::log_enabled!(log::Level::Warn));
}
