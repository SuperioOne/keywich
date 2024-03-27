import {
  modeCurrent,
  setModeUserPrefers,
  setModeCurrent,
} from "@skeletonlabs/skeleton";
import { writable } from "svelte/store";
import type { ConfigFile } from "../api";
import { create_debouncer } from "../utils";
import { Api } from "../api";
import { Log } from "../logger";

const CONFIG_DEFAULTS: ConfigFile = {
  color_theme: "crimson",
  is_light_theme: false,
  locale: "en",
};

export const ThemeOptions = [
  "crimson",
  "default",
  "gold-nouveau",
  "hamlindigo",
  "modern",
  "rocket",
  "sahara",
  "seafoam",
  "skeleton",
  "vintage",
  "wintry",
] as const;

export type ThemeOptionType = (typeof ThemeOptions)[number];

const light_switch = modeCurrent;
const { set, update, subscribe } = writable<ConfigFile>(CONFIG_DEFAULTS);
const write_scheduler = create_debouncer(
  (config: ConfigFile) => Api.update_config_json(config),
  {
    timeout: 750,
    onError: Log.error,
    onSuccess: () => Log.debug("config.json updated"),
  },
);

function setMode(value: boolean) {
  light_switch.set(value);
  setModeUserPrefers(value);
  setModeCurrent(value);
}

function set_locale(locale: string) {
  update((current) => {
    current.locale = locale;
    write_scheduler.update(current);

    return current;
  });
}

function set_theme(theme: ThemeOptionType) {
  update((current) => {
    current.color_theme = theme;
    document?.body?.setAttribute("data-theme", theme ?? "crimson");
    write_scheduler.update(current);

    return current;
  });
}

function set_dark_mode() {
  setMode(false);
  update((current) => {
    current.is_light_theme = false;
    write_scheduler.update(current);

    return current;
  });
}

function set_light_mode() {
  setMode(true);
  update((current) => {
    current.is_light_theme = true;
    write_scheduler.update(current);

    return current;
  });
}

function flip_mode() {
  update((current) => {
    current.is_light_theme = !current.is_light_theme;
    setMode(current.is_light_theme);
    write_scheduler.update(current);

    return current;
  });
}

function set_config(appConfig: ConfigFile) {
  document?.body?.setAttribute(
    "data-theme",
    appConfig.color_theme ?? "crimson",
  );
  set(appConfig);
  write_scheduler.update(appConfig);
  setMode(appConfig.is_light_theme ?? false);
}

/**
 * Overrides store contents but does not try to persist contents.
 * @param appConfig
 */
function init(appConfig: ConfigFile) {
  document?.body?.setAttribute(
    "data-theme",
    appConfig.color_theme ?? "crimson",
  );
  set(appConfig);
  setMode(appConfig.is_light_theme ?? false);
}

export const configStore = {
  subscribe,
  set: set_config,
  update,
  flip_mode,
  init,
  set_light_mode,
  set_dark_mode,
  set_theme,
  set_locale,
};
