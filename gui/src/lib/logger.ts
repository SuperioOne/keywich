import {type LoggerSink, LogLevel, type LogLevelType} from "./logger/types";
import {GlobalLogger} from "./logger/global_logger";
import {LogEmitter} from "./logger/log_emitter";

export * from "./logger/types";
export * from "./logger/sinks/console_sink";
export * from "./logger/sinks/store_sink";

const emitter = new LogEmitter();

/**
 * Configures the logger with the given sinks.
 *
 * @param sinks - The sinks to register with the logger.
 */
export const LoggerConfigurator = (sinks: LoggerSink[]) => {
  for (const sink of sinks) {
    emitter.register(sink);
  }
};

/**
 * Global Logger. Represents a global logger instance with an emitter
 */
export const Log = new GlobalLogger(emitter);

export function try_parse_log_level(value: string): LogLevelType | undefined {
  switch (value?.toUpperCase()) {
    case "ERROR":
      return LogLevel.ERROR;
    case "WARN":
      return LogLevel.WARN;
    case "INFO":
      return LogLevel.INFO;
    case "DEBUG":
      return LogLevel.DEBUG;
    case "TRACE":
      return LogLevel.TRACE;
    default:
      return undefined;
  }
}