import type {PageLoad} from './$types';
import {RPC} from "$lib";

export const load: PageLoad = async ({url}) => {
  const section = url.searchParams.get("section");
  const charsets = await RPC.get_charsets();

  return {
    charsets: charsets,
    section: section ?? undefined
  };
};