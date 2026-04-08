use convert_case::{Case, Casing};
use logforth::append::{self, Stdout};

use std::{num::NonZeroUsize, sync::Once};

use crate::layout::Layout as LoggerLayout;
use crate::types::LoggerOptions;

static INIT: Once = Once::new();

pub fn init(options: impl Into<Option<LoggerOptions>>) {
    INIT.call_once(|| {
        let options = options.into().unwrap_or_default();
        let logger_level = options.level.as_filter();
        let prefix = options.prefix.as_deref();
        let prefix_str = prefix.map(|value| value.to_case(Case::Pascal));

        let mut builder = logforth::starter_log::builder().dispatch(|d| {
            d.filter(logger_level)
                .append(Stdout::default().with_layout(LoggerLayout::new(prefix_str.clone(), true)))
        });

        if options.enable_file_logging {
            let log_dir = options
                .log_directory
                .unwrap_or_else(|| "logs".into());
            let file_prefix = prefix
                .map(|value| value.to_case(Case::Lower))
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| "logger".to_string());
            let mut file_builder = append::file::FileBuilder::new(&log_dir, file_prefix)
                .filename_suffix("log")
                .layout(LoggerLayout::new(prefix_str, false))
                .rollover_daily();

            if let Some(max_log_files) = NonZeroUsize::new(options.retention_days.unwrap_or(7) as usize)
            {
                file_builder = file_builder.max_log_files(max_log_files);
            }

            let file_appender = file_builder
                .build()
                .expect("Failed to create file appender");
            builder = builder.dispatch(|d| d.filter(logger_level).append(file_appender));
        }

        builder.apply();
        log::set_max_level(log::LevelFilter::Trace);
    });
}
