<template>
  <div class="report-view">
    <h2 style="margin-top: 0">核算报表</h2>

    <!-- 顶部筛选 -->
    <el-card shadow="never" class="page-card">
      <el-row :gutter="16" align="middle">
        <el-col :span="6">
          <el-select
            v-model="selectedAmoebaId"
            placeholder="请选择阿米巴"
            clearable
            style="width: 100%"
            @change="handleFilterChange"
          >
            <el-option
              v-for="a in amoebaStore.amoebas"
              :key="a.id"
              :label="a.name"
              :value="a.id!"
            />
          </el-select>
        </el-col>
        <el-col :span="10">
          <el-tabs v-model="periodType" @tab-change="handleFilterChange">
            <el-tab-pane label="按月" name="month" />
            <el-tab-pane label="按周" name="week" />
            <el-tab-pane label="按日" name="day" />
          </el-tabs>
        </el-col>
        <el-col :span="8">
          <el-date-picker
            v-model="dateRange"
            type="daterange"
            range-separator="至"
            start-placeholder="开始日期"
            end-placeholder="结束日期"
            value-format="YYYY-MM-DD"
            style="width: 100%"
            @change="handleFilterChange"
          />
        </el-col>
      </el-row>
    </el-card>

    <!-- 核算表格 -->
    <el-card shadow="never" class="page-card" v-loading="loading">
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center">
          <span>{{ reportTitle }}</span>
          <el-switch
            v-model="showComparison"
            active-text="显示上期对比"
            @change="handleFilterChange"
          />
        </div>
      </template>
      <AccountingTable
        :records="displayRecords"
        :show-comparison="showComparison"
      />
      <el-empty v-if="!loading && displayRecords.length === 0" description="暂无数据" />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAmoebaStore } from '@/stores/amoeba'
import { useTauri } from '@/composables/useTauri'
import AccountingTable from '@/components/AccountingTable.vue'
import type { AccountingRecord } from '@/types/record'

const amoebaStore = useAmoebaStore()
const { listRecords } = useTauri()

const selectedAmoebaId = ref<number | undefined>(undefined)
const periodType = ref('month')
const dateRange = ref<string[]>([])
const showComparison = ref(true)
const loading = ref(false)
const allRecords = ref<AccountingRecord[]>([])

const reportTitle = computed(() => {
  const amoeba = amoebaStore.amoebas.find((a) => a.id === selectedAmoebaId.value)
  const name = amoeba ? amoeba.name : '全部阿米巴'
  return `${name} - 核算报表`
})

const displayRecords = computed<AccountingRecord[]>(() => {
  let filtered = [...allRecords.value]

  // 按核算周期筛选
  if (periodType.value) {
    filtered = filtered.filter((r) => r.period_type === periodType.value)
  }

  // 按日期范围筛选
  if (dateRange.value && dateRange.value.length === 2) {
    const start = dateRange.value[0]
    const end = dateRange.value[1]
    filtered = filtered.filter((r) => r.period_start >= start && r.period_end <= end)
  }

  // 按时间排序（最新在前）
  filtered.sort((a, b) => b.period_start.localeCompare(a.period_start))

  // 返回本期和上期（用于对比）
  if (showComparison.value && filtered.length >= 2) {
    return [filtered[0], filtered[1]]
  } else if (filtered.length === 1) {
    return [filtered[0]]
  }

  return filtered
})

async function handleFilterChange() {
  loading.value = true
  try {
    if (selectedAmoebaId.value) {
      allRecords.value = await listRecords(selectedAmoebaId.value)
    } else {
      // 获取所有阿米巴的记录
      const promises = amoebaStore.amoebas.map(async (a) => {
        try {
          return await listRecords(a.id!)
        } catch {
          return []
        }
      })
      const results = await Promise.all(promises)
      allRecords.value = results.flat()
    }
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  await amoebaStore.fetchList()
  if (amoebaStore.amoebas.length > 0) {
    selectedAmoebaId.value = amoebaStore.amoebas[0].id
  }
  await handleFilterChange()
})
</script>

<style scoped>
.report-view {
  max-width: 1400px;
}
</style>
