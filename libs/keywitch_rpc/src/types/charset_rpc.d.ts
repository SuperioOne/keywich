import {CharsetItem, CharsetOptions} from "./contracts.js";
import {PropertyError, RPCResult, RPCVoidResult} from "./utility.js";

/**
 * Represents a charset RPC (Remote Procedure Call) service.
 */
export interface CharsetRPC {
  /**
   * Creates a new character set with the specified options.
   * @param charset - The options for the new character set.
   * @returns A promise resolving to the result of the RPC operation containing the added character set or an error message.
   */
  create_charset(charset: CharsetOptions): Promise<RPCResult<CharsetItem, string | PropertyError<CharsetOptions>>>;

  /**
   * Removes the charset with the specified ID.
   * @param id - The ID of the key to be removed.
   * @returns A promise resolving to the result of the RPC operation.
   */
  remove_charset(id: number): Promise<RPCVoidResult>;

  /**
   * Retrieves the available character sets.
   * @returns A promise resolving to the result of the RPC operation containing character set items or an error message.
   */
  get_charsets(): Promise<RPCResult<CharsetItem[]>>;
}