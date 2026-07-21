use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

use crate::types::LogLevel;

#[derive(Clone, Debug)]
pub(crate) struct AtomicFilter {
    level: Arc<AtomicU8>,
}

impl AtomicFilter {
    pub(crate) fn new(level: LogLevel) -> Self {
        Self {
            level: Arc::new(AtomicU8::new(level.to_u8())),
        }
    }

    pub(crate) fn set_level(&self, level: LogLevel) {
        self.level.store(level.to_u8(), Ordering::Relaxed);
    }

    pub(crate) fn current_level(&self) -> LogLevel {
        LogLevel::from_u8(self.level.load(Ordering::Relaxed))
    }
}

impl logforth::filter::Filter for AtomicFilter {
    fn enabled(
        &self,
        criteria: &logforth::record::FilterCriteria,
        _: &[Box<dyn logforth::Diagnostic>],
    ) -> logforth::filter::FilterResult {
        let current = self.current_level();
        if current == LogLevel::Off {
            return logforth::filter::FilterResult::Reject;
        }

        if criteria.level() <= logforth::record::Level::from(current) {
            logforth::filter::FilterResult::Neutral
        } else {
            logforth::filter::FilterResult::Reject
        }
    }
}
