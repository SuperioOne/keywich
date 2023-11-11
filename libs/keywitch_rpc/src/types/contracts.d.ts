export type  PasswordOutputType = "json" | "base64" | "text" | "qr";

export interface KeyOptions {
  /**
   * The target length of the password.
   */
  target_size: number;

  /**
   * The revision number.
   */
  revision: number;

  /**
   * Allowed character set to generate password.
   */
  charset: string;

  /**
   * The domain associated with the key.
   */
  domain: string;

  /**
   * The username for the specified domain.
   */
  user_name: string;

  /**
   * Optional notes for the key.
   */
  notes?: string;

  /**
   * Optional array of tags associated with the key.
   */
  tags?: string[];

  /**
   * Optional custom icon for the key.
   */
  custom_icon?: File;
}

export interface CharsetOptions {
  charset: string;
  name: string;
  description?: string;
}

export interface KeyMetadataItem {
  id: number;
  target_size: number;
  revision: number;
  charset: string;
  domain: string;
  user_name: string;
  notes?: string;
  pinned: boolean;
  createdAt: number;
  tags: string[];
  custom_icon?: string;
}

export interface CharsetItem {
  id: number;
  charset: string;
  name: string;
  description?: string;
}
