import {is_null_or_empty} from "@keywich/api/utils";
import {RPC} from "$lib";

export async function resolve_content_url(name: string): Promise<string> {
  if (is_null_or_empty(name)) {
    throw new Error("Empty content name");
  }

  const session_item = sessionStorage.getItem(`content:/${name}`);

  if (is_null_or_empty(session_item)) {
    const item = await RPC.convert_content_src(name);
    sessionStorage.setItem(`content:/${name}`, item);

    return item
  } else {
    return session_item;
  }
}