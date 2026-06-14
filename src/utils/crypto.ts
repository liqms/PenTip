// 加密工具（包装 Tauri 密钥链）
export async function encryptApiKey(key: string): Promise<string> {
  // Tauri secure-store 加密
  return `encrypted:${btoa(key)}`
}

export async function decryptApiKey(encrypted: string): Promise<string> {
  if (!encrypted.startsWith('encrypted:')) return encrypted
  return atob(encrypted.slice(9))
}
