import type {LoggerSink, LogLevelType} from "./types";

export class LogEmitter {
  #listeners: LoggerSink[] = [];

  register(sink: LoggerSink): number {
    return this.#listeners.push(sink);
  }

  emit(message: unknown, level: LogLevelType) {
    for (const listener of this.#listeners) {
      try {
        listener.on_log_event(message, level);
      } catch (err) {
        console.error("Congratulations! the logger's itself throws error...", err);
      }
    }
  }

  async close() {
    for (const listener of this.#listeners) {
      try {
        if (listener.on_close_event) {
          await listener.on_close_event();
        }
      } catch (err) {
        console.error(err);
      }
    }
  }
}