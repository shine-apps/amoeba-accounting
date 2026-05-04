export function formatMoney(value: number): string {
  return value.toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
}

export function formatPercent(value: number): string {
  return value.toFixed(1) + '%'
}

export function formatDate(dateStr: string): string {
  if (!dateStr) return ''
  return dateStr
}

export function getPeriodLabel(periodType: string): string {
  const map: Record<string, string> = { month: '月', week: '周', day: '日' }
  return map[periodType] || periodType
}
