import {RPCVoidResult} from "./utility.js";

/**
 * Represents a utility RPC (Remote Procedure Call) service.
 */
export interface UtilityRPC {
  /**
   * Saves a file with the provided data at the specified path.
   * When 'path' argument is undefined, implementers should save the file to a default path or,
   * if the platform supports it, prompt the user for a path.
   * @param fileData - The data of the file to be saved.
   * @param path - Optional. The path where the file should be saved. If undefined, implementers should handle path
   * selection.
   * @returns A promise resolving to the result of the RPC operation.
   */
  save_file(fileData: Uint8Array, path?: string): Promise<RPCVoidResult>;
}