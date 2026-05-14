import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { CategoryList, SaveCategoriesInput } from '@/types/category'
import { useTauri } from '@/composables/useTauri'

export const useCategoryStore = defineStore('category', () => {
  const categoriesByAmoeba = ref<Map<number, CategoryList>>(new Map())
  const loading = ref(false)

  const { getCategories, saveCategories, resetCategories } = useTauri()

  async function fetchByAmoeba(amoebaId: number): Promise<CategoryList> {
    const cached = categoriesByAmoeba.value.get(amoebaId)
    if (cached) return cached

    loading.value = true
    try {
      const result = await getCategories(amoebaId)
      categoriesByAmoeba.value.set(amoebaId, result)
      return result
    } finally {
      loading.value = false
    }
  }

  async function save(amoebaId: number, input: SaveCategoriesInput): Promise<void> {
    await saveCategories(amoebaId, input)
    categoriesByAmoeba.value.set(amoebaId, {
      income: input.income.map((c, i) => ({
        id: undefined,
        amoeba_id: amoebaId,
        category_type: 'income' as const,
        name: c.name,
        desc: c.desc,
        sort_order: i + 1,
      })),
      expense: input.expense.map((c, i) => ({
        id: undefined,
        amoeba_id: amoebaId,
        category_type: 'expense' as const,
        name: c.name,
        desc: c.desc,
        sort_order: i + 1,
      })),
    })
  }

  async function reset(amoebaId: number): Promise<CategoryList> {
    const result = await resetCategories(amoebaId)
    categoriesByAmoeba.value.set(amoebaId, result)
    return result
  }

  function getCached(amoebaId: number): CategoryList | undefined {
    return categoriesByAmoeba.value.get(amoebaId)
  }

  return {
    categoriesByAmoeba,
    loading,
    fetchByAmoeba,
    save,
    reset,
    getCached,
  }
})
