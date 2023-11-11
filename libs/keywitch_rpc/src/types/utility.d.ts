/**
 * Represents a successful RPC (Remote Procedure Call) result with data.
 * @template T - The type of data returned in the success case.
 */
export type RPCSuccessDataResult<T> = {
  success: true;
  data: T;
}

/**
 * Represents a successful RPC (Remote Procedure Call) result without data.
 */
export type RPCSuccessResult = {
  success: true;
}

/**
 * Represents the error structure for object property validation.
 * @template T - The type of detailed error information.
 */
export type PropertyError<T> = Partial<Record<keyof T, string[] | undefined>>

/**
 * Represents an RPC (Remote Procedure Call) result in case of failure with error information.
 * @template T - The type of detailed error information.
 */
export type RPCErrorResult<T> = {
  success: false;
  error: T
}

/**
 * Represents an RPC (Remote Procedure Call) result that can either be successful with data,
 * or unsuccessful with error information.
 * @template T - The type of data returned in the success case.
 * @template TError - The type of detailed error information.
 */
export type RPCResult<T, TError = string> = RPCSuccessDataResult<T> | RPCErrorResult<TError>;

/**
 * Represents an RPC (Remote Procedure Call) result that can either be successful without data,
 * or unsuccessful with error information.
 * @template TError - The type of detailed error information.
 */
export type RPCVoidResult<TError = string> = RPCSuccessResult | RPCErrorResult<TError>;