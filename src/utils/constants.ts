export const EXPENSE_CATEGORIES = [
  { id: 1, name: '原材料费' },
  { id: 2, name: '外部加工费' },
  { id: 3, name: '电费' },
  { id: 4, name: '折旧费' },
  { id: 5, name: '运输费' },
  { id: 6, name: '维修费' },
  { id: 7, name: '办公费' },
  { id: 8, name: '通讯费' },
  { id: 9, name: '差旅费' },
  { id: 10, name: '其他费用' },
] as const

export const INCOME_CATEGORIES = [
  { id: 1, name: '对外销售' },
  { id: 2, name: '内部交易' },
  { id: 3, name: '服务收入' },
  { id: 4, name: '其他收入' },
] as const

export const AMOEBA_TYPES = ['生产型', '营销型', '研发型', '管理型'] as const
export const PERIOD_TYPES = [
  { value: 'month', label: '按月' },
  { value: 'week', label: '按周' },
  { value: 'day', label: '按日' },
] as const
