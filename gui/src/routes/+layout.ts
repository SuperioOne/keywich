import type {LayoutLoad} from "./$types";

export const csr = true;
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = ({route}) => {
  return {
    routeId: route.id
  }
}