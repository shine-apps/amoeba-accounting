import { invoke } from '@tauri-apps/api/core'
import type { Amoeba, AmoebaInput } from '@/types/amoeba'
import type { AccountingRecord, RecordInput } from '@/types/record'

export function useTauri() {
  // ===== 阿米巴管理 =====
  async function listAmoebas(): Promise<Amoeba[]> {
    return invoke<Amoeba[]>('list_amoebas')
  }

  async function createAmoeba(input: AmoebaInput): Promise<Amoeba> {
    return invoke<Amoeba>('create_amoeba', { input })
  }

  async function updateAmoeba(id: number, input: AmoebaInput): Promise<Amoeba> {
    return invoke<Amoeba>('update_amoeba', { id, input })
  }

  async function deleteAmoeba(id: number): Promise<void> {
    return invoke<void>('delete_amoeba', { id })
  }

  // ===== 核算记录管理 =====
  async function listRecords(amoebaId: number): Promise<AccountingRecord[]> {
    return invoke<AccountingRecord[]>('list_records', { amoebaId })
  }

  async function getRecord(id: number): Promise<AccountingRecord> {
    return invoke<AccountingRecord>('get_record', { id })
  }

  async function saveRecord(input: RecordInput): Promise<AccountingRecord> {
    return invoke<AccountingRecord>('save_record', { input })
  }

  async function deleteRecord(id: number): Promise<void> {
    return invoke<void>('delete_record', { id })
  }

  // ===== 导出 =====
  async function exportExcel(
    amoebaId: number,
    periodType: string,
    periodStart: string,
    periodEnd: string
  ): Promise<string> {
    return invoke<string>('export_excel', { amoebaId, periodType, periodStart, periodEnd })
  }

  return {
    listAmoebas,
    createAmoeba,
    updateAmoeba,
    deleteAmoeba,
    listRecords,
    getRecord,
    saveRecord,
    deleteRecord,
    exportExcel,
  }
}
