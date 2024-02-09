import type {LayoutLoad} from "./$types";
import {RPC} from "$lib";

export const csr = true;
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({route}) => {
  const app_config = await RPC.get_configs();

  return {
    route_id: route.id,
    app_config: app_config
  }
}