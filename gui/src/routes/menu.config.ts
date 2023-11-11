import KeyIcon from "$lib/icons/key.svelte"
import SettingsIcon from "$lib/icons/settings.svelte"
import ActivityIcon from "$lib/icons/activity.svelte"
import type {ComponentType} from "svelte";

export interface MenuItem {
  label: string;
  target?: string;
  icon?: ComponentType;
}

const menuConfig: MenuItem[] = [
  {
    label: "Dashboard",
    target: "/",
    icon: ActivityIcon,
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