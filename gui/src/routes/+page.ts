import type {PageLoad} from './$types';
import {Log, RPC} from "$lib";

export const load: PageLoad = async () => {
  const result = await RPC.KeyMetadata.get_pinned_items();

  if (result.success) {
    return {
      pinnedItems: result.data,
    };
  } else {
    // TODO: show error panel instead of empty pinned list
    Log.error(result.error);
    return {
      pinnedItems: []
    }
  }
};