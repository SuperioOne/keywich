import type {PageLoad} from './$types';
import {RPC} from "$lib";
import type {PassMetadataItem} from "$lib"

export const load: PageLoad = async ({}) => {
  const passMetadataItems: PassMetadataItem[] = await RPC.instance.get_password_collection();
  return {
    passwords: passMetadataItems,
  };
};