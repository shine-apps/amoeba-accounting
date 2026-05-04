<template>
  <div class="export-page">
    <h2 style="margin-top: 0">导出</h2>

    <!-- 导出表单 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <span>导出核算数据</span>
      </template>
      <el-form :model="exportForm" label-width="100px" style="max-width: 600px">
        <el-form-item label="阿米巴">
          <el-select v-model="exportForm.amoeba_id" placeholder="请选择阿米巴" style="width: 100%">
            <el-option
              v-for="a in amoebaStore.amoebas"
              :key="a.id"
              :label="a.name"
              :value="a.id!"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="核算周期">
          <el-select v-model="exportForm.period_type" placeholder="请选择" style="width: 100%">
            <el-option
              v-for="p in PERIOD_TYPES"
              :key="p.value"
              :label="p.label"
              :value="p.value"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="日期范围">
          <el-date-picker
            v-model="dateRange"
            type="daterange"
            range-separator="至"
            start-placeholder="开始日期"
            end-placeholder="结束日期"
            value-format="YYYY-MM-DD"
            style="width: 100%"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="exporting" @click="handleExport">
            <el-icon><Download /></el-icon>
            导出 Excel
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 导出历史 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <span>导出历史</span>
      </template>
      <el-table :data="exportHistory" border stripe style="width: 100%">
        <el-table-column prop="amoeba_name" label="阿米巴" min-width="140" />
        <el-table-column prop="period_type" label="周期" width="100" />
        <el-table-column prop="period_start" label="开始日期" width="120" />
        <el-table-column prop="period_end" label="结束日期" width="120" />
        <el-table-column prop="exported_at" label="导出时间" min-width="180" />
        <el-table-column prop="file_path" label="文件路径" min-width="200" show-overflow-tooltip />
      </el-table>
      <el-empty v-if="exportHistory.length === 0" description="暂无导出记录" />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { useAmoebaStore } from '@/stores/amoeba'
import { useTauri } from '@/composables/useTauri'
import { PERIOD_TYPES } from '@/utils/constants'

const amoebaStore = useAmoebaStore()
const { exportExcel } = useTauri()

const exporting = ref(false)
const dateRange = ref<string[]>([])

const exportForm = reactive({
  amoeba_id: undefined as number | undefined,
  period_type: 'month' as string,
  period_start: '' as string,
  period_end: '' as string,
})

interface ExportHistoryItem {
  amoeba_name: string
  period_type: string
  period_start: string
  period_end: string
  exported_at: string
  file_path: string
}

const exportHistory = ref<ExportHistoryItem[]>([])

async function handleExport() {
  if (!exportForm.amoeba_id) {
    ElMessage.warning('请选择阿米巴')
    return
  }
  if (!dateRange.value || dateRange.value.length !== 2) {
    ElMessage.warning('请选择日期范围')
    return
  }

  exportForm.period_start = dateRange.value[0]
  exportForm.period_end = dateRange.value[1]

  exporting.value = true
  try {
    const filePath = await exportExcel(
      exportForm.amoeba_id,
      exportForm.period_type,
      exportForm.period_start,
      exportForm.period_end
    )
    ElMessage.success(`导出成功：${filePath}`)

    // 添加到历史记录
    const amoeba = amoebaStore.amoebas.find((a) => a.id === exportForm.amoeba_id)
    exportHistory.value.unshift({
      amoeba_name: amoeba?.name || '未知',
      period_type: exportForm.period_type,
      period_start: exportForm.period_start,
      period_end: exportForm.period_end,
      exported_at: new Date().toLocaleString('zh-CN'),
      file_path: filePath,
    })
  } catch (error: any) {
    ElMessage.error(error.message || '导出失败')
  } finally {
    exporting.value = false
  }
}

onMounted(() => {
  amoebaStore.fetchList()
})
</script>

<style scoped>
.export-page {
  max-width: 1200px;
}
</style>
