/**
 * Function to return the provided value if it is not null, undefined, or NaN,
 * otherwise returns the default value.
 *
 * @template T - The type of the value and default value.
 * @param value - The value to check.
 * @param defaultValue - The default value to return if the provided value is null, undefined, or NaN.
 * @returns - The value if it is not null, undefined, or NaN, otherwise returns the default value.
 */
export function or_default<T>(value: T | undefined | null, defaultValue: T): T {
  if (value === null || value === undefined || (typeof value === "number" && isNaN(value))) {
    return defaultValue;
  }

  return value;
}