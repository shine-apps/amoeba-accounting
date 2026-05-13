import { describe, it, expect } from 'vitest'
import { formatMoney, formatPercent, formatDate, getPeriodLabel } from '../format'

describe('formatMoney', () => {
  it('formats positive numbers with thousands separator and 2 decimal places', () => {
    const result = formatMoney(1234567.89)
    expect(result).toContain('1,234,567.89')
  })

  it('formats zero', () => {
    const result = formatMoney(0)
    expect(result).toContain('0.00')
  })

  it('formats negative numbers', () => {
    const result = formatMoney(-5000.5)
    expect(result).toContain('5,000.50')
  })

  it('rounds to 2 decimal places', () => {
    const result = formatMoney(100.456)
    expect(result).toContain('100.46')
  })
})

describe('formatPercent', () => {
  it('formats positive percentage with 1 decimal place', () => {
    expect(formatPercent(52.16)).toBe('52.2%')
  })

  it('formats zero', () => {
    expect(formatPercent(0)).toBe('0.0%')
  })

  it('formats negative percentage', () => {
    expect(formatPercent(-10.54)).toBe('-10.5%')
  })

  it('handles whole numbers', () => {
    expect(formatPercent(100)).toBe('100.0%')
  })
})

describe('formatDate', () => {
  it('returns the date string as-is', () => {
    expect(formatDate('2026-05-13')).toBe('2026-05-13')
  })

  it('returns empty string for empty input', () => {
    expect(formatDate('')).toBe('')
  })
})

describe('getPeriodLabel', () => {
  it('returns Chinese label for month', () => {
    expect(getPeriodLabel('month')).toBe('月')
  })

  it('returns Chinese label for week', () => {
    expect(getPeriodLabel('week')).toBe('周')
  })

  it('returns Chinese label for day', () => {
    expect(getPeriodLabel('day')).toBe('日')
  })

  it('returns the input for unknown types', () => {
    expect(getPeriodLabel('year')).toBe('year')
  })
})
