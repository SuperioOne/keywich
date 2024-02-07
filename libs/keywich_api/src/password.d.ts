export type  PasswordOutputType = "Json" | "Base64" | "Text" | "Qr" | "Uri";

export interface PasswordRequest {
  profile_id: number;
  content: string;
  output_type: PasswordOutputType
}

export interface PasswordGenerateRequest {
  content: string,
  revision: number,
  domain: string,
  username: string,
  charset: string,
  version: string,
  target_len: number,
  output_type: PasswordOutputType,
}

export interface PasswordsRpcApi {
  generate_password_from(request: PasswordRequest): Promise<string>;

  generate_password(request: PasswordGenerateRequest): Promise<string>;
}