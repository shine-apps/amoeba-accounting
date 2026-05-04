export interface ExpenseDetail {
  id?: number
  record_id?: number
  category: string
  amount: number
  description: string
}

export interface ExpenseDetailInput {
  category: string
  amount: number
  description: string
}

export interface LaborTime {
  id?: number
  record_id?: number
  normal_hours: number
  overtime_hours: number
  public_hours: number
  headcount: number
}

export interface LaborTimeInput {
  normal_hours: number
  overtime_hours: number
  public_hours: number
  headcount: number
}

export interface AccountingResult {
  total_sales: number
  total_expense: number
  added_value: number
  total_hours: number
  unit_value: number
  sales_per_person: number
  value_per_person: number
  value_rate: number
  expense_rate: number
}
