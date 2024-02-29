import type {PageLoad} from "./$types";
import {RPC} from "$lib";
import {is_null_or_empty} from "@keywich/api/utils";

export const load: PageLoad = async ({url}) => {
  const search_query = url.searchParams.get("s");
  const keys = is_null_or_empty(search_query)
    ? await RPC.get_keys()
    : await RPC.search_keys(search_query);

  return {
    keys: keys,
    search_query: search_query
  };
};
