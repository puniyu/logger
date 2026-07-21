
#[derive(Clone, Debug)]
pub(crate) struct LevelFilter;

impl LevelFilter {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl logforth::filter::Filter for LevelFilter {
    fn enabled(
        &self,
        criteria: &logforth::record::FilterCriteria,
        _: &[Box<dyn logforth::Diagnostic>],
    ) -> logforth::filter::FilterResult {
        use log::LevelFilter;
        let max = log::max_level();
        if max == LevelFilter::Off {
            return logforth::filter::FilterResult::Reject;
        }

        let max_level = max.to_level().unwrap_or(log::Level::Error);
        let request_level = match criteria.level() {
            logforth::record::Level::Error
            | logforth::record::Level::Error2
            | logforth::record::Level::Error3
            | logforth::record::Level::Error4
            | logforth::record::Level::Fatal
            | logforth::record::Level::Fatal2
            | logforth::record::Level::Fatal3
            | logforth::record::Level::Fatal4 => log::Level::Error,
            logforth::record::Level::Warn
            | logforth::record::Level::Warn2
            | logforth::record::Level::Warn3
            | logforth::record::Level::Warn4 => log::Level::Warn,
            logforth::record::Level::Info
            | logforth::record::Level::Info2
            | logforth::record::Level::Info3
            | logforth::record::Level::Info4 => log::Level::Info,
            logforth::record::Level::Debug
            | logforth::record::Level::Debug2
            | logforth::record::Level::Debug3
            | logforth::record::Level::Debug4 => log::Level::Debug,
            logforth::record::Level::Trace
            | logforth::record::Level::Trace2
            | logforth::record::Level::Trace3
            | logforth::record::Level::Trace4 => log::Level::Trace,
        };

        if request_level <= max_level {
            logforth::filter::FilterResult::Neutral
        } else {
            logforth::filter::FilterResult::Reject
        }
    }
}
