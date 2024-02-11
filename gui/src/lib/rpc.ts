import _rpc from "@keywich/tauri_api";
import type {KeywichRpcApi} from "@keywich/api";
import {Log} from "./logger";


const proxyed_api = {};
for (const rpcKey in _rpc) {

  const fn = new Proxy(Reflect.get(_rpc, rpcKey), {
    apply(target: any, thisArg: unknown, argArray: unknown[]): unknown {
      Log.debug(`fn ${rpcKey}: args (${JSON.stringify(argArray)})`);

      const start = performance.mark(`${rpcKey}_str`);
      const result = Reflect.apply(target, thisArg, argArray);
      const end = performance.mark(`${rpcKey}_end`);
      const measure = performance.measure(start.name, end.name);

      Log.debug(`fn ${rpcKey}: execution duration ${measure.duration}ms`);

      performance.clearMarks(start.name)
      performance.clearMarks(end.name);

      return result;
    }
  })

  Reflect.set(proxyed_api, rpcKey, fn);
}

export const RPC = proxyed_api as KeywichRpcApi;