import type {LogEmitter} from "./log_emitter";
import type {LogLevelType} from "./types";
import {LogLevel,} from "./types";

export class GlobalLogger {
  #emitter: LogEmitter;

  constructor(emitter: LogEmitter) {
    this.#emitter = emitter;
  }

  log(message: any, level: LogLevelType): void {
    this.#emitter.emit(message, level);
  }

  error(message: any): void {
    this.log(message, LogLevel.ERROR);
  }

  warn(message: any): void {
    this.log(message, LogLevel.WARN);
  }

  info(message: any): void {
    this.log(message, LogLevel.INFO);
  }

  debug(message: any): void {
    this.log(message, LogLevel.DEBUG);
  }

  trace(message: any): void {
    this.log(message, LogLevel.TRACE);
  }
}
