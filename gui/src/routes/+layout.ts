import type {LayoutLoad} from "./$types";

export const csr = true;
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({route}) => {
  return {
    route_id: route.id,
  }
}