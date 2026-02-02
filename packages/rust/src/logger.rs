use chrono_tz::Asia::Shanghai;
use convert_case::{Case, Casing};
use owo_colors::OwoColorize;
use std::{fmt, str::FromStr, sync::Once};
use tracing::{
    Subscriber,
    field::{Field, Visit},
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    Layer,
    filter::LevelFilter,
    fmt::{FormatEvent, FormatFields},
    layer::SubscriberExt,
    registry::LookupSpan,
};

static INIT: Once = Once::new();

pub struct LoggerOptions {
    /// 日志等级
    pub level: String,
    /// 是否启用文件日志记录
    pub enable_file_logging: bool,
    /// 自定义前缀
    pub prefix: Option<String>,
    /// 日志文件保存路径
    pub log_directory: Option<String>,
    /// 日志文件保留天数
    pub retention_days: Option<u8>,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            enable_file_logging: false,
            prefix: None,
            log_directory: None,
            retention_days: None,
        }
    }
}

impl LoggerOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置日志等级
    pub fn with_level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }
    /// 设置是否启用文件日志记录
    pub fn with_file_logging(mut self, enable: bool) -> Self {
        self.enable_file_logging = enable;
        self
    }

    /// 设置自定义前缀
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    /// 设置日志文件保存目录
    pub fn with_log_directory(mut self, directory: impl Into<String>) -> Self {
        self.log_directory = Some(directory.into());
        self
    }
    /// 设置日志文件保留天数
    pub fn with_retention_days(mut self, days: u8) -> Self {
        self.retention_days = Some(days);
        self
    }
}

struct MessageVisitor<'a, W> {
    writer: &'a mut W,
    strip_ansi: bool,
}

impl<'a, W> MessageVisitor<'a, W> {
    fn new(writer: &'a mut W, strip_ansi: bool) -> Self {
        Self { writer, strip_ansi }
    }
}

impl<'a, W> Visit for MessageVisitor<'a, W>
where
    W: fmt::Write,
{
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            if self.strip_ansi {
                let cleaned_bytes = strip_ansi_escapes::strip(value);
                if let Ok(cleaned) = String::from_utf8(cleaned_bytes) {
                    let _ = write!(self.writer, "{}", cleaned);
                } else {
                    let _ = write!(self.writer, "{}", value);
                }
            } else {
                let _ = write!(self.writer, "{}", value);
            }
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            if self.strip_ansi {
                let formatted = format!("{:?}", value);
                let cleaned_bytes = strip_ansi_escapes::strip(&formatted);
                if let Ok(cleaned) = String::from_utf8(cleaned_bytes) {
                    let _ = write!(self.writer, "{}", cleaned);
                } else {
                    let _ = write!(self.writer, "{}", formatted);
                }
            } else {
                let _ = write!(self.writer, "{:?}", value);
            }
        }
    }
}

struct Formatter {
    prefix: String,
    color: bool,
}

impl<S, N> FormatEvent<S, N> for Formatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        if self.color {
            write!(writer, "[{}] ", self.prefix.magenta())?;
        } else {
            write!(writer, "[{}] ", &self.prefix)?;
        }

        let shanghai_time = chrono::Local::now().with_timezone(&Shanghai);
        write!(writer, "[{}] ", shanghai_time.format("%H:%M:%S%.3f"))?;

        let level = event.metadata().level();
        if self.color {
            use tracing::Level;
            match *level {
                Level::ERROR => write!(writer, "[{: <7}] ", level.red())?,
                Level::WARN => write!(writer, "[{: <7}] ", level.yellow())?,
                Level::INFO => write!(writer, "[{: <7}] ", level.green())?,
                Level::DEBUG => write!(writer, "[{: <7}] ", level.blue())?,
                Level::TRACE => write!(writer, "[{: <7}] ", level.magenta())?,
            }
        } else {
            write!(writer, "[{: <7}] ", level)?;
        }

        let mut visitor = MessageVisitor::new(&mut writer, !self.color);
        event.record(&mut visitor);

        writeln!(writer)
    }
}

pub fn init(options: impl Into<Option<LoggerOptions>>) {
    INIT.call_once(|| {
        let options = options.into().unwrap_or_default();

        let logger_level = options
            .level
            .parse::<LogLevel>()
            .unwrap_or(LogLevel(LevelFilter::INFO))
            .0;
        let prefix = options.prefix.as_deref().unwrap_or("puniyu");
        let prefix_str = prefix.to_case(Case::Pascal);

        let console_subscriber = tracing_subscriber::fmt::layer()
            .event_format(Formatter {
                prefix: prefix_str.clone(),
                color: true,
            })
            .with_filter(logger_level);

        let mut layers = vec![console_subscriber.boxed()];

        if options.enable_file_logging {
            let log_dir = options.log_directory.unwrap_or_else(|| "logs".to_string());
            let _ = std::fs::create_dir_all(&log_dir);
            let file_appender = RollingFileAppender::builder()
                .rotation(Rotation::DAILY)
                .filename_prefix(prefix.to_case(Case::Lower))
                .filename_suffix("log")
                .max_log_files(options.retention_days.unwrap_or(7) as usize)
                .build(&log_dir)
                .expect("Failed to create file appender");

            let file_subscriber = tracing_subscriber::fmt::layer()
                .event_format(Formatter {
                    prefix: prefix_str,
                    color: false,
                })
                .with_writer(file_appender)
                .with_ansi(false)
                .with_filter(logger_level);

            layers.push(file_subscriber.boxed());
        }

        let subscriber = tracing_subscriber::registry().with(layers);
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global subscriber");
        tracing_log::LogTracer::init().expect("Failed to initialize LogTracer");
    });
}

struct LogLevel(LevelFilter);

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let filter = match s.to_lowercase().as_str() {
            "trace" => LevelFilter::TRACE,
            "debug" => LevelFilter::DEBUG,
            "info" => LevelFilter::INFO,
            "warn" | "warning" => LevelFilter::WARN,
            "error" => LevelFilter::ERROR,
            "off" => LevelFilter::OFF,
            _ => LevelFilter::INFO,
        };
        Ok(LogLevel(filter))
    }
}
