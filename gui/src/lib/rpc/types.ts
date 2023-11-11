export type  PasswordOutputType = "json" | "base64" | "text" | "qr";

export interface KeyOptions {
  target_size: number;
  revision: number;
  charset: string;
  domain: string;
  user_name: string;
  notes?: string;
  tags?: string[];
  custom_icon?: File;
}

export interface KeyMetadataItem {
  id: number;
  target_size: number;
  revision: number;
  charset: string;
  domain: string;
  user_name: string;
  notes?: string;
  pinned: boolean;
  createdAt: number;
  tags: string[];
}

export interface CharsetItem {
  id: number;
  charset: string;
  name: string;
  description?: string;
}

export type RPCSuccessResult<T> = {
  success: true;
  data: T;
}

export type ErrorType<T> = T extends string
  ? T
  : Partial<Record<keyof T, string[] | undefined>>

export type RPCErrorResult<T> = {
  success: false;
  errors: ErrorType<T>
}

export type RPCResult<T> = RPCSuccessResult<T> | RPCErrorResult<T>;