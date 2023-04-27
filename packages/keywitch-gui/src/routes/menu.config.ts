import { KeyIcon, UserIcon, SettingsIcon } from "$lib/icons";
import type { ComponentType } from "svelte";

export interface MenuItem
{
  label: string;
  target?: string;
  icon?: ComponentType;
}

const menuConfig: MenuItem[] = [
  {
    label: "Keys",
    target: "/",
    icon: KeyIcon,
  },
  {
    label: "Profiles",
    target: "/profiles",
    icon: UserIcon,
  },
  {
    label: "Settings",
    target: "/settings",
    icon: SettingsIcon,
  },
];

export default menuConfig;