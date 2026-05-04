import type { ExpenseDetail, LaborTime, AccountingResult } from './accounting'

export interface AccountingRecord {
  id?: number
  amoeba_id: number
  period_type: string
  period_start: string
  period_end: string
  external_sales: number
  internal_sales: number
  remark: string
  created_at: string
  updated_at: string
  expenses: ExpenseDetail[]
  labor: LaborTime
  result?: AccountingResult
}

export interface RecordInput {
  amoeba_id: number
  period_type: string
  period_start: string
  period_end: string
  external_sales: number
  internal_sales: number
  remark: string
  expenses: ExpenseDetailInput[]
  labor: LaborTimeInput
}

export interface ExpenseDetailInput {
  category: string
  amount: number
  description: string
}

export interface LaborTimeInput {
  normal_hours: number
  overtime_hours: number
  public_hours: number
  headcount: number
}
