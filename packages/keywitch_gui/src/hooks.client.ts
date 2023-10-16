import {ApplicationSink, ConsoleSink, Log, LoggerConfigurator, LogLevel} from "$lib";

LoggerConfigurator([
  ConsoleSink(LogLevel.DEBUG),
  ApplicationSink(LogLevel.DEBUG, 1000)
]);
