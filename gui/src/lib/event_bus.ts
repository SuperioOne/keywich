import {
  emit as tauri_emit,
  listen as tauri_listen,
} from "@tauri-apps/api/event";
import type { Event, EventCallback } from "@tauri-apps/api/event";
import { Log } from "./logger";

class _AppEventBus {
  #listeners: Array<() => void>;

  constructor() {
    this.#listeners = [];
  }

  async addListener<T>(
    event_type: string,
    callback: EventCallback<T>,
  ): Promise<number> {
    const handler = (event: Event<T>) => callback(event);
    const unregister = await tauri_listen(event_type, handler);

    this.#listeners.push(unregister);

    return this.#listeners.length - 1;
  }

  removeListener(id: number) {
    const unregister = this.#listeners[id];

    if (unregister) {
      unregister();
      this.#listeners.splice(id, 1);
    }
  }

  emit<T>(event_name: string, payload: T) {
    tauri_emit(event_name, payload).catch((err) => {
      Log.error(err);
    });
  }

  removeAll() {
    for (const unregister of this.#listeners) {
      unregister();
    }

    this.#listeners = [];
  }
}

export const AppEventBus = new _AppEventBus();
