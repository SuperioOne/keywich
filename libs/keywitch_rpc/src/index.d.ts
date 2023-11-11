import {CharsetRPC} from "./types/charset_rpc.js";
import {KeyMetadataRPC} from "./types/keymetadata_rpc.js";
import {UtilityRPC} from "./types/utility_rpc.js";

export * from "./types/contracts.js";
export * from "./types/keymetadata_rpc.js";
export * from "./types/utility_rpc.js";
export * from "./types/charset_rpc.js";
export * from "./types/utility.js";

export type RPCDefinition = {
  Charset: CharsetRPC,
  KeyMetadata: KeyMetadataRPC,
  Utility: UtilityRPC
}