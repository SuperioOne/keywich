import type {RPCResult} from "@keywitch/rpc";

export function to_promise<TResult>(action: Promise<RPCResult<TResult, any>>): Promise<TResult> {
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