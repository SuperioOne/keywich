import {ApplicationSink, ConsoleSink, get_filter_history, Log, LoggerConfigurator, LogLevel} from "$lib";

LoggerConfigurator([
  ConsoleSink(LogLevel.DEBUG),
  ApplicationSink(LogLevel.DEBUG, 1000)
]);