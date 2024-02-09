import type {KeysRpcApi} from "./api/keys.js";
import type {UtilityRpcApi} from "./api/utilities.js";
import type {CharsetRpcApi} from "./api/charsets.js";
import type {PasswordsRpcApi} from "./api/password.js";
import type {ConfigRPCApi} from "./api/configs.js";

export type * from "./api/keys.js";
export type * from "./api/utilities.js";
export type * from "./api/configs.js";
export type * from "./api/charsets.js";
export type * from "./api/password.js";
export type * from "./api/types.js";

export interface KeywichRpcApi extends KeysRpcApi,
  PasswordsRpcApi,
  UtilityRpcApi,
  CharsetRpcApi,
  ConfigRPCApi {
}

