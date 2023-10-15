import type {LoggerSink} from "./logger/types";
import {GlobalLogger} from "./logger/global_logger";
import {LogEmitter} from "./logger/log_emitter";

export * from "./logger/types";
export * from "./logger/sinks/console_sink";
export * from "./logger/sinks/store_sink";

const emitter = new LogEmitter();
export const LoggerConfigurator = (sinks: LoggerSink[]) => {
  for (const sink of sinks) {
    emitter.register(sink);
  }
};
export const Log = new GlobalLogger(emitter);