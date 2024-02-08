import type {ErrorResponse, PropertyError} from "../api/types.js"

/**
 * Checks if given value is an instance of ErrorResponse
 * @param error - value to check.
 */
export function is_error_response(error: unknown): error is ErrorResponse;

/**
 * Checks if given value is undefined, null or empty array/string
 * @param value - value to check.
 */
export function is_null_or_empty(value: string | unknown[] | null | undefined): value is undefined | null;


/**
 * Function to return the provided value if it is not null, undefined, or NaN,
 * otherwise returns the default value.
 *
 * @template T - The type of the value and default value.
 * @param value - The value to check.
 * @param defaultValue - The default value to return if the provided value is null, undefined, or NaN.
 * @returns - The value if it is not null, undefined, or NaN, otherwise returns the default value.
 */
export function or_default<T>(value: T | undefined | null, defaultValue: T): T;