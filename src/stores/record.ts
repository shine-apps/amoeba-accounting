import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AccountingRecord, RecordInput } from '@/types/record'
import { useTauri } from '@/composables/useTauri'

export const useRecordStore = defineStore('record', () => {
  const records = ref<AccountingRecord[]>([])
  const currentRecord = ref<AccountingRecord | null>(null)
  const loading = ref(false)

  const { listRecords, getRecord, saveRecord, deleteRecord } = useTauri()

  async function fetchByAmoeba(amoebaId: number) {
    loading.value = true
    try {
      records.value = await listRecords(amoebaId)
    } finally {
      loading.value = false
    }
  }

  async function fetchById(id: number) {
    loading.value = true
    try {
      currentRecord.value = await getRecord(id)
      return currentRecord.value
    } finally {
      loading.value = false
    }
  }

  async function save(input: RecordInput) {
    const record = await saveRecord(input)
    const index = records.value.findIndex((r) => r.id === record.id)
    if (index !== -1) {
      records.value[index] = record
    } else {
      records.value.unshift(record)
    }
    currentRecord.value = record
    return record
  }

  async function remove(id: number) {
    await deleteRecord(id)
    records.value = records.value.filter((r) => r.id !== id)
    if (currentRecord.value?.id === id) {
      currentRecord.value = null
    }
  }

  return {
    records,
    currentRecord,
    loading,
    fetchByAmoeba,
    fetchById,
    save,
    remove,
  }
})
