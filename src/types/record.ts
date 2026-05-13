import type { ExpenseDetail, IncomeDetail, LaborTime, AccountingResult } from './accounting'

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
  income_details: IncomeDetail[]
  expenses: ExpenseDetail[]
  labor: LaborTime
  result?: AccountingResult
}

export interface RecordInput {
  amoeba_id: number
  period_type: string
  period_start: string
  period_end: string
  remark: string
  income_details: IncomeDetailInput[]
  expenses: ExpenseDetailInput[]
  labor: LaborTimeInput
}

export type { ExpenseDetailInput, IncomeDetailInput, LaborTimeInput }

