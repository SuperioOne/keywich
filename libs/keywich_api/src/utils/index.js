/**
 * @type {import("./types.js").is_error_response}
 */
export function is_error_response(error) {
  return typeof error === "object" &&
    error !== null &&
    Reflect.has(error, "message") &&
    Reflect.has(error, "code");
}

/**
 * @type {import("./types.js").is_null_or_empty}
 */
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

/**
 * @type {import("./types.js").or_default}
 */
export function or_default(value, defaultValue) {
  if (value === null || value === undefined || (typeof value === "number" && isNaN(value))) {
    return defaultValue;
  }

  return value;
}

/**
 * @param {(...args: unknown[]) => TResult | Promise<TResult>} action
 * @param {import("./types.js").DebounceOptions<TResult>} options
 * @returns {import("./types.js").DebouncerController<unknown>}
 * @template {unknown} TResult
 */
export function create_debouncer(action, options) {
  const {
    onSuccess,
    onError,
    timeout
  } = options

  /** @type {number} **/
  let timer_id;

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
              .then(value => {
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
    }
  };
}
