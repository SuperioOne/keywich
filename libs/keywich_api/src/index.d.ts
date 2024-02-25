import type {KeysRpcApi} from "./api/keys.js";
import type {UtilityRpcApi} from "./api/utilities.js";
import type {CharsetRpcApi} from "./api/charsets.js";
import type {PasswordsRpcApi} from "./api/password.js";
import type {ConfigRPCApi} from "./api/configs.js";
import type {AccountRpcApi} from "./api/account.js";

export type * from "./api/account.js";
export type * from "./api/charsets.js";
export type * from "./api/configs.js";
export type * from "./api/keys.js";
export type * from "./api/password.js";
export type * from "./api/types.js";
export type * from "./api/utilities.js";

export interface KeywichRpcApi extends KeysRpcApi,
  PasswordsRpcApi,
  UtilityRpcApi,
  CharsetRpcApi,
  ConfigRPCApi,
  AccountRpcApi {
}

export type AppEvent<T, EType> = {
  payload: T,
  name: EType
}

export interface AppEventBus<EType> {

  addListener<T = unknown>(type: EType, callback: (event: AppEvent<T, EType>) => void): Promise<number>;

  emit<T = unknown>(event_name: EType, payload?: T): void;

  removeListener(id: number): void;

  removeAll(): void;
}
