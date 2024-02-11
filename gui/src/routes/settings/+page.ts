import type {PageLoad} from './$types';
import {RPC} from "$lib";

export const load: PageLoad = async ({url}) => {
  const section = url.searchParams.get("section");
  const charsets = await RPC.get_charsets();
  const locales = await RPC.get_locales();

  return {
    charsets: charsets,
    section: section ?? undefined,
    locales: locales
  };
};