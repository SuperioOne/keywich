export interface UtilityRpcApi {
  save_file(fileData: Uint8Array, path?: string): Promise<boolean>;

  get_system_locale(): Promise<string>;

  load_locale(locale: string): Promise<Record<string, string>>;

  convert_content_src(content_name: string): Promise<string>;

  copy_to_clipboard(value: string): Promise<void>;
}