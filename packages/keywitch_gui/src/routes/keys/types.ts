import type {KeyOptions} from "$lib";

export const ModalAction =
  {
    submit: 0,
    closed: 1
  } as const

export type ModalActionType = typeof ModalAction[keyof typeof ModalAction];

export interface ModalFormSubmitResult {
  type: typeof ModalAction.submit,
  data: KeyOptions
}

export interface ModalCloseResult {
  type: typeof ModalAction.closed
}

export type ModalActionResult = ModalFormSubmitResult | ModalCloseResult;