import {getToastStore} from "@skeletonlabs/skeleton";

export const getExtendedToastStore = () => {
  const {subscribe, close, trigger, freeze, unfreeze, clear} = getToastStore();

  return {
    subscribe,
    close,
    trigger,
    freeze,
    unfreeze,
    clear,
    trigger_success: (message: string) => trigger({
      message: message,
      background: "variant-filled-success",
      timeout: 1500
    }),
    trigger_error: (message: string) => trigger({
      message: message,
      background: "variant-filled-error",
      timeout: 3000
    }),
    trigger_warning: (message: string) => trigger({
      message: message,
      background: "variant-filled-warning",
      timeout: 2500
    }),
    trigger_info: (message: string) => trigger({
      message: message,
      background: "variant-filled-secondary",
      timeout: 1500
    }),
  }
}; 