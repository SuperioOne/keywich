import type {LoggerSink, LogLevelType} from "../types";
import {writable} from "svelte/store";

export type LogItem = {
  message: string | Error,
  level: LogLevelType,
  timestamp: number
};

const {subscribe, set, update} = writable<LogItem[]>([]);
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
            return currentBuffer.slice(currentBuffer.length - maxHistory);
          } else {
            return currentBuffer;
          }
        });
      }
    }
  }
} 