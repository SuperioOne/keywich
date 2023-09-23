import type {CharsetItem, KeyManagerRpc, KeyMetadataItem} from "$lib/rpc/rpc";
import {invoke} from "@tauri-apps/api/tauri"

export const TauriRpc: KeyManagerRpc = {
  calculate_password(): Promise<string> {
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
  }
}