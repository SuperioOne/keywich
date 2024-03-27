import type { PageLoad } from "./$types";
import { Api } from "$lib";

export const load: PageLoad = async ({ url }) => {
  const section = url.searchParams.get("section");
  const charsets = await Api.get_charsets();

  return {
    charsets: charsets,
    section: section ?? undefined,
  };
};

