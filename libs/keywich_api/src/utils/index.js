/** @type {import("./types.js").is_error_response} */
export function is_error_response(error) {
  return typeof error === "object" &&
    error !== null &&
    Reflect.has(error, "message") &&
    Reflect.has(error, "code");
}

/** @type {import("./types.js").is_null_or_empty} */
export function is_null_or_empty(value) {
  switch (typeof value) {
    case "undefined":
      return true;
    case "object":
      return value === null || value?.length < 1;
    case "string":
      return value.trim().length < 1;
  }
}

/** @type {import("./types.js").or_default} */
export function or_default(value, defaultValue) {
  if (value === null || value === undefined || (typeof value === "number" && isNaN(value))) {
    return defaultValue;
  }

  return value;
}