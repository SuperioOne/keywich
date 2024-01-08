import type {LogEmitter} from "./log_emitter";
import type {LogLevelType} from "./types";
import {LogLevel,} from "./types";

export class GlobalLogger {
  #emitter: LogEmitter;

  constructor(emitter: LogEmitter) {
    this.#emitter = emitter;
  }

  log(message: unknown, level: LogLevelType): void {
    this.#emitter.emit(message, level);
  }

  error(message: unknown): void {
    this.log(message, LogLevel.ERROR);
  }

  warn(message: unknown): void {
    this.log(message, LogLevel.WARN);
  }

  info(message: unknown): void {
    this.log(message, LogLevel.INFO);
  }

  debug(message: unknown): void {
    this.log(message, LogLevel.DEBUG);
  }

  trace(message: unknown): void {
    this.log(message, LogLevel.TRACE);
  }
}
