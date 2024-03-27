import type { ConfigFile } from "./api";

export * from "./stores/filter_history";
export * from "./stores/config_store";
export * from "./stores/i18n_store";
export * from "./stores/extended_toast_store";
export * from "./stores/log_panel_store";

export interface AppSettings {
  config?: ConfigFile;
}
