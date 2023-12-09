import type {LoggerSink, LogLevelType} from "../types";
import {writable} from "svelte/store";

export type LogItem = {
  message: string | Error,
  level: LogLevelType,
  timestamp: number
};

const {subscribe, set, update} = writable<LogItem[]>([]);

/**
 * Application Log Reader is a utility class used to read logs of an application.
 * It provides functionality to subscribe to log events and reset the log.
 */
export const ApplicationLogReader = {
  subscribe,
  reset: () => set([])
};

function format_message(value: any) {
  switch (typeof value) {
    case "undefined":
      return "undefined"
    case "object":
      return value instanceof Error
        ? value.message
        : JSON.stringify(value, undefined, 2);
    case "string":
      return value;
    case "symbol":
      return value.toString();
    default:
      return String(value);
  }
}

/**
 * ApplicationSink is a Svelte store implementation of logger sink that filters log events based on a maximum log
 * level and keeps a history of log events up to a maximum limit.
 * @param maxLevel - The maximum log level to allow. Only log events with a level lower or equal to
 * this will be processed.
 * @param [maxHistory=1000] - The maximum number of log events to keep in history. Defaults to 1000 if not provided.
 * @returns - The logger sink object.
 */
export const ApplicationSink = (maxLevel: LogLevelType, maxHistory = 1000): LoggerSink => {
  return {
    onLogEvent: (message, level) => {
      if (level <= maxLevel) {
        let formattedMessage = format_message(message);
        update((currentBuffer) => {
          currentBuffer.push({
            message: formattedMessage,
            level,
            timestamp: Date.now()
          });

          if (currentBuffer.length > maxHistory) {
            return currentBuffer.splice(0, currentBuffer.length - maxHistory);
          }

          return currentBuffer;
        });
      }
    }
  }
} 