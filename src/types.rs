use std::path::PathBuf;

use log::{Level, LevelFilter};
use logforth::record::{Level as LogforthLevel, LevelFilter as LogforthLevelFilter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl From<Level> for LogLevel {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::Error,
            Level::Warn => Self::Warn,
            Level::Info => Self::Info,
            Level::Debug => Self::Debug,
            Level::Trace => Self::Trace,
        }
    }
}

impl From<LevelFilter> for LogLevel {
    fn from(value: LevelFilter) -> Self {
        match value {
            LevelFilter::Off => Self::Off,
            LevelFilter::Error => Self::Error,
            LevelFilter::Warn => Self::Warn,
            LevelFilter::Info => Self::Info,
            LevelFilter::Debug => Self::Debug,
            LevelFilter::Trace => Self::Trace,
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

impl LogLevel {
    pub const fn to_u8(self) -> u8 {
        match self {
            Self::Trace => 0,
            Self::Debug => 1,
            Self::Info => 2,
            Self::Warn => 3,
            Self::Error => 4,
            Self::Off => 5,
        }
    }

    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Trace,
            1 => Self::Debug,
            2 => Self::Info,
            3 => Self::Warn,
            4 => Self::Error,
            _ => Self::Off,
        }
    }
}

impl From<LogLevel> for LogforthLevel {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace | LogLevel::Off => Self::Trace,
        }
    }
}

impl From<LogLevel> for LogforthLevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::MoreSevereEqual(LogforthLevel::Error),
            LogLevel::Warn => Self::MoreSevereEqual(LogforthLevel::Warn),
            LogLevel::Info => Self::MoreSevereEqual(LogforthLevel::Info),
            LogLevel::Debug => Self::MoreSevereEqual(LogforthLevel::Debug),
            LogLevel::Trace => Self::MoreSevereEqual(LogforthLevel::Trace),
        }
    }
}


pub struct LoggerOptions {
    /// 日志等级
    pub level: LevelFilter,
    /// 是否启用文件日志记录
    pub enable_file_logging: bool,
    /// 自定义前缀
    pub prefix: Option<String>,
    /// 日志文件保存路径
    pub log_directory: PathBuf,
    /// 日志文件保留天数
    pub retention_days: u8,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            level: LevelFilter::Info,
            enable_file_logging: false,
            prefix: None,
            log_directory: PathBuf::from("logs"),
            retention_days: 7,
        }
    }
}

impl LoggerOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置日志等级
    pub fn with_level(mut self, level: LevelFilter) -> Self {
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
        self.log_directory = directory.into();
        self
    }

    /// 设置日志文件保留天数
    pub fn with_retention_days(mut self, days: u8) -> Self {
        self.retention_days = days;
        self
    }
}
