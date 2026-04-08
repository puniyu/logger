use std::{path::PathBuf, str::FromStr};

use logforth::record;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let level = match s.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "off" => LogLevel::Off,
            _ => return Err(()),
        };

        Ok(level)
    }
}

impl LogLevel {
    pub(crate) const fn as_filter(self) -> record::LevelFilter {
        match self {
            LogLevel::Trace => record::LevelFilter::MoreSevereEqual(record::Level::Trace),
            LogLevel::Debug => record::LevelFilter::MoreSevereEqual(record::Level::Debug),
            LogLevel::Info => record::LevelFilter::MoreSevereEqual(record::Level::Info),
            LogLevel::Warn => record::LevelFilter::MoreSevereEqual(record::Level::Warn),
            LogLevel::Error => record::LevelFilter::MoreSevereEqual(record::Level::Error),
            LogLevel::Off => record::LevelFilter::Off,
        }
    }
}

pub struct LoggerOptions {
    /// 日志等级
    pub level: LogLevel,
    /// 是否启用文件日志记录
    pub enable_file_logging: bool,
    /// 自定义前缀
    pub prefix: Option<String>,
    /// 日志文件保存路径
    pub log_directory: Option<PathBuf>,
    /// 日志文件保留天数
    pub retention_days: Option<u8>,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            enable_file_logging: false,
            prefix: None,
            log_directory: Some(PathBuf::from("logs")),
            retention_days: Some(7),
        }
    }
}

impl LoggerOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置日志等级
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// 设置是否启用文件日志记录
    pub fn with_file_logging(mut self, enable: bool) -> Self {
        self.enable_file_logging = enable;
        self
    }

    /// 设置自定义前缀
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// 设置日志文件保存目录
    pub fn with_log_directory(mut self, directory: impl Into<PathBuf>) -> Self {
        self.log_directory = Some(directory.into());
        self
    }

    /// 设置日志文件保留天数
    pub fn with_retention_days(mut self, days: u8) -> Self {
        self.retention_days = Some(days);
        self
    }
}