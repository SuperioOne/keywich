import { getModeOsPrefers, getModeUserPrefers } from "@skeletonlabs/skeleton";
import { writable } from "svelte/store";

function createThemeStore()
{
  const userTheme = getModeUserPrefers();
  const { subscribe, update } = writable(userTheme);

  return {
    subscribe,
    flip: () => update((value) => !value)
  };
}

export const themeStore = createThemeStore();