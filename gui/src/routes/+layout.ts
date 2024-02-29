import type {LayoutLoad} from "./$types";

export const csr = true;
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({route, url}) => {
  return {
    route_id: route.id,
    path: url.pathname
  }
}