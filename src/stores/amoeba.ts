import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Amoeba, AmoebaInput } from '@/types/amoeba'
import { useTauri } from '@/composables/useTauri'

export const useAmoebaStore = defineStore('amoeba', () => {
  const amoebas = ref<Amoeba[]>([])
  const currentAmoeba = ref<Amoeba | null>(null)
  const loading = ref(false)

  const { listAmoebas, createAmoeba, updateAmoeba, deleteAmoeba } = useTauri()

  async function fetchList() {
    loading.value = true
    try {
      amoebas.value = await listAmoebas()
    } finally {
      loading.value = false
    }
  }

  async function create(input: AmoebaInput) {
    const amoeba = await createAmoeba(input)
    amoebas.value.push(amoeba)
    return amoeba
  }

  async function update(id: number, input: AmoebaInput) {
    const amoeba = await updateAmoeba(id, input)
    const index = amoebas.value.findIndex((a) => a.id === id)
    if (index !== -1) {
      amoebas.value[index] = amoeba
    }
    if (currentAmoeba.value?.id === id) {
      currentAmoeba.value = amoeba
    }
    return amoeba
  }

  async function remove(id: number) {
    await deleteAmoeba(id)
    amoebas.value = amoebas.value.filter((a) => a.id !== id)
    if (currentAmoeba.value?.id === id) {
      currentAmoeba.value = null
    }
  }

  return {
    amoebas,
    currentAmoeba,
    loading,
    fetchList,
    create,
    update,
    remove,
  }
})
