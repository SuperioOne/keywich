import type {LoggerSink, LogLevelType} from "../types";
import {writable, readonly} from "svelte/store";

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

export const ApplicationSink = (maxLevel: LogLevelType, maxHistory = 1000): LoggerSink => {
  return {
    onLogEvent: (message, level) => {
      if (level <= maxLevel) {
        update((currentBuffer) => {
          currentBuffer.push({
            message,
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