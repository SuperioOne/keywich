export interface AppConfig {
  is_light_theme?: boolean;
  color_theme?: string;
  locale?: string;
}

export interface ConfigRPCApi {
  update_configs(configs: AppConfig): Promise<void>;

  get_configs(): Promise<AppConfig>;
}