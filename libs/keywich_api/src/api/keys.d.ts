interface BaseKeyRequest {
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
}

export interface KeyRequest extends BaseKeyRequest {
  /** Optional custom icon data for the key. */
  custom_icon?: Uint8Array;
}

export interface KeyUpdateRequest extends BaseKeyRequest {
  /** Optional custom icon data for the key. */
  custom_icon?: CustomIconType;
}

export type CustomIconType = { type: "buffer", data: Uint8Array } | { type: "name", name: string };

export interface KeyItem {
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
}

export interface KeysRpcApi {
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