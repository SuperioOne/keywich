import type {KeyMetadataItem} from "$lib";

export const ModalAction = {
  submitted: 0,
  closed: 1
} as const

export type ModalActionType = typeof ModalAction[keyof typeof ModalAction];

export interface ModalFormSubmitResult {
  type: typeof ModalAction.submitted,
  data: KeyMetadataItem
}

export interface ModalCloseResult {
  type: typeof ModalAction.closed
}

export type ModalActionResult = ModalFormSubmitResult | ModalCloseResult;