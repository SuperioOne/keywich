import {emit as tauri_emit, listen as tauri_listen} from '@tauri-apps/api/event';

/**
 * @template EType
 * @typedef {import("@keywich/api").AppEventBus<EType>} EventBus
 */

/**
 * @template T
 * @template EType
 * @typedef {import("@keywich/api").AppEvent<T, EType>} AppEvent
 */

/**
 * @typedef {"unlock_required"} EventType
 */

/**
 * @implements {EventBus<EventType>}
 */
class TauriAppEventBus {
  /** @type {Array<() => void>} */
  #listeners;

  constructor() {
    this.#listeners = [];
  }

  /**
   * @template T
   * @param {EventType} event_type
   * @param {(e:AppEvent<T,EventType>) => void | Promise<void>} callback
   * @returns {Promise<number>}
   */
  async addListener(event_type, callback) {
    /** @type {(event: AppEvent<T, EventType>) => void} */
    const handler = (event) => {
      const action = callback(event);

      if (action instanceof Promise) {
        action.catch(err => {
          console.error(err);
        })
      }
    }

    const unregister = await tauri_listen(event_type, handler);
    this.#listeners.push(unregister);

    return this.#listeners.length - 1;
  }

  /**
   * @param {number} id
   * @returns {void}
   */
  removeListener(id) {
    const unregister = this.#listeners[id];

    if (unregister) {
      unregister();
      this.#listeners.splice(id, 1);
    }
  }

  /**
   * @template T
   * @param {EventType} event_name
   * @param {T} payload
   * @returns {void}
   */
  emit(event_name, payload) {
    tauri_emit(event_name, payload)
      .catch(err => {
        console.error(err)
      });
  }

  /**
   * @returns {void}
   */
  removeAll() {
    for (const unregister of this.#listeners) {
      unregister();
    }

    this.#listeners = [];
  }
}

const event_bus = new TauriAppEventBus();

export default event_bus;