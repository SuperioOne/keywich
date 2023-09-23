import type {PageLoad} from './$types';
import {RPC} from "$lib";
import type {KeyMetadataItem} from "$lib"

export const load: PageLoad = async ({}) => {
  const keyMetadataItems: KeyMetadataItem[] = await RPC.instance.get_key_collection();
  return {
    keys: keyMetadataItems,
  };
};
