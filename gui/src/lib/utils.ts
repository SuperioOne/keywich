export * from "./utils/key_filter_tokenizer";

import type {
  ErrorResponse,
  FieldError,
  ValidationErrorResponse,
} from "./api/types";

/**
 * Represents the error structure for object property validation.
 * @template T - The type of detailed error information.
 */
export type ValidationError<T> = Partial<
  Record<keyof T, FieldError[] | undefined>
>;

export function is_error_response(error: unknown): error is ErrorResponse {
  return (
    typeof error === "object" &&
    error !== null &&
    Reflect.has(error, "message") &&
    Reflect.has(error, "code")
  );
}

export function is_validation_error_response(
  error: unknown,
): error is ValidationErrorResponse {
  return is_error_response(error) && Reflect.has(error, "fields");
}

export function is_null_or_empty(
  value: string | unknown[] | null | undefined,
): value is null | undefined {
  switch (typeof value) {
    case "undefined":
      return true;
    case "object":
      return value === null || value?.length < 1;
    case "string":
      return value.trim().length < 1;
  }
}

export function or_default<T>(
  value: T,
  defaultValue: NonNullable<T>,
): NonNullable<T> {
  if (
    value === null ||
    value === undefined ||
    (typeof value === "number" && isNaN(value))
  ) {
    return defaultValue;
  }

  return value;
}

export type DebounceOptions<T> = {
  timeout: number;
  onSuccess?: (data: T) => void;
  onError?: (e: unknown) => void;
};

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
};

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
  options: DebounceOptions<TResult>,
): DebouncerController<Action<void>>;

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
  options: DebounceOptions<TResult>,
): DebouncerController<Action_1<T1, void>>;

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
  options: DebounceOptions<TResult>,
): DebouncerController<Action_2<T1, T2, void>>;

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
  options: DebounceOptions<TResult>,
): DebouncerController<Action_3<T1, T2, T3, void>>;

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
  options: DebounceOptions<TResult>,
): DebouncerController<(...args: unknown[]) => void> {
  const { onSuccess, onError, timeout } = options;

  let timer_id: number;

  return {
    update: (...args) => {
      if (timer_id !== undefined) {
        clearTimeout(timer_id);
      }

      timer_id = setTimeout(() => {
        try {
          const result = action(...args);

          if (result instanceof Promise) {
            const curr_timer = timer_id;

            result
              .then((value) => {
                // drop if timer is changed
                if (curr_timer === timer_id && onSuccess) {
                  onSuccess(value);
                }
              })
              .catch((err) => {
                if (curr_timer === timer_id && onError) {
                  onError(err);
                }
              });
          } else if (onSuccess) {
            onSuccess(result);
          }
        } catch (err) {
          if (onError) {
            onError(err);
          }
        }
      }, timeout);
    },
  };
}

