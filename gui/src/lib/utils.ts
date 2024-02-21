export * from "./utils/key_filter_tokenizer";

import type {FieldError} from "@keywich/api";

/**
 * Represents the error structure for object property validation.
 * @template T - The type of detailed error information.
 */
export type ValidationError<T> = Partial<Record<keyof T, FieldError[] | undefined>>