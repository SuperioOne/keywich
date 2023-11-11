import type {CharsetItem, KeyMetadataItem, KeyOptions} from "$lib/rpc/types";
import {invoke} from "@tauri-apps/api/tauri"
import type {KeyManagerRpc} from "$lib";

export const TauriRpc: KeyManagerRpc = {
  save_file(fileData: Uint8Array, path?: string): Promise<void> {
    return Promise.resolve(undefined);
  },
  generate_password(): Promise<string> {
    return Promise.resolve("");
  },
  get_charsets(): Promise<CharsetItem[]> {
    return invoke<CharsetItem[]>("get_charsets");
  },
  get_key_collection(): Promise<KeyMetadataItem[]> {
    return invoke<KeyMetadataItem[]>("get_passwords");
  },
  get_pinned_items(): Promise<KeyMetadataItem[]> {
    return invoke<KeyMetadataItem[]>("get_pinned");
  },
  add_key: function (request: KeyOptions): Promise<KeyMetadataItem> {
    throw new Error("Function not implemented.");
  },
  remove_key: function (id: number): Promise<boolean> {
    throw new Error("Function not implemented.");
  }
}