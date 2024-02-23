export interface AccountRpcApi {

  login(password: string): Promise<void>;

  logout(): Promise<void>;
}