// 日期格式化工具
export function formatDate(timestamp: number, locale = 'zh-CN'): string {
  return new Date(timestamp).toLocaleDateString(locale)
}

export function formatDateTime(timestamp: number, locale = 'zh-CN'): string {
  return new Date(timestamp).toLocaleString(locale)
}
