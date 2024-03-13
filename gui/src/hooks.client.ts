import {
  ApplicationSink, ConsoleSink, LoggerConfigurator, try_parse_log_level,
  LogLevel, configStore, i18nStore, RPC, AppEventBus, Log
} from "$lib";
import {goto} from "$app/navigation";

RPC.load_configs().then(async (app_config) => {
  try {
    if (app_config.configs) {
      const LOG_LEVEL = try_parse_log_level(app_config.log_level) ?? LogLevel.INFO;
      LoggerConfigurator.setup([
        ConsoleSink(LOG_LEVEL),
        ApplicationSink(LOG_LEVEL, 1000)
      ]);

      configStore.init(app_config.configs);

      if (app_config.configs?.locale && app_config.locale_keys) {
        i18nStore.init_locale({
          locale: app_config.configs.locale,
          locale_keys: app_config.locale_keys,
          available_locales: app_config.available_locales
        });
      }
    } else {
      console.error("Application config is empty.");
    }
  } catch (err) {
    console.error(err);
  }

  await AppEventBus.addListener("unlock_required", async () => {
    Log.debug("Event received. Redirecting to unlock page.");
    sessionStorage.removeItem("unlocked");
    await goto("unlock");
  });
});
