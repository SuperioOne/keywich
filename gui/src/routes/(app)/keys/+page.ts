import type { PageLoad } from "./$types";
import { Api, is_null_or_empty } from "$lib";

export const load: PageLoad = async ({ url }) => {
  const search_query = url.searchParams.get("s");
  const keys = is_null_or_empty(search_query)
    ? await Api.get_keys()
    : await Api.search_keys(search_query);

  return {
    keys: keys,
    search_query: search_query,
  };
};
