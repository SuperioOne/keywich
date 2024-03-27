import type { PageLoad } from "./$types";
import { Api } from "$lib";

export const load: PageLoad = async () => {
  const configs = await Api.load_configs();

  return {
    is_db_created: configs.is_db_created,
  };
};
