export interface UtilityRpcApi {
  save_file(fileData: Uint8Array, path?: string): Promise<boolean>;

  convert_icon_src(icon_name: string): string;

  copy_to_clipboard(value: string): Promise<void>;

  upload_icon(data: Uint8Array): Promise<string>;
}