export const ModalAction = {
  submitted: 0,
  closed: 1
} as const

export type ModalActionType = typeof ModalAction[keyof typeof ModalAction];

export interface ModalFormSubmitResult<T> {
  type: typeof ModalAction.submitted,
  data: T
}

export interface ModalCloseResult {
  type: typeof ModalAction.closed
}

export type ModalActionResult<T> = ModalFormSubmitResult<T> | ModalCloseResult;