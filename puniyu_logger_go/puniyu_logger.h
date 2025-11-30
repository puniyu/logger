#ifdef _WIN32
  #ifdef PUNIYU_LOGGER_EXPORTS
    #define PUNIYU_LOGGER __declspec(dllexport)
  #else
    #define PUNIYU_LOGGER __declspec(dllimport)
  #endif
#else
  #define PUNIYU_LOGGER
#endif


#ifndef PUNIYU_LOGGER_H
#define PUNIYU_LOGGER_H

/* 警告: 此文件由 cbindgen 自动生成，请勿手动修改 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct LoggerOptions {
  /**
   * 日志等级 (C 字符串指针，可为 NULL 使用默认值 "info")
   */
  const char *level;
  /**
   * 是否启用文件日志记录 (0 = false, 非0 = true)
   */
  int enable_file_logging;
  /**
   * 自定义前缀 (C 字符串指针，可为 NULL)
   */
  const char *prefix;
  /**
   * 日志文件保存路径 (C 字符串指针，可为 NULL)
   */
  const char *log_directory;
  /**
   * 日志文件保留天数 (0 表示使用默认值 7)
   */
  uint8_t retention_days;
} LoggerOptions;

/**
 * 初始化日志系统（使用默认配置）
 */
PUNIYU_LOGGER void logger_init_default(void);

/**
 * 初始化日志系统（使用自定义配置，options 为 NULL 时使用默认配置）
 *
 * # Safety
 * 调用者需确保 `options` 是有效的 `LoggerOptions` 指针或 NULL
 */
PUNIYU_LOGGER void puniyu_logger_init(const struct LoggerOptions *options);

/**
 * 记录 TRACE 级别日志
 *
 * # Safety
 * 调用者需确保 `message` 是有效的 C 字符串
 */
PUNIYU_LOGGER void puniyu_log_trace(const char *message);

/**
 * 记录 DEBUG 级别日志
 */
PUNIYU_LOGGER void log_debug(const char *message);

/**
 * 记录 INFO 级别日志
 */
PUNIYU_LOGGER void log_info(const char *message);

/**
 * 记录 WARN 级别日志
 */
PUNIYU_LOGGER void log_warn(const char *message);

/**
 * 记录 ERROR 级别日志
 */
PUNIYU_LOGGER void log_error(const char *message);

#endif  /* PUNIYU_LOGGER_H */
