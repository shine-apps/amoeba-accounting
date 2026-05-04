export const EXPENSE_CATEGORIES = [
  { code: 'material', name: '原材料费' },
  { code: 'outsourcing', name: '外部加工费' },
  { code: 'electricity', name: '电费' },
  { code: 'depreciation', name: '折旧费' },
  { code: 'transport', name: '运输费' },
  { code: 'maintenance', name: '维修费' },
  { code: 'office', name: '办公费' },
  { code: 'communication', name: '通讯费' },
  { code: 'travel', name: '差旅费' },
  { code: 'other', name: '其他费用' },
] as const

export const AMOEBA_TYPES = ['生产型', '营销型', '研发型', '管理型'] as const
export const PERIOD_TYPES = [
  { value: 'month', label: '按月' },
  { value: 'week', label: '按周' },
  { value: 'day', label: '按日' },
] as const
