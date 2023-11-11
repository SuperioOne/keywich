import type {CharsetOptions, KeyOptions} from "../types/contracts.js";
import type {PropertyError, RPCErrorResult} from "../types/utility.js";

export function parse_KeyOptions(options: KeyOptions): RPCErrorResult<PropertyError<KeyOptions>> | undefined;

export function parse_CharsetOptions(options: CharsetOptions): RPCErrorResult<PropertyError<CharsetOptions>> | undefined;
