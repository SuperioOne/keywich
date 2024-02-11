import type {ErrorResponse} from "../api/types.js";

/**
 * Checks if given value is an instance of ErrorResponse
 * @param {unknown} error - value to check.
 * @returns {error is ErrorResponse}
 */
export function is_error_response(error: unknown): error is ErrorResponse;

/**
 * Checks if given value is undefined, null or empty array/string
 * @param {string | unknown[] | null | undefined} value - value to check.
 * @returns {value is undefined | null}
 */
export function is_null_or_empty(value: string | unknown[] | null | undefined): value is null | undefined;

/**
 * Function to return the provided value if it is not null, undefined, or NaN,
 * otherwise returns the default value.
 *
 * @template T - The type of the value and default value.
 * @param {T} value - The value to check.
 * @param {T} defaultValue - The default value to return if the provided value is null, undefined, or NaN.
 * @returns - The value if it is not null, undefined, or NaN, otherwise returns the default value.
 */
export function or_default<T>(value: T, defaultValue: NonNullable<T>): NonNullable<T>;

export type DebounceOptions<T> = {
  timeout: number;
  onSuccess?: (data: T) => void;
  onError?: (e: unknown) => void;
}

export interface Action_1<T1, TResult> {
  (arg1: T1): TResult;
}

export interface Action_2<T1, T2, TResult> {
  (arg1: T1, arg2: T2): TResult;
}

export interface Action_3<T1, T2, T3, TResult> {
  (arg1: T1, arg2: T2, arg3: T3): TResult;
}

export interface Action<TResult> {
  (): TResult;
}

export type DebouncerController<TAction> = {
  update: TAction;
}


export function create_debouncer<TResult = unknown>(action: Action<TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action<void>>;
export function create_debouncer<T1, TResult = unknown>(action: Action_1<T1, TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action_1<T1, void>>;
export function create_debouncer<T1, T2, TResult = unknown>(action: Action_2<T1, T2, TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action_2<T1, T2, void>>;
export function create_debouncer<T1, T2, T3, TResult = unknown>(action: Action_3<T1, T2, T3, TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action_3<T1, T2, T3, void>>;
export function create_debouncer<TResult = unknown>(action: (...args: unknown[]) => TResult | Promise<TResult>, options: DebounceOptions<TResult>): DebouncerController<unknown>;