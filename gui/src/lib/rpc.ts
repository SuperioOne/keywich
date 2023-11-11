import {TestRpc} from "$lib/rpc/test_rpc";
import {env} from "$env/dynamic/public";
import {TauriRpc} from "./rpc/tauri_rpc";
import type {KeyManagerRpc} from "./rpc/rpc";

export * from "./rpc/rpc";
export * from "./rpc/types";

export function get_rpc(): KeyManagerRpc {

  if (env.PUBLIC_RPC_MODE === "tauri")
    return TauriRpc;
  else
    // ignore web for now
    return TauriRpc
}

export const RPC = (() => {
  console.debug("main");
  return new TestRpc()
})();