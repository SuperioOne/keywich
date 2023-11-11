import type {PageLoad} from './$types';
import type {KeyMetadataItem} from "$lib"
import {RPC} from "$lib/rpc";
import charsetRPC from "$lib/rpc/test_rpc/charsets";

export const load: PageLoad = async ({}) => {
  charsetRPC.charsets();
  const passMetadataItems: KeyMetadataItem[] = await RPC.get_pinned_items();
  return {
    pinnedItems: passMetadataItems,
  };
};