import type {RPCResult} from "@keywitch/rpc";

/**
 * Convert a Promise with RPCResult to a regular Promise.
 *
 * @param action - The Promise with RPCResult to convert.
 * @returns A Promise that resolves with the data if RPCResult is successful,
 * or rejects with the error if RPCResult is unsuccessful.
 */
export function to_promise<TResult, TError = string>(action: Promise<RPCResult<TResult, TError>>): Promise<TResult> {
  return new Promise((resolve, reject) => {
    action
      .then(result => {
        if (result.success) {
          resolve(result.data);
        } else {
          reject(result.error);
        }
      })
      .catch(reject);
  });
}