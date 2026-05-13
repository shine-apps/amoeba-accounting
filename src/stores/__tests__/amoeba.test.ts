import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useAmoebaStore } from '../amoeba'
import type { Amoeba, AmoebaInput } from '@/types/amoeba'

const mockListAmoebas = vi.fn()
const mockCreateAmoeba = vi.fn()
const mockUpdateAmoeba = vi.fn()
const mockDeleteAmoeba = vi.fn()

vi.mock('@/composables/useTauri', () => ({
  useTauri: () => ({
    listAmoebas: mockListAmoebas,
    createAmoeba: mockCreateAmoeba,
    updateAmoeba: mockUpdateAmoeba,
    deleteAmoeba: mockDeleteAmoeba,
  }),
}))

function makeAmoeba(overrides: Partial<Amoeba> = {}): Amoeba {
  return {
    id: 1,
    name: '测试阿米巴',
    amoeba_type: '生产型',
    leader: '张三',
    parent_id: undefined,
    status: 'active',
    created_at: '2026-05-01T00:00:00',
    updated_at: '2026-05-01T00:00:00',
    ...overrides,
  }
}

describe('useAmoebaStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  describe('fetchList', () => {
    it('populates amoebas and sets loading', async () => {
      const items = [makeAmoeba({ id: 1 }), makeAmoeba({ id: 2, name: '二组' })]
      mockListAmoebas.mockResolvedValue(items)

      const store = useAmoebaStore()
      const promise = store.fetchList()
      expect(store.loading).toBe(true)
      await promise
      expect(store.amoebas).toEqual(items)
      expect(store.loading).toBe(false)
    })

    it('sets loading to false even on failure', async () => {
      mockListAmoebas.mockRejectedValue(new Error('fail'))

      const store = useAmoebaStore()
      await expect(store.fetchList()).rejects.toThrow('fail')
      expect(store.loading).toBe(false)
    })
  })

  describe('create', () => {
    it('calls createAmoeba and appends to list', async () => {
      const input: AmoebaInput = { name: '新组', amoeba_type: '营销型', leader: '李四' }
      const created = makeAmoeba({ id: 3, name: '新组', amoeba_type: '营销型', leader: '李四' })
      mockCreateAmoeba.mockResolvedValue(created)

      const store = useAmoebaStore()
      const result = await store.create(input)
      expect(result).toEqual(created)
      expect(store.amoebas).toContainEqual(created)
    })
  })

  describe('update', () => {
    it('updates item in list and currentAmoeba if matching', async () => {
      const store = useAmoebaStore()
      store.amoebas = [makeAmoeba({ id: 1, name: '旧名' }), makeAmoeba({ id: 2, name: '二组' })]
      store.currentAmoeba = makeAmoeba({ id: 1, name: '旧名' })

      const input: AmoebaInput = { name: '新名', amoeba_type: '生产型', leader: '张三' }
      const updated = makeAmoeba({ id: 1, name: '新名' })
      mockUpdateAmoeba.mockResolvedValue(updated)

      const result = await store.update(1, input)
      expect(result).toEqual(updated)
      expect(store.amoebas[0].name).toBe('新名')
      expect(store.currentAmoeba?.name).toBe('新名')
    })

    it('does not affect currentAmoeba if different id', async () => {
      const store = useAmoebaStore()
      store.amoebas = [makeAmoeba({ id: 1 })]
      store.currentAmoeba = makeAmoeba({ id: 2 })

      const input: AmoebaInput = { name: '改名', amoeba_type: '生产型', leader: '张三' }
      mockUpdateAmoeba.mockResolvedValue(makeAmoeba({ id: 1, name: '改名' }))

      await store.update(1, input)
      expect(store.currentAmoeba?.id).toBe(2)
    })
  })

  describe('remove', () => {
    it('removes from list and clears currentAmoeba if matching', async () => {
      const store = useAmoebaStore()
      store.amoebas = [makeAmoeba({ id: 1 }), makeAmoeba({ id: 2 })]
      store.currentAmoeba = makeAmoeba({ id: 1 })

      mockDeleteAmoeba.mockResolvedValue(undefined)

      await store.remove(1)
      expect(store.amoebas).toHaveLength(1)
      expect(store.amoebas[0].id).toBe(2)
      expect(store.currentAmoeba).toBeNull()
    })

    it('keeps currentAmoeba if different id', async () => {
      const store = useAmoebaStore()
      store.amoebas = [makeAmoeba({ id: 1 }), makeAmoeba({ id: 2 })]
      store.currentAmoeba = makeAmoeba({ id: 2 })

      mockDeleteAmoeba.mockResolvedValue(undefined)

      await store.remove(1)
      expect(store.currentAmoeba).not.toBeNull()
    })
  })
})
