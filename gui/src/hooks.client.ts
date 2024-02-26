import {AppEventBus, Log, type LogLevelType} from "$lib";
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

RPC.load_configs().then(async (app_config) => {
  try {
    if (app_config.configs) {
      configStore.init(app_config.configs);

      if (app_config.configs?.locale && app_config.locale_keys) {
        i18nStore.init_locale({
          locale: app_config.configs.locale,
          locale_keys: app_config.locale_keys,
          available_locales: app_config.available_locales
        });
      }
    }
  } catch (err) {
    Log.error(err);
  }

  await AppEventBus.addListener("unlock_required", async () => {
    Log.debug("Event received. Redirecting to unlock page.");
    sessionStorage.removeItem("unlocked");
    await goto("unlock");
  });
});
