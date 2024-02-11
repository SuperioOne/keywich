import {writable} from "svelte/store";

const {subscribe, set, update} = writable<boolean>(false);

export const logPanelStore = {
  subscribe,
  open: () => set(true),
  close: () => set(false),
  flip: () => update((value) => !value),
}