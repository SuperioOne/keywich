export function or_default<T>(value: T | undefined | null, defaultValue: T): T {
  if (value === null || value === undefined || (typeof value === "number" && isNaN(value))) {
    return defaultValue;
  }

  return value;
}