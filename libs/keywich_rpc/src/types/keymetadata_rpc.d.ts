import {KeyMetadataItem, KeyOptions, PasswordOutputType} from "./contracts.js";
import {PropertyError, RPCResult, RPCVoidResult} from "./utility.js";

/**
 * Represents a key filter object that can be used to filter keys.
 * @interface
 */
export interface KeyFilter {
  username?: string[];
  domain?: string[];
  tag?: string[];
  searchTokens?: string[];
}

/**
 * Represents a key RPC (Remote Procedure Call) service.
 */
export interface KeyMetadataRPC {
  /**
   * Creates a new key with the provided options.
   * @param options - The options for the new key.
   * @returns A promise resolving to the result of the RPC operation containing key metadata or an error message.
   */
  create_key(options: KeyOptions): Promise<RPCResult<KeyMetadataItem, string | PropertyError<KeyOptions>>>;

  /**
   * Generates a password for the specified key ID and output type.
   * @param id - The ID of the key for which the password should be generated.
   * @param outputType - Optional. The type of output for the generated password.
   * @returns A promise resolving to the result of the RPC operation containing the generated password or an error message.
   */
  generate_password(id: number, outputType?: PasswordOutputType): Promise<RPCResult<string>>;

  /**
   * Retrieves the collection of keys.
   * @returns A promise resolving to the result of the RPC operation containing key metadata items or an error message.
   */
  get_key_collection(filter?: KeyFilter): Promise<RPCResult<KeyMetadataItem[]>>;

  /**
   * Retrieves the collection of pinned keys.
   * @returns A promise resolving to the result of the RPC operation containing pinned key metadata items or an error message.
   */
  get_pinned_items(): Promise<RPCResult<KeyMetadataItem[]>>;

  /**
   * Pins the key with the specified ID.
   * @param id - The ID of the key to be pinned.
   * @returns A promise resolving to the result of the RPC operation.
   */
  pin_key(id: number): Promise<RPCVoidResult>;

  /**
   * Removes the key with the specified ID.
   * @param id - The ID of the key to be removed.
   * @returns A promise resolving to the result of the RPC operation.
   */
  remove_key(id: number): Promise<RPCVoidResult>;

  /**
   * Unpins the key with the specified ID.
   * @param id - The ID of the key to be unpinned.
   * @returns A promise resolving to the result of the RPC operation.
   */
  unpin_key(id: number): Promise<RPCVoidResult>;

  /**
   * Updates the key with the provided options.
   * @param id - The key id.
   * @param options - The options for updating the key.
   * @returns A promise resolving to the result of the RPC operation containing updated key metadata or an error message.
   */
  update_key(id: number, options: KeyOptions): Promise<RPCResult<KeyMetadataItem, string | PropertyError<KeyOptions>>>;
}