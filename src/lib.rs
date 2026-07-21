mod layout;
mod logger;
mod types;
mod filter;

pub use logger::init;
pub use owo_colors;
pub use types::LoggerOptions;

/// 更新当前日志等级。
pub fn set_level(level: log::LevelFilter) {
    logger::set_current_level(level);
}

/// 获取当前运行时日志等级。
pub fn get_level() -> log::LevelFilter {
    logger::current_level()
}
