import type {KeysRpcApi} from "./keys.js";
import type {UtilityRpcApi} from "./utilities.js";
import type {CharsetRpcApi} from "./charsets.js";
import type {PasswordsRpcApi} from "./password.js";

export type * from "./keys.js";
export type * from "./utilities.js";
export type * from "./charsets.js";
export type * from "./password.js";
export type * from "./types.js";

export interface KeywichRpcApi extends KeysRpcApi, PasswordsRpcApi, UtilityRpcApi, CharsetRpcApi {
}

