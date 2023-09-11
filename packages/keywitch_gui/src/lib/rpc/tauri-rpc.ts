import type {CharsetItem, KeyManagerRpc, PassMetadataItem} from "$lib/rpc/rpc";
import {invoke} from "@tauri-apps/api/tauri"

export const TauriRpc: KeyManagerRpc = {
  get_charsets(): Promise<CharsetItem[]> {
    return invoke<CharsetItem[]>("get_charsets");
  },
  get_password_collection(): Promise<PassMetadataItem[]> {
    return invoke<PassMetadataItem[]>("get_passwords");
  },
  get_pinned_items(): Promise<PassMetadataItem[]> {
    return invoke<PassMetadataItem[]>("get_pinned");
  },
}