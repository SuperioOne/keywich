import {Log, type LogLevelType} from "$lib";
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
import {goto} from "$app/navigation";

const LOG_LEVEL: LogLevelType = or_default(try_parse_log_level(env.PUBLIC_KW_LOG_LEVEL), LogLevel.INFO);

LoggerConfigurator([
  ConsoleSink(LOG_LEVEL),
  ApplicationSink(LOG_LEVEL, 1000)
]);

RPC.get_configs().then(async (app_config) => {
  const locale = app_config.locale ?? "en";
  configStore.init(app_config);
  const resources = await RPC.load_locale(locale);
  i18nStore.init_locale(locale, resources);
});

if (sessionStorage.getItem("unlocked") !== "1") {
  goto("unlock").then(() => {
    Log.debug("Redirecting to unlock page.");
  })
}