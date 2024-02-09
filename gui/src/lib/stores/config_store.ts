import {
  modeCurrent,
  setModeUserPrefers,
  setModeCurrent,
} from '@skeletonlabs/skeleton';
import {writable} from "svelte/store";
import type {AppConfig} from "@keywich/api";
import {create_debouncer} from "@keywich/api/utils";
import {RPC} from "$lib/rpc";
import {Log} from "$lib/logger";

export const ThemeOptions = [
  "crimson",
  "skeleton",
  "hamlindigo",
  "wintry",
  "rocket",
  "vintage"
] as const;
export type ThemeOptionType = typeof ThemeOptions[number];

function init_config_store(defaults?: AppConfig) {
  const light_switch = modeCurrent;
  const {subscribe, set, update} = writable<AppConfig>(defaults ?? {
    color_theme: "crimson",
    is_light_theme: true,
    locale: "en"
  });

  const setMode = (value: boolean) => {
    light_switch.set(value);
    setModeUserPrefers(value);
    setModeCurrent(value);
  };

  const write_scheduler = create_debouncer(
    (config: AppConfig) => RPC.update_configs(config),
    {
      timeout: 1000,
      onError: Log.error,
      onSuccess: () => Log.debug("config.json updated")
    });

  return {
    subscribe,
    set_theme: (theme: ThemeOptionType) => {
      update(current => {
        current.color_theme = theme;
        document?.body?.setAttribute("data-theme", theme ?? "crimson");
        write_scheduler.update(current);

        return current;
      })
    },
    set_dark_mode: () => {
      setMode(false);
      update(current => {
        current.is_light_theme = false;
        write_scheduler.update(current);

        return current;
      });
    },
    set_light_mode: () => {
      setMode(true);
      update(current => {
        current.is_light_theme = true;
        write_scheduler.update(current);

        return current;
      });
    },
    flip_mode: () => {
      update(current => {
        current.is_light_theme = !current.is_light_theme
        setMode(current.is_light_theme);
        write_scheduler.update(current);

        return current;
      })
    },
    set: (appConfig: AppConfig) => {
      document?.body?.setAttribute("data-theme", appConfig.color_theme ?? "crimson");
      set(appConfig);
      write_scheduler.update(appConfig);
      setMode(appConfig.is_light_theme ?? false);
    },
  };
}

export const configStore = init_config_store();
