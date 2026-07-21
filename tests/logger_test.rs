use log::LevelFilter;
use owo_colors::OwoColorize;
use puniyu_logger::{LoggerOptions, init as log_init};
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
fn set_max_level_updates_runtime_level() {
    log_init(None);

    log::set_max_level(LevelFilter::Trace);
    assert_eq!(log::max_level(), LevelFilter::Trace);
    assert!(log::log_enabled!(log::Level::Trace));
    log::trace!("trace via set_max_level");

    log::set_max_level(LevelFilter::Warn);
    assert_eq!(log::max_level(), LevelFilter::Warn);
    assert!(!log::log_enabled!(log::Level::Info));
    assert!(log::log_enabled!(log::Level::Warn));

    log::set_max_level(LevelFilter::Info);
}

#[test]
#[serial]
fn set_max_level_consistent_with_log_enabled() {
    log_init(None);

    for level in [
        LevelFilter::Off,
        LevelFilter::Error,
        LevelFilter::Warn,
        LevelFilter::Info,
        LevelFilter::Debug,
        LevelFilter::Trace,
    ] {
        log::set_max_level(level);
        assert_eq!(
            log::max_level(),
            level,
            "max_level should match for {level:?}"
        );

        let log_level = match level {
            LevelFilter::Off => None,
            LevelFilter::Error => Some(log::Level::Error),
            LevelFilter::Warn => Some(log::Level::Warn),
            LevelFilter::Info => Some(log::Level::Info),
            LevelFilter::Debug => Some(log::Level::Debug),
            LevelFilter::Trace => Some(log::Level::Trace),
        };

        if let Some(log_level) = log_level {
            assert!(
                log::log_enabled!(log_level),
                "log_enabled!({log_level:?}) should be true when max_level == {level:?}"
            );
        }
    }

    log::set_max_level(LevelFilter::Info);
}
