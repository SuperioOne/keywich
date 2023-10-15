import type {LoggerSink, LogLevelType} from "./types";

export class LogEmitter {
  #listeners: LoggerSink[] = [];

  register(sink: LoggerSink): number {
    return this.#listeners.push(sink);
  }

  emit(message: string | Error, level: LogLevelType) {
    for (const listener of this.#listeners) {
      try {
        listener.onLogEvent(message, level);
      } catch (err) {
        console.error("Congratulations! the logger's itself throws error...", err);
      }
    }
  }

  async close() {
    for (const listener of this.#listeners) {
      try {
        if (listener.onCloseEvent) {
          await listener.onCloseEvent();
        }
      } catch (err) {
        console.error(err);
      }
    }
  }
}