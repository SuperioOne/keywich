import {ApplicationSink, ConsoleSink, LoggerConfigurator, LogLevel} from "$lib";

LoggerConfigurator([
  ConsoleSink(LogLevel.DEBUG),
  ApplicationSink(LogLevel.DEBUG, 1000)
]);