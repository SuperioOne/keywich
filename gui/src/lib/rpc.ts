import _rpc from "@keywich/tauri_api";
import type {KeywichRpcApi} from "@keywich/api";
import {Log} from "./logger";
import {env} from "$env/dynamic/public"

let API = _rpc;
if (env.PUBLIC_KW_LOG_LEVEL === "DEBUG") {
  const proxyed_api = {};
  for (const rpcKey in _rpc) {

    const fn = new Proxy(Reflect.get(_rpc, rpcKey), {
      apply(target: any, thisArg: unknown, argArray: unknown[]): unknown {
        Log.debug(`fn ${rpcKey}: args (${JSON.stringify(argArray, (key, value) => {
          if (value instanceof Uint8Array) {
            return "[REDACTED] UInt8Array"
          } else {
            return value;
          }
        })})`);

        return Reflect.apply(target, thisArg, argArray);
      }
    })

    Reflect.set(proxyed_api, rpcKey, fn);
  }

  API = proxyed_api as KeywichRpcApi;
}

export const RPC = API;