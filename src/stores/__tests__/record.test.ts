import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useRecordStore } from '../record'
import type { AccountingRecord, RecordInput } from '@/types/record'

const mockListRecords = vi.fn()
const mockGetRecord = vi.fn()
const mockSaveRecord = vi.fn()
const mockDeleteRecord = vi.fn()

vi.mock('@/composables/useTauri', () => ({
  useTauri: () => ({
    listRecords: mockListRecords,
    getRecord: mockGetRecord,
    saveRecord: mockSaveRecord,
    deleteRecord: mockDeleteRecord,
  }),
}))

function makeRecord(overrides: Partial<AccountingRecord> = {}): AccountingRecord {
  return {
    id: 1,
    amoeba_id: 1,
    period_type: 'month',
    period_start: '2026-05-01',
    period_end: '2026-05-31',
    external_sales: 800000,
    internal_sales: 200000,
    remark: '',
    created_at: '2026-05-01T00:00:00',
    updated_at: '2026-05-01T00:00:00',
    expenses: [],
    labor: {
      normal_hours: 160,
      overtime_hours: 20,
      public_hours: 0,
      headcount: 5,
    },
    result: {
      total_sales: 1000000,
      total_expense: 400000,
      added_value: 600000,
      total_hours: 180,
      unit_value: 3333.33,
      sales_per_person: 200000,
      value_per_person: 120000,
      value_rate: 60,
      expense_rate: 40,
    },
    ...overrides,
  }
}

describe('useRecordStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('fetchByAmoeba', () => {
    it('populates records and sets loading', async () => {
      const items = [makeRecord({ id: 1 }), makeRecord({ id: 2 })]
      mockListRecords.mockResolvedValue(items)

      const store = useRecordStore()
      const promise = store.fetchByAmoeba(1)
      expect(store.loading).toBe(true)
      await promise
      expect(store.records).toEqual(items)
      expect(store.loading).toBe(false)
    })

    it('sets loading to false even on failure', async () => {
      mockListRecords.mockRejectedValue(new Error('fail'))

      const store = useRecordStore()
      await expect(store.fetchByAmoeba(1)).rejects.toThrow('fail')
      expect(store.loading).toBe(false)
    })
  })

  describe('fetchById', () => {
    it('sets currentRecord on success', async () => {
      const record = makeRecord({ id: 42 })
      mockGetRecord.mockResolvedValue(record)

      const store = useRecordStore()
      const result = await store.fetchById(42)
      expect(result).toEqual(record)
      expect(store.currentRecord).toEqual(record)
    })
  })

  describe('save', () => {
    it('prepends new record when id not in list', async () => {
      const store = useRecordStore()
      store.records = [makeRecord({ id: 1, remark: 'existing' })]

      const input: RecordInput = {
        amoeba_id: 1,
        period_type: 'month',
        period_start: '2026-06-01',
        period_end: '2026-06-30',
        external_sales: 500000,
        internal_sales: 0,
        remark: 'new',
        expenses: [],
        labor: { normal_hours: 160, overtime_hours: 0, public_hours: 0, headcount: 3 },
      }

      const saved = makeRecord({ id: 2, remark: 'new' })
      mockSaveRecord.mockResolvedValue(saved)

      const result = await store.save(input)
      expect(result).toEqual(saved)
      expect(store.records).toHaveLength(2)
      expect(store.records[0].id).toBe(2) // prepended
      expect(store.currentRecord).toEqual(saved)
    })

    it('replaces record in list when id exists', async () => {
      const store = useRecordStore()
      store.records = [makeRecord({ id: 1, remark: 'old' })]

      const input: RecordInput = {
        amoeba_id: 1,
        period_type: 'month',
        period_start: '2026-05-01',
        period_end: '2026-05-31',
        external_sales: 900000,
        internal_sales: 0,
        remark: 'updated',
        expenses: [],
        labor: { normal_hours: 160, overtime_hours: 0, public_hours: 0, headcount: 3 },
      }

      const saved = makeRecord({ id: 1, remark: 'updated' })
      mockSaveRecord.mockResolvedValue(saved)

      const result = await store.save(input)
      expect(result).toEqual(saved)
      expect(store.records).toHaveLength(1)
      expect(store.records[0].remark).toBe('updated')
    })
  })

  describe('remove', () => {
    it('removes from list and clears currentRecord if matching', async () => {
      const store = useRecordStore()
      store.records = [makeRecord({ id: 1 }), makeRecord({ id: 2 })]
      store.currentRecord = makeRecord({ id: 1 })

      mockDeleteRecord.mockResolvedValue(undefined)

      await store.remove(1)
      expect(store.records).toHaveLength(1)
      expect(store.records[0].id).toBe(2)
      expect(store.currentRecord).toBeNull()
    })

    it('keeps currentRecord if different id', async () => {
      const store = useRecordStore()
      store.records = [makeRecord({ id: 1 }), makeRecord({ id: 2 })]
      store.currentRecord = makeRecord({ id: 2 })

      mockDeleteRecord.mockResolvedValue(undefined)

      await store.remove(1)
      expect(store.currentRecord).not.toBeNull()
    })
  })
})
