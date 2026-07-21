use convert_case::{Case, Casing};
use logforth::append::{self, Stdout};

use std::num::NonZeroUsize;
use std::sync::Once;

use crate::filter::LevelFilter;
use crate::layout::Layout as LoggerLayout;
use crate::types::LoggerOptions;

static INIT: Once = Once::new();

pub fn init(options: impl Into<Option<LoggerOptions>>) {
    INIT.call_once(|| {
        let options = options.into().unwrap_or_default();
        let level = options.level;
        let filter = LevelFilter::new();
        let prefix = options.prefix.map(|s| s.to_case(Case::Pascal));

        let mut builder = logforth::starter_log::builder().dispatch(|d| {
            d.filter(filter.clone())
                .append(Stdout::default().with_layout(LoggerLayout::new(prefix.clone(), true)))
        });

        if options.enable_file_logging {
            let log_dir = options.log_directory.clone();
            let file_prefix = prefix
                .clone()
                .unwrap_or("logger".to_string())
                .to_case(Case::Lower);
            let mut file_builder = append::file::FileBuilder::new(&log_dir, file_prefix)
                .filename_suffix("log")
                .layout(LoggerLayout::new(prefix, false))
                .rollover_daily();

            if let Some(max_log_files) = NonZeroUsize::new(options.retention_days as usize) {
                file_builder = file_builder.max_log_files(max_log_files);
            }

            let file_appender = file_builder
                .build()
                .expect("Failed to create file appender");
            builder = builder.dispatch(|d| d.filter(filter).append(file_appender));
        }

        builder.apply();
        log::set_max_level(level);
    });
}
