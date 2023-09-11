export interface KeyManagerRpc {
  get_charsets(): Promise<CharsetItem[]>;

  get_pinned_items(): Promise<PassMetadataItem[]>;

  get_password_collection(): Promise<PassMetadataItem[]>;

  get_password(): Promise<string>;
}

export interface PassMetadataItem {
  id: number;
  target_size: number;
  revision: number;
  charset: string;
  domain: string;
  user_name: string;
  notes?: string;
  pinned: boolean;
}

export interface CharsetItem {
  id: number;
  charset: string;
  description?: string;
}