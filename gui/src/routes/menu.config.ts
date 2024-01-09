import type {ComponentType} from "svelte";
import type {LayoutRouteId} from "./$types";

export interface MenuItem {
  label: string;
  target?: LayoutRouteId;
  icon?: ComponentType;
}