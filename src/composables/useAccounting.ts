import type { AccountingResult, ExpenseDetailInput, IncomeDetailInput, LaborTimeInput } from '@/types/accounting'

export function useAccounting() {
  const calculate = (
    incomeDetails: IncomeDetailInput[],
    expenses: ExpenseDetailInput[],
    labor: LaborTimeInput,
  ): AccountingResult => {
    const totalSales = incomeDetails.reduce((sum, i) => sum + i.amount, 0)
    const totalExpense = expenses.reduce((sum, e) => sum + e.amount, 0)
    const addedValue = totalSales - totalExpense
    const totalHours = labor.normal_hours + labor.overtime_hours + labor.public_hours
    const unitValue = totalHours > 0 ? addedValue / totalHours : 0
    const headcount = Math.max(labor.headcount, 1)
    const valueRate = totalSales > 0 ? (addedValue / totalSales) * 100 : 0
    const expenseRate = totalSales > 0 ? (totalExpense / totalSales) * 100 : 0
    return {
      total_sales: totalSales,
      total_expense: totalExpense,
      added_value: addedValue,
      total_hours: totalHours,
      unit_value: unitValue,
      sales_per_person: totalSales / headcount,
      value_per_person: addedValue / headcount,
      value_rate: valueRate,
      expense_rate: expenseRate,
    }
  }
  return { calculate }
}
