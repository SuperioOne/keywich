export interface KeyManagerRpc {
  get_charsets(): Promise<CharsetItem[]>;

  get_pinned_items(): Promise<KeyMetadataItem[]>;

  get_key_collection(): Promise<KeyMetadataItem[]>;

  calculate_password(id: number, outputType?: PasswordOutputType): Promise<string>;

  add_key(options: KeyOptions): Promise<KeyMetadataItem>;

  remove_key(id: number): Promise<boolean>;

  save_file(fileData: Uint8Array, path?: string): Promise<void>;
}

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