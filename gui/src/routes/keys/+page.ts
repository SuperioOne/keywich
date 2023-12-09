import type {PageLoad} from './$types';
import RPC from "@keywitch/memory_rpc";
import {Log} from "$lib";
import type {KeyFilter} from "@keywitch/rpc";

export const load: PageLoad = async ({url, params, data}) => {
  const filter = get_filter(url);
  const result = await RPC.KeyMetadata.get_key_collection(filter);

  if (result.success) {
    return {
      keys: result.data,
    };
  } else {
    Log.error(result.error);
    return {
      keys: [],
    }
  }
};

function get_filter(url: URL): KeyFilter | undefined {
  const tag = url.searchParams.getAll("t");
  const username = url.searchParams.getAll("u");
  const domain = url.searchParams.getAll("d");
  const terms = url.searchParams.getAll("s");

  if (tag.length > 0 || username.length > 0 || domain.length > 0 || terms.length > 0) {
    return {
      username,
      tag,
      domain,
      searchTokens: terms
    }
  } else {
    return undefined;
  }
}