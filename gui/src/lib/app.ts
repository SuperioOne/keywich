import {type AppActions, init_actions} from "./app_actions";
import {Log} from "./logger";
import {initializeStores, storePopup} from "@skeletonlabs/skeleton";
import {arrow, autoUpdate, computePosition, flip, offset, shift} from "@floating-ui/dom";
import {configStore} from "./stores/config_store";
import {RPC} from "./rpc";
import type {AppConfig} from "@keywich/api";

export class App {
  static #actions: AppActions | undefined;

  static get Actions(): AppActions {
    if (this.#actions) {
      return this.#actions;
    } else {
      Log.error("App not initialized.");
      throw Error("App not initialized.");
    }
  }

  static init(conf: AppConfig) {
    initializeStores();

    storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});
    configStore.set(conf);

    this.#actions = init_actions(RPC);
  }
}