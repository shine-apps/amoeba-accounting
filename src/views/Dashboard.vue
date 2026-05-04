<template>
  <div class="dashboard">
    <h2 style="margin-top: 0">首页仪表盘</h2>

    <!-- 统计卡片 -->
    <el-row :gutter="16" class="page-card">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <el-icon :size="32" color="#4472C4"><OfficeBuilding /></el-icon>
          <div class="stat-value">{{ amoebaStore.amoebas.length }}</div>
          <div class="stat-label">阿米巴总数</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <el-icon :size="32" color="#67c23a"><Money /></el-icon>
          <div class="stat-value">{{ formatMoney(totalSales) }}</div>
          <div class="stat-label">本月总销售额</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <el-icon :size="32" color="#e6a23c"><Timer /></el-icon>
          <div class="stat-value">{{ formatMoney(avgUnitValue) }}</div>
          <div class="stat-label">平均单位时间附加值</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <el-icon :size="32" color="#f56c6c"><TrendCharts /></el-icon>
          <div class="stat-value">{{ formatPercent(avgValueRate) }}</div>
          <div class="stat-label">平均附加值率</div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 各阿米巴最新核算结果 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <span>各阿米巴最新核算结果</span>
      </template>
      <el-table :data="latestResults" border stripe v-loading="loading" style="width: 100%">
        <el-table-column prop="amoeba_name" label="阿米巴名称" min-width="140" />
        <el-table-column prop="leader" label="负责人" width="100" />
        <el-table-column prop="period" label="核算期间" min-width="160" />
        <el-table-column label="总销售额" width="140" align="right">
          <template #default="{ row }">
            {{ formatMoney(row.total_sales) }}
          </template>
        </el-table-column>
        <el-table-column label="总费用" width="140" align="right">
          <template #default="{ row }">
            {{ formatMoney(row.total_expense) }}
          </template>
        </el-table-column>
        <el-table-column label="附加价值" width="140" align="right">
          <template #default="{ row }">
            <span :class="{ negative: row.added_value < 0 }">
              {{ formatMoney(row.added_value) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="单位时间附加值" width="160" align="right">
          <template #default="{ row }">
            <strong :class="{ negative: row.unit_value < 0 }">
              {{ formatMoney(row.unit_value) }}
            </strong>
          </template>
        </el-table-column>
        <el-table-column label="附加值率" width="100" align="right">
          <template #default="{ row }">
            {{ formatPercent(row.value_rate) }}
          </template>
        </el-table-column>
      </el-table>
      <el-empty v-if="!loading && latestResults.length === 0" description="暂无数据" />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAmoebaStore } from '@/stores/amoeba'
import { useRecordStore } from '@/stores/record'
import { useTauri } from '@/composables/useTauri'
import { formatMoney, formatPercent } from '@/utils/format'
import type { AccountingRecord } from '@/types/record'

const amoebaStore = useAmoebaStore()
const recordStore = useRecordStore()
const { listRecords } = useTauri()

const loading = ref(false)
const allLatestRecords = ref<AccountingRecord[]>([])

interface LatestResult {
  amoeba_name: string
  leader: string
  period: string
  total_sales: number
  total_expense: number
  added_value: number
  unit_value: number
  value_rate: number
}

const latestResults = computed<LatestResult[]>(() => {
  return allLatestRecords.value
    .filter((r) => r.result)
    .map((r) => {
      const amoeba = amoebaStore.amoebas.find((a) => a.id === r.amoeba_id)
      return {
        amoeba_name: amoeba?.name || '未知',
        leader: amoeba?.leader || '-',
        period: `${r.period_start} ~ ${r.period_end}`,
        total_sales: r.result!.total_sales,
        total_expense: r.result!.total_expense,
        added_value: r.result!.added_value,
        unit_value: r.result!.unit_value,
        value_rate: r.result!.value_rate,
      }
    })
})

const totalSales = computed(() => {
  return latestResults.value.reduce((sum, r) => sum + r.total_sales, 0)
})

const avgUnitValue = computed(() => {
  const valid = latestResults.value.filter((r) => r.unit_value !== 0)
  if (valid.length === 0) return 0
  return valid.reduce((sum, r) => sum + r.unit_value, 0) / valid.length
})

const avgValueRate = computed(() => {
  const valid = latestResults.value.filter((r) => r.value_rate !== 0)
  if (valid.length === 0) return 0
  return valid.reduce((sum, r) => sum + r.value_rate, 0) / valid.length
})

onMounted(async () => {
  loading.value = true
  try {
    await amoebaStore.fetchList()
    // 获取每个阿米巴的最新记录
    const promises = amoebaStore.amoebas.map(async (amoeba) => {
      try {
        const records = await listRecords(amoeba.id!)
        if (records.length > 0) {
          return records[0] // 最新记录排在第一个
        }
        return null
      } catch {
        return null
      }
    })
    const results = await Promise.all(promises)
    allLatestRecords.value = results.filter((r): r is AccountingRecord => r !== null)
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.dashboard {
  max-width: 1400px;
}

.stat-card {
  text-align: center;
  padding: 10px 0;
}

.stat-card .el-icon {
  margin-bottom: 8px;
}

.stat-value {
  font-size: 28px;
  font-weight: bold;
  color: #4472C4;
}

.stat-label {
  font-size: 14px;
  color: #909399;
  margin-top: 8px;
}

.negative {
  color: #f56c6c;
}
</style>
