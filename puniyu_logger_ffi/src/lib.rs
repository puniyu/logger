//! puniyu_logger 的 C FFI 接口导出
//!
//! 提供 C 语言可调用的日志接口

use puniyu_logger::LoggerOptions as PuniyuLoggerOptions;
use std::ffi::{CStr, c_char, c_int};
use puniyu_logger::{debug, error, info, trace, warn};


#[repr(C)]
pub struct LoggerOptions {
    /// 日志等级 (C 字符串指针，可为 NULL 使用默认值 "info")
    pub level: *const c_char,
    /// 是否启用文件日志记录 (0 = false, 非0 = true)
    pub enable_file_logging: c_int,
    /// 自定义前缀 (C 字符串指针，可为 NULL)
    pub prefix: *const c_char,
    /// 日志文件保存路径 (C 字符串指针，可为 NULL)
    pub log_directory: *const c_char,
    /// 日志文件保留天数 (0 表示使用默认值 7)
    pub retention_days: u8,
}

fn c_str_to_option(ptr: *const c_char) -> Option<String> {
    (!ptr.is_null())
        .then(|| unsafe { CStr::from_ptr(ptr) })
        .and_then(|s| s.to_str().ok())
        .map(Into::into)
}

/// 初始化日志系统（使用默认配置）
#[unsafe(no_mangle)]
pub extern "C" fn logger_init_default() {
    puniyu_logger::init(None);
}

/// 初始化日志系统（使用自定义配置，options 为 NULL 时使用默认配置）
///
/// # Safety
/// 调用者需确保 `options` 是有效的 `LoggerOptions` 指针或 NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn puniyu_logger_init(options: *const LoggerOptions) {
    let rust_options = (!options.is_null()).then(|| {
        let opts = unsafe { &*options };
        let mut o = PuniyuLoggerOptions::default().with_file_logging(opts.enable_file_logging != 0);
        if let Some(v) = c_str_to_option(opts.level) { o = o.with_level(&v); }
        if let Some(v) = c_str_to_option(opts.prefix) { o = o.with_prefix(&v); }
        if let Some(v) = c_str_to_option(opts.log_directory) { o = o.with_log_directory(v); }
        if opts.retention_days > 0 { o = o.with_retention_days(opts.retention_days); }
        o
    });
    puniyu_logger::init(rust_options);
}

/// 记录 TRACE 级别日志
///
/// # Safety
/// 调用者需确保 `message` 是有效的 C 字符串
#[unsafe(no_mangle)]
pub extern "C" fn puniyu_log_trace(message: *const c_char) {
    if let Some(msg) = c_str_to_option(message) {
        trace!("{}", msg);
    }
}

/// 记录 DEBUG 级别日志
#[unsafe(no_mangle)]
pub extern "C" fn log_debug(message: *const c_char) {
    if let Some(msg) = c_str_to_option(message) {
        debug!("{}", msg);
    }
}

/// 记录 INFO 级别日志
#[unsafe(no_mangle)]
pub extern "C" fn log_info(message: *const c_char) {
    if let Some(msg) = c_str_to_option(message) {
        info!("{}", msg);
    }
}

/// 记录 WARN 级别日志
#[unsafe(no_mangle)]
pub extern "C" fn log_warn(message: *const c_char) {
    if let Some(msg) = c_str_to_option(message) {
        warn!("{}", msg);
    }
}

/// 记录 ERROR 级别日志
#[unsafe(no_mangle)]
pub extern "C" fn log_error(message: *const c_char) {
    if let Some(msg) = c_str_to_option(message) {
        error!("{}", msg);
    }
}
