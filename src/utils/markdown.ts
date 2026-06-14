// Markdown 工具
export function stripMarkdown(md: string): string {
  return md.replace(/[#*`~\[\]()>|_-]/g, '').trim()
}

export function excerpt(md: string, maxLen = 100): string {
  const text = stripMarkdown(md)
  return text.length > maxLen ? text.slice(0, maxLen) + '...' : text
}
