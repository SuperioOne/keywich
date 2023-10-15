import type {PageLoad} from './$types';
import type {KeyMetadataItem} from "$lib"
import {RPC} from "$lib";

export const load: PageLoad = async ({}) => {
  const passMetadataItems: KeyMetadataItem[] = await RPC.get_pinned_items();
  return {
    pinnedItems: passMetadataItems,
  };
};