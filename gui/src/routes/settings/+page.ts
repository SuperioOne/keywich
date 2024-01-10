import type {PageLoad} from './$types';
import {Log, RPC} from "$lib";
import type {CharsetItem} from "@keywitch/rpc";

export const load: PageLoad = async ({url}) => {
  const section = url.searchParams.get("section");
  const charsets = await RPC.Charset.get_charsets();

  if (charsets.success) {
    return {
      charsets: charsets.data,
      section: section ?? undefined
    };
  } else {
    Log.error(charsets.error);
    return {
      charsets: [] as CharsetItem[],
      section: section ?? undefined
    }
  }
};