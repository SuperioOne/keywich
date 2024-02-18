import type {ErrorResponse} from "../api/types.js";

/**
 * Checks if the given value is an instance of ErrorResponse.
 * @param {unknown} error - The value to check.
 * @returns {error is ErrorResponse} True if the value is an instance of ErrorResponse, false otherwise.
 */
export function is_error_response(error: unknown): error is ErrorResponse;

/**
 * Checks if the given value is undefined, null, or an empty array/string.
 * @param {string | unknown[] | null | undefined} value - The value to check.
 * @returns {value is undefined | null} True if the value is undefined or null, false otherwise.
 */
export function is_null_or_empty(value: string | unknown[] | null | undefined): value is null | undefined;

/**
 * Returns the provided value if it is not null, undefined, or NaN,
 * otherwise returns the default value.
 * @template T - The type of the value and default value.
 * @param {T} value - The value to check.
 * @param {T} defaultValue - The default value to return if the provided value is null, undefined, or NaN.
 * @returns {NonNullable<T>} The value if it is not null, undefined, or NaN, otherwise returns the default value.
 */
export function or_default<T>(value: T, defaultValue: NonNullable<T>): NonNullable<T>;

export type DebounceOptions<T> = {
  timeout: number;
  onSuccess?: (data: T) => void;
  onError?: (e: unknown) => void;
}

/**
 * Represents a function with one argument and a result.
 * @template T1 - The type of the first argument.
 * @template TResult - The type of the result.
 */
export interface Action_1<T1, TResult> {
  (arg1: T1): TResult;
}

/**
 * Represents a function with two arguments and a result.
 * @template T1 - The type of the first argument.
 * @template T2 - The type of the second argument.
 * @template TResult - The type of the result.
 */
export interface Action_2<T1, T2, TResult> {
  (arg1: T1, arg2: T2): TResult;
}

/**
 * Represents a function with three arguments and a result.
 * @template T1 - The type of the first argument.
 * @template T2 - The type of the second argument.
 * @template T3 - The type of the third argument.
 * @template TResult - The type of the result.
 */
export interface Action_3<T1, T2, T3, TResult> {
  (arg1: T1, arg2: T2, arg3: T3): TResult;
}

/**
 * Represents a function with no arguments and a result.
 * @template TResult - The type of the result.
 */
export interface Action<TResult> {
  (): TResult;
}

/**
 * A controller object for a debounced function.
 * @template TAction - The type of the action function.
 */
export type DebouncerController<TAction> = {
  update: TAction;
}

/**
 * @overload
 * Creates a debounced function with the given action and options. The returned function will delay calling the action
 * by the specified timeout in milliseconds. If onSuccess or onError are provided, they will be called when the action
 * completes successfully or with an error respectively.
 * @param action - A function to debounce. This function should take no arguments and can return a promise.
 * @param options - Options for configuring the behavior of the returned debounced function.
 * @returns A controller object that can be called repeatedly without calling the provided action until the specified
 * timeout has elapsed.
 **/
export function create_debouncer<TResult = unknown>(
  action: Action<TResult | Promise<TResult>>,
  options: DebounceOptions<TResult>): DebouncerController<Action<void>>;

/**
 * @overload
 * Creates a debounced function with the given action and options. The returned function will delay calling the action
 * by the specified timeout in milliseconds. If onSuccess or onError are provided, they will be called when the action
 * completes successfully or with an error respectively. The first argument of the action function will be typed as T1.
 * @param action - A function to debounce that takes a single argument of type T1.
 * @param options - Options for configuring the behavior of the returned debounced function.
 * @returns A function that can be called repeatedly without calling the provided action until the specified timeout has
 * elapsed. The first argument of the returned function will be of type T1.
 */
export function create_debouncer<T1, TResult = unknown>(
  action: Action_1<T1, TResult | Promise<TResult>>,
  options: DebounceOptions<TResult>): DebouncerController<Action_1<T1, void>>;

/**
 * @overload
 * Creates a debounced function with the given action and options. The returned function will delay calling the action
 * by the specified timeout in milliseconds. If onSuccess or onError are provided, they will be called when the action
 * completes successfully or with an error respectively. The first argument of the action function will be typed as T1
 * and the second argument will be typed as T2.
 * @param action - A function to debounce that takes two arguments of types T1 and T2 and returns a promise.
 * @param options - Options for configuring the behavior of the returned debounced function.
 * @returns A controller object that can be called repeatedly without calling the provided action until the specified
 * timeout has elapsed. The first and second arguments of the returned function will be of types T1 and T2 respectively.
 **/
export function create_debouncer<T1, T2, TResult = unknown>(
  action: Action_2<T1, T2, TResult | Promise<TResult>>,
  options: DebounceOptions<TResult>): DebouncerController<Action_2<T1, T2, void>>;

/**
 * @overload
 * Creates a debounced function with the given action and options. The returned function will delay calling the action
 * by the specified timeout in milliseconds. If onSuccess or onError are provided, they will be called when the action
 * completes successfully or with an error respectively. The first argument of the action function will be typed as T1,
 * the second argument will be typed as T2 and the third argument will be typed as T3.
 * @param action - A function to debounce that takes three arguments of types T1, T2, and T3.
 * @param options - Options for configuring the behavior of the returned debounced function.
 * @returns A controller object that can be called repeatedly without calling the provided action * until the specified
 * timeout has elapsed. The first, second and third arguments of the returned function will be of types T1, T2, and T3
 * respectively.
 **/
export function create_debouncer<T1, T2, T3, TResult = unknown>(
  action: Action_3<T1, T2, T3, TResult | Promise<TResult>>,
  options: DebounceOptions<TResult>): DebouncerController<Action_3<T1, T2, T3, void>>;

/**
 * @overload
 * Creates a debounced function with the given action and options. The returned function will delay calling the action
 * by the specified timeout in milliseconds. If onSuccess or onError are provided, they will be called when the action
 * completes successfully or with an error respectively. The action function can take any number of arguments and can
 * return a promise.
 * @param action - A function to debounce that takes any number of arguments.
 * @param options - Options for configuring the behavior of the returned debounced function.
 * @returns A controller object that can be called repeatedly without calling the provided action until the specified
 * timeout has elapsed. The action function can take any number of arguments and should return a promise.
 **/
export function create_debouncer<TResult = unknown>(
  action: (...args: unknown[]) => TResult | Promise<TResult>,
  options: DebounceOptions<TResult>): DebouncerController<(...args:unknown[]) => void>;
