use std::time::SystemTime;

use jiff::Timestamp;
use logforth::record;
use owo_colors::OwoColorize;

#[derive(Debug)]
pub(crate) struct Layout {
    prefix: Option<String>,
    color: bool,
}

impl Layout {
    pub(crate) fn new(prefix: Option<String>, color: bool) -> Self {
        Self { prefix, color }
    }

    fn format_timestamp(time: SystemTime) -> String {
        Timestamp::try_from(time)
            .and_then(|time| time.in_tz("Asia/Shanghai"))
            .map(|time| time.strftime("%H:%M:%S%.3f").to_string())
            .unwrap_or_else(|_| "00:00:00.000".to_string())
    }

    fn format_level(&self, level: record::Level) -> String {
        let plain = format!("{:<7}", level.name());

        if !self.color {
            return plain;
        }

        match level {
            record::Level::Error
            | record::Level::Error2
            | record::Level::Error3
            | record::Level::Error4
            | record::Level::Fatal
            | record::Level::Fatal2
            | record::Level::Fatal3
            | record::Level::Fatal4 => format!("{: <7}", level.name().red()),
            record::Level::Warn
            | record::Level::Warn2
            | record::Level::Warn3
            | record::Level::Warn4 => format!("{: <7}", level.name().yellow()),
            record::Level::Info
            | record::Level::Info2
            | record::Level::Info3
            | record::Level::Info4 => format!("{: <7}", level.name().green()),
            record::Level::Debug
            | record::Level::Debug2
            | record::Level::Debug3
            | record::Level::Debug4 => format!("{: <7}", level.name().blue()),
            record::Level::Trace
            | record::Level::Trace2
            | record::Level::Trace3
            | record::Level::Trace4 => format!("{: <7}", level.name().magenta()),
        }
    }
}

impl logforth::Layout for Layout {
    fn format(
        &self,
        record: &record::Record,
        _: &[Box<dyn logforth::Diagnostic>],
    ) -> Result<Vec<u8>, logforth::Error> {
        let prefix = self.prefix.as_ref().map(|prefix| {
            if self.color {
                format!("{}", prefix.magenta())
            } else {
                prefix.to_string()
            }
        });
        let time = Self::format_timestamp(record.time());
        let level = self.format_level(record.level());
        let message = if self.color {
            record.payload().to_string()
        } else {
            match String::from_utf8(strip_ansi_escapes::strip(record.payload())) {
                Ok(cleaned) => cleaned,
                Err(_) => record.payload().to_string(),
            }
        };

        let prefix_part = prefix
            .map(|prefix| format!("[{prefix}] "))
            .unwrap_or_default();

        Ok(format!("{prefix_part}[{time}] [{level}] {message}").into_bytes())
    }
}