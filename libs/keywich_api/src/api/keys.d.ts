export interface KeyRequest {
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

  /** Optional custom icon data for the key. */
  custom_icon?: Uint8Array;

  /** Optional password generator version. */
  version?: string;
}

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

export interface SearchQuery {
  username?: string[];
  domain?: string[];
  tag?: string[];
}

export interface KeysRpcApi {
  delete_key(id: number): Promise<void>;

  get_key_by_id(id: number): Promise<KeyItem>;

  get_keys(): Promise<KeyItem[]>;

  get_keys(): Promise<KeyItem[]>;

  get_pinned_keys(): Promise<KeyItem[]>;

  insert_key(options: KeyRequest): Promise<number>;

  pin_key(id: number): Promise<void>;

  search_keys(query: SearchQuery): Promise<KeyItem[]>;

  unpin_key(id: number): Promise<void>;

  update_key(id: number, options: KeyRequest): Promise<void>;
}