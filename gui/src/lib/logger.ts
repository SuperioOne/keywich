import type {LoggerSink} from "./logger/types";
import {GlobalLogger} from "./logger/global_logger";
import {LogEmitter} from "./logger/log_emitter";

export * from "./logger/types";
export * from "./logger/sinks/console_sink";
export * from "./logger/sinks/store_sink";

const emitter = new LogEmitter();

/**
 * Configures the logger with the given sinks.
 *
 * @param sinks - The sinks to register with the logger.
 */
export const LoggerConfigurator = (sinks: LoggerSink[]) => {
  for (const sink of sinks) {
    emitter.register(sink);
  }
};

/**
 * Global Logger. Represents a global logger instance with an emitter
 */
export const Log = new GlobalLogger(emitter);