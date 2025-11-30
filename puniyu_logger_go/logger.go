package logger

/*
#cgo CFLAGS: -I${SRCDIR}
#cgo linux LDFLAGS: ${SRCDIR}/libpuniyu_logger_ffi.a -lm -ldl -lpthread
#cgo windows LDFLAGS: ${SRCDIR}/puniyu_logger_ffi.lib -lws2_32 -luserenv -lntdll -lbcrypt
#include "puniyu_logger.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

// LoggerOptions 日志配置选项
type LoggerOptions struct {
	Level            string
	EnableFileLogging bool
	Prefix           string
	LogDirectory     string
	RetentionDays    uint8
}

// InitDefault 使用默认配置初始化日志
func InitDefault() {
	C.logger_init_default()
}

// Init 使用自定义配置初始化日志
func Init(opts *LoggerOptions) {
	if opts == nil {
		C.puniyu_logger_init(nil)
		return
	}

	var cOpts C.struct_LoggerOptions

	if opts.Level != "" {
		cLevel := C.CString(opts.Level)
		defer C.free(unsafe.Pointer(cLevel))
		cOpts.level = cLevel
	}

	if opts.EnableFileLogging {
		cOpts.enable_file_logging = 1
	}

	if opts.Prefix != "" {
		cPrefix := C.CString(opts.Prefix)
		defer C.free(unsafe.Pointer(cPrefix))
		cOpts.prefix = cPrefix
	}

	if opts.LogDirectory != "" {
		cDir := C.CString(opts.LogDirectory)
		defer C.free(unsafe.Pointer(cDir))
		cOpts.log_directory = cDir
	}

	cOpts.retention_days = C.uint8_t(opts.RetentionDays)

	C.puniyu_logger_init(&cOpts)
}

// Trace 记录 TRACE 级别日志
func Trace(msg string) {
	cMsg := C.CString(msg)
	defer C.free(unsafe.Pointer(cMsg))
	C.puniyu_log_trace(cMsg)
}

// Debug 记录 DEBUG 级别日志
func Debug(msg string) {
	cMsg := C.CString(msg)
	defer C.free(unsafe.Pointer(cMsg))
	C.log_debug(cMsg)
}

// Info 记录 INFO 级别日志
func Info(msg string) {
	cMsg := C.CString(msg)
	defer C.free(unsafe.Pointer(cMsg))
	C.log_info(cMsg)
}

// Warn 记录 WARN 级别日志
func Warn(msg string) {
	cMsg := C.CString(msg)
	defer C.free(unsafe.Pointer(cMsg))
	C.log_warn(cMsg)
}

// Error 记录 ERROR 级别日志
func Error(msg string) {
	cMsg := C.CString(msg)
	defer C.free(unsafe.Pointer(cMsg))
	C.log_error(cMsg)
}