import type {PageLoad} from  "./$types";
import {RPC} from "$lib";

export const load: PageLoad = async () => {
  const pinned_items = await RPC.get_pinned_keys();

  return {
    pinned_items: pinned_items
  };
}