export interface AppConfig {
  is_dark_theme?: boolean;
  colorTheme?: string;
  locale?: string;
}

export interface ConfigRPCApi {
  update_configs(configs: AppConfig): Promise<void>;

  get_configs(): Promise<AppConfig>;
}