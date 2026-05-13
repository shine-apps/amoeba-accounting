import { describe, it, expect } from 'vitest'
import { useAccounting } from '../useAccounting'
import type { ExpenseDetailInput, LaborTimeInput } from '@/types/accounting'

const { calculate } = useAccounting()

describe('useAccounting.calculate', () => {
  it('computes all 9 formulas correctly for standard data', () => {
    const expenses: ExpenseDetailInput[] = [
      { category: 'material', amount: 400000, description: '原材料' },
      { category: 'electricity', amount: 50000, description: '电费' },
      { category: 'depreciation', amount: 30000, description: '折旧' },
    ]
    const labor: LaborTimeInput = {
      normal_hours: 800,
      overtime_hours: 100,
      public_hours: 100,
      headcount: 10,
    }

    const result = calculate(expenses, labor, 800000, 200000)

    expect(result.total_sales).toBeCloseTo(1_000_000, 2)
    expect(result.total_expense).toBeCloseTo(480_000, 2)
    expect(result.added_value).toBeCloseTo(520_000, 2)
    expect(result.total_hours).toBeCloseTo(1_000, 2)
    expect(result.unit_value).toBeCloseTo(520, 2)
    expect(result.sales_per_person).toBeCloseTo(100_000, 2)
    expect(result.value_per_person).toBeCloseTo(52_000, 2)
    expect(result.value_rate).toBeCloseTo(52, 2)
    expect(result.expense_rate).toBeCloseTo(48, 2)
  })

  it('handles zero sales — negative added value, zero rates', () => {
    const expenses: ExpenseDetailInput[] = [
      { category: 'material', amount: 1000, description: '' },
    ]
    const labor: LaborTimeInput = {
      normal_hours: 100,
      overtime_hours: 0,
      public_hours: 0,
      headcount: 5,
    }

    const result = calculate(expenses, labor, 0, 0)

    expect(result.total_sales).toBeCloseTo(0, 2)
    expect(result.total_expense).toBeCloseTo(1000, 2)
    expect(result.added_value).toBeCloseTo(-1000, 2)
    expect(result.unit_value).toBeCloseTo(-10, 2)
    expect(result.value_rate).toBeCloseTo(0, 2)
    expect(result.expense_rate).toBeCloseTo(0, 2)
  })

  it('returns zero unit_value when total hours is zero', () => {
    const expenses: ExpenseDetailInput[] = []
    const labor: LaborTimeInput = {
      normal_hours: 0,
      overtime_hours: 0,
      public_hours: 0,
      headcount: 1,
    }

    const result = calculate(expenses, labor, 10000, 0)

    expect(result.total_hours).toBeCloseTo(0, 2)
    expect(result.unit_value).toBeCloseTo(0, 2)
  })

  it('handles empty expense list', () => {
    const labor: LaborTimeInput = {
      normal_hours: 160,
      overtime_hours: 0,
      public_hours: 0,
      headcount: 2,
    }

    const result = calculate([], labor, 50000, 0)

    expect(result.total_expense).toBeCloseTo(0, 2)
    expect(result.added_value).toBeCloseTo(50_000, 2)
    expect(result.unit_value).toBeCloseTo(312.5, 2)
  })

  it('handles loss scenario (expenses > sales)', () => {
    const expenses: ExpenseDetailInput[] = [
      { category: 'material', amount: 900000, description: '' },
    ]
    const labor: LaborTimeInput = {
      normal_hours: 500,
      overtime_hours: 0,
      public_hours: 0,
      headcount: 5,
    }

    const result = calculate(expenses, labor, 500000, 0)

    expect(result.added_value).toBeLessThan(0)
    expect(result.added_value).toBeCloseTo(-400_000, 2)
    expect(result.unit_value).toBeCloseTo(-800, 2)
  })
})
