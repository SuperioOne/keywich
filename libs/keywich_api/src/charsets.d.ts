export interface CharsetItem {
  name: string;
  charset: string;
  description?: string;
}

export type CharsetOptions = CharsetItem;

export interface CharsetRpcApi {
  insert_charset(charset: CharsetOptions): Promise<string>;

  delete_charset(name: string): Promise<void>;

  get_charsets(): Promise<CharsetItem[]>;
}