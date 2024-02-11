import {Log} from "./logger";

export const EventTypes = ["locale_change", "notification"] as const;
export type EventType = typeof EventTypes[number];

export class AppEvent<T> extends CustomEvent<T> {
  constructor(type: EventType, data: T) {
    super(type, {
      detail: data
    });
  }
}

class AppEventBus {
  #base: EventTarget;
  constructor() {
    this.#base = new EventTarget();
  }

  addEventListener(type: EventType,
                   callback: (event: AppEvent<any>) => void,
                   options?: AddEventListenerOptions | boolean): void {
    this.#base.addEventListener(type, callback as EventListener, options);
  }

  dispatchEvent<T>(event: AppEvent<T>): boolean {
    Log.debug(`Global BUS: ${event.type}, ${JSON.stringify(event.detail, null, 2)}`)
    return this.#base.dispatchEvent(event);
  }

  removeEventListener(type: EventType,
                      callback: (event: AppEvent<any>) => void,
                      options?: EventListenerOptions | boolean) {
    this.#base.removeEventListener(type, callback as EventListener, options);
  }
}

export const GlobalEventBus = new AppEventBus();