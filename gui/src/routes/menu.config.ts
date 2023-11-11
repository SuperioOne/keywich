import KeyIcon from "$lib/icons/key.svelte"
import SettingsIcon from "$lib/icons/settings.svelte"
import HomeIcon from "$lib/icons/home.svelte"
import type {ComponentType} from "svelte";

export interface MenuItem {
  label: string;
  target?: string;
  icon?: ComponentType;
}

const menuConfig: MenuItem[] = [
  {
    label: "Home",
    target: "/",
    icon: HomeIcon,
  },
  {
    label: "Keys",
    target: "/keys",
    icon: KeyIcon,
  },
  {
    label: "Settings",
    target: "/settings",
    icon: SettingsIcon,
  },
];

export default menuConfig;