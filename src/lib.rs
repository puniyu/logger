mod layout;
mod logger;
mod types;
mod filter;

pub use logger::init;
pub use owo_colors;
pub use types::LoggerOptions;

/// 运行时更新当前日志等级。
///
/// 此接口仅修改进程内 facade 的全局最大等级，不重新应用 `logforth` 的输出过滤。
/// 若在初始化时已固定 dispatch 级别，仅靠此函数无法改变已应用的 dispatch 阈值。
pub fn set_level(level: log::LevelFilter) {
    logger::set_current_level(level);
}

/// 获取当前维护的运行时日志等级。
pub fn get_level() -> log::LevelFilter {
    logger::current_level()
}
