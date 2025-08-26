// SSH密钥类型定义
export interface SshKeyPair {
  id: string;
  name: string;
  key_type: SshKeyType;
  key_size: number;
  comment: string;
  public_key: string;
  private_key: string; // 加密存储
  fingerprint: string;
  created_at: string;
  last_used?: string;
}

export type SshKeyType = 'Rsa' | 'Ed25519' | 'Ecdsa';

export interface KeyGenerationParams {
  name: string;
  key_type: SshKeyType;
  key_size: number;
  comment: string;
  passphrase?: string; // 密钥密码（可选）
}

// 应用配置类型
export interface AppConfig {
  theme: 'light' | 'dark';
  auto_backup: boolean;
  backup_retention: number;
  default_key_type: SshKeyType;
  default_key_size: number;
  ssh_config_path: string;
}

// SSH配置类型
export interface SshHostConfig {
  host_pattern: string;
  hostname?: string;
  user?: string;
  port?: number;
  identity_file?: string;
  other_options: Record<string, string>;
}

export interface SshConfig {
  hosts: SshHostConfig[];
  global_settings: Record<string, string>;
}

// 认证相关类型
export interface AuthState {
  is_authenticated: boolean;
  is_initialized: boolean;
}

// 导入导出类型
export interface ExportData {
  version: string;
  keys: SshKeyPair[];
  config: AppConfig;
  exported_at: string;
}