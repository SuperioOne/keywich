import type {LogLevelType} from "$lib";
import {
  ApplicationSink,
  ConsoleSink,
  LoggerConfigurator,
  LogLevel,
  try_parse_log_level,
  configStore,
  i18nStore,
  RPC
} from "$lib";
import {env} from "$env/dynamic/public";
import {or_default} from "@keywich/api/utils";

const LOG_LEVEL: LogLevelType = or_default(try_parse_log_level(env.PUBLIC_KW_LOG_LEVEL), LogLevel.INFO);

LoggerConfigurator([
  ConsoleSink(LOG_LEVEL),
  ApplicationSink(LOG_LEVEL, 1000)
]);

const app_config = await RPC.get_configs();
const locale = app_config.locale ?? "en";
const resources = await RPC.load_locale(locale);

configStore.init(app_config);
i18nStore.init_locale(locale, resources);