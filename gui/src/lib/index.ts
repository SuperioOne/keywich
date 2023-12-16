import type {EventDispatcher} from "$lib/event_manager";
import type {RPCApi} from "@keywitch/rpc";
import {getContext, setContext} from "svelte";

export * from "./components";
export * from "./logger";
export * from "./utils";
export * from "./event_manager";
export * from "./stores";


export type KeywitchAppContext = {
  AppEvents: EventDispatcher,
  RPC: RPCApi
}
const CONTEXT_KEY = "keywitch_app_context";

export const get_app_context = () => getContext<KeywitchAppContext>(CONTEXT_KEY);
export const set_app_context = (context: KeywitchAppContext) => setContext<KeywitchAppContext>(CONTEXT_KEY, context);