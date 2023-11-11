import type {CharsetItem, KeyMetadataItem, KeyOptions, PasswordOutputType, RPCResult} from "$lib/rpc/types";

export interface KeyManagerRpc {
  get_charsets(): Promise<CharsetItem[]>;

  get_pinned_items(): Promise<KeyMetadataItem[]>;

  get_key_collection(): Promise<KeyMetadataItem[]>;

  generate_password(id: number, outputType?: PasswordOutputType): Promise<string>;

  add_key(options: KeyOptions): Promise<RPCResult<KeyMetadataItem>>;

  update_key(options: KeyOptions): Promise<RPCResult<KeyMetadataItem>>;

  remove_key(id: number): Promise<boolean>;

  save_file(fileData: Uint8Array, path?: string): Promise<void>;
}

export interface CharsetsRPC {
  get_charsets(): Promise<CharsetItem[]>;

  add_charset(): Promise<boolean>;

}

export interface KeysRPC {

}
