export interface KeyManagerRpc {
  get_charsets(): Promise<CharsetItem[]>;

  get_pinned_items(): Promise<KeyMetadataItem[]>;

  get_key_collection(): Promise<KeyMetadataItem[]>;

  calculate_password(): Promise<string>;

  add_key(request: NewKeyRequest): Promise<KeyMetadataItem>

  remove_key(id: number): Promise<boolean>;
}

export interface NewKeyRequest {
  target_size: number;
  revision: number;
  charset: string;
  domain: string;
  user_name: string;
  notes?: string;
  tags?: string[];
  custom_icon?: File
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
}

export interface CharsetItem {
  id: number;
  charset: string;
  description?: string;
}