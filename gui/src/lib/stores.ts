import {initializeStores as initialize_skeleton_stores} from "@skeletonlabs/skeleton";
import {storePopup} from "@skeletonlabs/skeleton";
import {arrow, autoUpdate, computePosition, flip, offset, shift} from "@floating-ui/dom";
import {configStore} from "$lib/stores/config_store";
import type {AppConfig} from "@keywich/api";

export * from "./stores/filter_history";
export * from "./stores/config_store";
export * from "./stores/i18n_store";
export * from "./stores/extended_toast_store";
export * from "./stores/log_panel_store";

export interface AppSettings {
  config?: AppConfig
}

export function initialize_stores(settings: AppSettings) {

}