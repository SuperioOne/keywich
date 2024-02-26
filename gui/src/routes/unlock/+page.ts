import type {PageLoad} from "./$types";
import {RPC} from "$lib";

export const load: PageLoad = async () => {
  const configs = await RPC.load_configs();

  return {
    is_db_created: configs.is_db_created
  }
}
