import {type AppActions, init_actions} from "./app_actions";
import {Log} from "./logger";
import {getModeAutoPrefers, initializeStores, storePopup} from "@skeletonlabs/skeleton";
import {arrow, autoUpdate, computePosition, flip, offset, shift} from "@floating-ui/dom";
import {themeStore} from "./stores/theme_store";
import {RPC} from "./rpc";

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

  static init() {
    initializeStores();

    storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});
    themeStore.set({
      name: "crimson",
      isLight: getModeAutoPrefers()
    });

    this.#actions = init_actions(RPC);
  }
}