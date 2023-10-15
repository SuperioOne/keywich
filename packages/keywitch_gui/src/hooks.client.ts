import {ApplicationSink, ConsoleSink, Log, LoggerConfigurator, LogLevel} from "$lib";

LoggerConfigurator([
  ConsoleSink(LogLevel.DEBUG),
  ApplicationSink(LogLevel.DEBUG, 1000)
]);

Log.debug("EXTREME LONG sad as as asas as asssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss");
Log.error("Halloo error");
Log.error(new Error("Halloo error"));
Log.warn("Halloo warn");
Log.trace("Halloo warn");
Log.info("Halloo Info");
