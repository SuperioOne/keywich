import type {CharsetOptions, KeyOptions} from "@keywich/api/src/contracts.js";
import type {PropertyError, RPCErrorResult} from "@keywich/api/src/utility.js";

export function parse_KeyOptions(options: KeyOptions): RPCErrorResult<PropertyError<KeyOptions>> | undefined;

export function parse_CharsetOptions(options: CharsetOptions): RPCErrorResult<PropertyError<CharsetOptions>> | undefined;
