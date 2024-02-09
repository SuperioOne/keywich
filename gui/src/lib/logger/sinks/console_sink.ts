import type {LoggerSink, LogLevelType} from "../types";
import {LogLevel} from "../types";

/**
 * ConsoleSink is a logger sink that outputs log events to the native console.
 *
 * @param maxLevel - The maximum log level to output.
 * @returns - The logger sink object.
 */
export const ConsoleSink = (maxLevel: LogLevelType): LoggerSink => {
  return {
    on_log_event: (message, level) => {
      if (level <= maxLevel) {
        switch (level) {
          case LogLevel.ERROR:
            console.error(message);
            break;
          case LogLevel.WARN:
            console.warn(message);
            break;
          case LogLevel.INFO:
            console.info(message);
            break;
          case LogLevel.DEBUG:
            console.debug(message);
            break;
          case LogLevel.TRACE:
          default:
            console.trace(message);
            break;
        }
      }
    }
  }
}