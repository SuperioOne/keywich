import type {PageLoad} from './$types';
import RPC from "@keywitch/memory_rpc";
import {Log} from "$lib";

export const load: PageLoad = async ({url, params}) => {
  const search = url.searchParams.get("search");
  const result = await RPC.KeyMetadata.get_key_collection();

  if (result.success) {
    return {
      keys: result.data,
      search: search
    };
  } else {
    Log.error(result.error);
    return {
      keys: [],
      search: search
    }
  }
};
