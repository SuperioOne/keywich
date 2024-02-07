/**
 * Represents the error structure for object property validation.
 * @template T - The type of detailed error information.
 */
export type PropertyError<T> = Partial<Record<keyof T, string[] | undefined>>

export type ErrorResponse = {
  message: string;
  code: string;
}