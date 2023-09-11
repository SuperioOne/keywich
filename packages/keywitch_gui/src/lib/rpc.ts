export * from "./rpc/rpc";

import {env} from "$env/dynamic/public";
import {TauriRpc} from "./rpc/tauri-rpc";
import type {KeyManagerRpc} from "./rpc/rpc";

export function get_rpc(): KeyManagerRpc {
  if (env.PUBLIC_RPC_MODE === "tauri")
    return TauriRpc;
  else
    // ignore web for now
    return TauriRpc
}

export class RPC {
  public static instance = get_rpc();
}