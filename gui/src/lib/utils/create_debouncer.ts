export type DebounceOptions<T> = {
  timeout: number;
  onSuccess?: (data: T) => void;
  onError?: (e: unknown) => void;
}

interface Action_1<T1, TResult> {
  (arg1: T1): TResult;
}

interface Action_2<T1, T2, TResult> {
  (arg1: T1, arg2: T2): TResult;
}

interface Action_3<T1, T2, T3, TResult> {
  (arg1: T1, arg2: T2, arg3: T3): TResult;
}

interface Action<TResult> {
  (): TResult;
}

export type DebouncerController<TAction> = {
  update: TAction;
}

export function create_debouncer<TResult = any>(action: Action<TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action<void>>;
export function create_debouncer<T1, TResult = any>(action: Action_1<T1, TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action_1<T1, void>>;
export function create_debouncer<T1, T2, TResult = any>(action: Action_2<T1, T2, TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action_2<T1, T2, void>>;
export function create_debouncer<T1, T2, T3, TResult = any>(action: Action_3<T1, T2, T3, TResult | Promise<TResult>>, options: DebounceOptions<TResult>): DebouncerController<Action_3<T1, T2, T3, void>>;
export function create_debouncer<TResult = any>(action: (...args: any[]) => TResult | Promise<TResult>, options: DebounceOptions<TResult>): DebouncerController<any> {
  const {
    onSuccess,
    onError,
    timeout
  } = options
  let timerId: any;

  return {
    update: (...args: unknown[]) => {

      if (timerId !== undefined) {
        clearTimeout(timerId);
      }

      timerId = setTimeout(() => {
        try {
          const result = action(...args);

          if (result instanceof Promise) {
            const currentTimer = timerId;

            result
              .then(value => {
                // drop if timer is changed
                if (currentTimer === timerId && onSuccess) {
                  onSuccess(value);
                }
              })
              .catch((err) => {
                if (currentTimer === timerId && onError) {
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
