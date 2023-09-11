import type {PageLoad} from './$types';
import type {PassMetadataItem} from "$lib"
import {RPC} from "$lib";

export const load: PageLoad = async ({}) => {
  const passMetadataItems: PassMetadataItem[] = await RPC.instance.get_pinned_items();
  return {
    pinnedItems: passMetadataItems,
  };
};