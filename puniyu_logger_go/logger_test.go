package logger

import "testing"

func TestInitDefault(t *testing.T) {
	InitDefault()
	Info("猪咪")
}

func TestInitWithOptions(t *testing.T) {
	Init(&LoggerOptions{
		Level:             "debug",
		EnableFileLogging: true,
		Prefix:            "test",
		LogDirectory:      "./logs",
		RetentionDays:     3,
	})

	Trace("这是 trace 日志")
	Debug("这是 debug 日志")
	Info("这是 info 日志")
	Warn("这是 warn 日志")
	Error("这是 error 日志")
}