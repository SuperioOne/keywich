export interface BaseErrorResponse {
  message: string;
  code: string;
  details?: string;
}

export interface ValidationErrorResponse extends BaseErrorResponse {
  fields: Record<string, FieldError[]>;
}

export type FieldErrorTypes = "charset" | "length" | "range" | "must_match" | "password_must_match";

export type FieldError = {
  code: FieldErrorTypes;
  message: string | null;
  params: Record<string, unknown>;
}

/**
 * Represents the structure of an error response.
 */
export type ErrorResponse = BaseErrorResponse | ValidationErrorResponse;