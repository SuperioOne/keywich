import type {LogEmitter} from "./log_emitter";
import type {LogLevelType} from "./types";
import {LogLevel,} from "./types";

export class GlobalLogger {
  #emitter: LogEmitter;

  constructor(emitter: LogEmitter) {
    this.#emitter = emitter;
  }

  log(message: string | Error, level: LogLevelType): void {
    this.#emitter.emit(message, level);
  }

  error(message: string | Error): void {
    this.log(message, LogLevel.ERROR);
  }

  warn(message: string | Error): void {
    this.log(message, LogLevel.WARN);
  }

  info(message: string | Error): void {
    this.log(message, LogLevel.INFO);
  }

  debug(message: string | Error): void {
    this.log(message, LogLevel.DEBUG);
  }

  trace(message: string | Error): void {
    this.log(message, LogLevel.TRACE);
  }
}
