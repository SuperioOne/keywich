export type BaseErrorResponse = {
  message: string;
  code: string;
  details?: string;
};

export type FieldErrorTypes =
  | "charset"
  | "length"
  | "range"
  | "must_match"
  | "password_must_match";

export type FieldError = {
  code: FieldErrorTypes;
  message: string | null;
  params: Record<string, unknown>;
};

export type ValidationErrorResponse = BaseErrorResponse & {
  fields: Record<string, FieldError[]>;
};

/**
 * Represents the structure of an error response.
 */
export type ErrorResponse = BaseErrorResponse | ValidationErrorResponse;

export type KeyOptions = {
  target_size: number;
  revision: number;
  charset: string;
  domain: string;
  username: string;
  notes?: string;
  tags?: string[];
  version?: string;
  custom_icon?: string;
};

export type PasswordOutputType = "Json" | "Base64" | "Text" | "Qr";

export type PasswordRequest = {
  profile_id: number;
  output_type: PasswordOutputType;
};

export type PasswordGenerateRequest = {
  content: string;
  revision: number;
  domain: string;
  username: string;
  charset: string;
  version: string;
  target_len: number;
  output_type: PasswordOutputType;
};

export type CharsetItem = {
  name: string;
  charset: string;
  description?: string;
};

export type CharsetOptions = CharsetItem;

export type ConfigFile = {
  is_light_theme?: boolean;
  color_theme?: string;
  locale?: string;
};

export type AppConfig = {
  configs?: ConfigFile;
  is_db_created: boolean;
  locale_keys?: Record<string, string>;
  available_locales: string[];
  log_level: "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE";
};

type BaseKeyRequest = {
  /** The target length of the password. */
  target_size: number;

  /** The revision number. */
  revision?: number;

  /** Allowed character set to generate password. */
  charset: string;

  /** The domain associated with the key. */
  domain: string;

  /** The username for the specified domain. */
  username: string;

  /** Optional notes for the key. */
  notes?: string;

  /** Optional array of tags associated with the key. */
  tags?: string[];

  /** Optional password generator version. */
  version?: string;
};

export type KeyRequest = BaseKeyRequest & {
  /** Optional custom icon data for the key. */
  custom_icon?: string;
};

export type KeyUpdateRequest = BaseKeyRequest & {
  /** Optional custom icon data for the key. */
  custom_icon?: CustomIconType;
};

export type CustomIconType =
  | { type: "path"; path: string }
  | { type: "name"; name: string };

export type KeyItem = {
  id: number;
  target_size: number;
  revision: number;
  charset: string;
  domain: string;
  username: string;
  notes?: string;
  pinned: boolean;
  created_at: number;
  tags: string[];
  custom_icon?: string;
};

export type VerifyResponse = {
  is_valid: boolean;
  path: string;
};

export interface KeyApi {
  delete_key(id: number): Promise<void>;
  get_key_by_id(id: number): Promise<KeyItem>;
  get_keys(): Promise<KeyItem[]>;
  get_keys(): Promise<KeyItem[]>;
  get_pinned_keys(): Promise<KeyItem[]>;
  insert_key(request: KeyRequest): Promise<number>;
  pin_key(id: number): Promise<void>;
  search_keys(query: string): Promise<KeyItem[]>;
  unpin_key(id: number): Promise<void>;
  update_key(id: number, request: KeyUpdateRequest): Promise<void>;
}

export interface AccountApi {
  login(password: string): Promise<void>;
  logout(): Promise<void>;
}

export interface ConfigApi {
  update_config_json(configs: ConfigFile): Promise<void>;
  get_config_json(): Promise<ConfigFile>;
  load_configs(): Promise<AppConfig>;
  load_locale(locale: string): Promise<Record<string, string>>;
}

export interface CharsetApi {
  insert_charset(charset: CharsetOptions): Promise<string>;
  delete_charset(name: string): Promise<void>;
  get_charsets(): Promise<CharsetItem[]>;
}

export interface UtilityApi {
  save_file(fileData: Uint8Array, path?: string): Promise<boolean>;
  select_file(extensions?: string[]): Promise<string | undefined>;
  convert_icon_src(icon_name: string): string;
  convert_img_src(img_path: string): string;
  copy_to_clipboard(value: string): Promise<void>;
  upload_icon(path: string): Promise<string>;
  backup(path?: string): Promise<void>;
  restore(path?: string): Promise<void>;
  verify_backup(path?: string): Promise<VerifyResponse>;
}

export interface PasswordApi {
  generate_password_from(request: PasswordRequest): Promise<string>;
  generate_password(request: PasswordGenerateRequest): Promise<string>;
}

export interface KeywichApi
  extends KeyApi,
    PasswordApi,
    UtilityApi,
    CharsetApi,
    ConfigApi,
    AccountApi {}
