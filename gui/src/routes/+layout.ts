import { getVersion } from "@tauri-apps/api/app";
import type { LayoutLoad } from "./$types";

export const csr = true;
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({ route, url }) => {
  const app_version = await getVersion();

  return {
    version: app_version,
    route_id: route.id,
    path: url.pathname,
  };
};

