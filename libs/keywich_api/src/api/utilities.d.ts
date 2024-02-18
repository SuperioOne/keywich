export interface UtilityRpcApi {
  save_file(fileData: Uint8Array, path?: string): Promise<boolean>;

  load_locale(locale: string): Promise<Record<string, string>>;

  convert_icon_src(icon_name: string): string;

  copy_to_clipboard(value: string): Promise<void>;

  get_locales(): Promise<string[]>;

  upload_icon(data: Uint8Array): Promise<string>;
}