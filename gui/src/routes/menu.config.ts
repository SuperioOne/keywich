import HomeIcon from "$lib/icons/home.svelte"
import KeyIcon from "$lib/icons/key.svelte"
import SettingsIcon from "$lib/icons/settings.svelte"
import type {ComponentType} from "svelte";
import type {LayoutRouteId} from "./$types";

export interface MenuItem {
  label: string;
  target?: LayoutRouteId;
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