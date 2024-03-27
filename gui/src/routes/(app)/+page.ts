import type { PageLoad } from "./$types";
import { Api } from "$lib";

export const load: PageLoad = async () => {
  const pinned_items = await Api.get_pinned_keys();

  return {
    pinned_items: pinned_items,
  };
};

