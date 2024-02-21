import type {PageLoad} from './$types';
import {Log, RPC} from "$lib";

export const load: PageLoad = async () => {
  try {
    const pinned_items = await RPC.get_pinned_keys();

    return {
      pinned_items: pinned_items
    };
  } catch (err) {
    Log.error(err);
  }
}