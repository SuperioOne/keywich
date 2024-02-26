export interface ConfigFile {
  is_light_theme?: boolean;
  color_theme?: string;
  locale?: string;
}

export interface AppConfig {
  configs?: ConfigFile;
  is_db_created: boolean;
  locale_keys?: Record<string, string>;
  available_locales: string[];
}

export interface ConfigRPCApi {
  update_config_json(configs: ConfigFile): Promise<void>;

  get_config_json(): Promise<ConfigFile>;

  load_configs(): Promise<AppConfig>;

  load_locale(locale: string): Promise<Record<string, string>>;
}