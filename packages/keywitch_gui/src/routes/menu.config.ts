import { KeyIcon, UserIcon, SettingsIcon, ActivityIcon} from "$lib/icons";
import type { ComponentType } from "svelte";

export interface MenuItem
{
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