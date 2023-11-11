import type {PageLoad} from './$types';
import RPC from "@keywitch/memory_rpc";
import {Log} from "$lib";

export const load: PageLoad = async ({}) => {
  const result = await RPC.KeyMetadata.get_key_collection();
  
  if (result.success) {
    return {
      keys: result.data,
    };
  } else {
    Log.error(result.error);
    return {
      keys: []
    }
  }
};
