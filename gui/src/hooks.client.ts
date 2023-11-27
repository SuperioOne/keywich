import {ApplicationSink, ConsoleSink, get_filter_history, Log, LoggerConfigurator, LogLevel} from "$lib";

LoggerConfigurator([
  ConsoleSink(LogLevel.DEBUG),
  ApplicationSink(LogLevel.DEBUG, 1000)
]);

get_filter_history().set([
  "abc",
  "abc def ghi",
  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"
])
