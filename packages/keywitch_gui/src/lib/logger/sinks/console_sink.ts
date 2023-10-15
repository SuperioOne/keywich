import type {LoggerSink, LogLevelType} from "../types";
import {LogLevel} from "../types";

export const ConsoleSink = (maxLevel: LogLevelType): LoggerSink => {
  return {
    onLogEvent: (message, level) => {
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