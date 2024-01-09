import {
  modeCurrent,
  setModeUserPrefers,
  setModeCurrent,
} from '@skeletonlabs/skeleton';
import {writable} from "svelte/store";

export const ThemeOptions = [
  "crimson",
  "skeleton",
  "hamlindigo",
  "wintry",
  "rocket",
  "vintage"
] as const;
export type ThemeOptionType = typeof ThemeOptions[number];
export type ThemeDetails = {
  name: ThemeOptionType;
  isLight: boolean;
}

function init_theme_store(defaults?: ThemeDetails) {
  const lightSwitch = modeCurrent;
  const {subscribe, set, update} = writable<ThemeDetails>(defaults ?? {
    name: "crimson",
    isLight: false
  });

  const setMode = (value: boolean) => {
    lightSwitch.set(value);
    setModeUserPrefers(value);
    setModeCurrent(value);
  };

  return {
    subscribe,
    set_theme: (theme: ThemeOptionType) => {
      update(current => {
        current.name = theme;
        document?.body?.setAttribute("data-theme", theme ?? "crimson");
        return current;
      })
    },
    set_dark_mode: () => {
      setMode(false);
      update(current => {
        current.isLight = false;
        return current;
      });
    },
    set_light_mode: () => {
      setMode(true);
      update(current => {
        current.isLight = true;
        return current;
      });
    },
    flip_mode: () => {
      update(current => {
        current.isLight = !current.isLight
        setMode(current.isLight);
        return current;
      })
    },
    set: (theme: ThemeDetails) => {
      document?.body?.setAttribute("data-theme", theme.name ?? "crimson");
      set(theme);
      setMode(theme.isLight);
    },
  };
}

export const themeStore = init_theme_store();
