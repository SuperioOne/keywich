import {getToastStore} from "@skeletonlabs/skeleton";

export const getExtendedToastStore = () => {
  let {subscribe, close, trigger, freeze, unfreeze, clear} = getToastStore();

  return {
    subscribe,
    close,
    trigger,
    freeze,
    unfreeze,
    clear,
    trigger_success: (message: string) => trigger({
      message: message,
      background: "variant-soft-success",
      timeout: 1500
    }),    
    trigger_error: (message: string) => trigger({
      message: message,
      background: "variant-soft-error",
      timeout: 3000
    }),    
    trigger_warning: (message: string) => trigger({
      message: message,
      background: "variant-soft-warning",
      timeout: 2500
    }),   
    trigger_info: (message: string) => trigger({
      message: message,
      background: "variant-soft-secondary",
      timeout: 1500
    }),
  }
}; 