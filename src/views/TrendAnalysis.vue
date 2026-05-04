<template>
  <div class="trend-analysis">
    <h2 style="margin-top: 0">趋势分析</h2>

    <!-- 顶部筛选 -->
    <el-card shadow="never" class="page-card">
      <el-row :gutter="16" align="middle">
        <el-col :span="10">
          <el-form-item label="阿米巴" label-width="60px">
            <el-select
              v-model="selectedAmoebaIds"
              multiple
              collapse-tags
              collapse-tags-tooltip
              placeholder="请选择阿米巴（可多选）"
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
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="指标" label-width="60px">
            <el-select v-model="selectedMetric" style="width: 100%" @change="handleFilterChange">
              <el-option label="单位时间附加值" value="unit_value" />
              <el-option label="附加值率" value="value_rate" />
              <el-option label="费用率" value="expense_rate" />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="6">
          <el-form-item label="周期" label-width="60px">
            <el-select v-model="periodType" style="width: 100%" @change="handleFilterChange">
              <el-option label="按月" value="month" />
              <el-option label="按周" value="week" />
              <el-option label="按日" value="day" />
            </el-select>
          </el-form-item>
        </el-col>
      </el-row>
    </el-card>

    <!-- 图表 -->
    <el-card shadow="never" class="page-card" v-loading="loading">
      <template #header>
        <span>{{ metricLabel }}趋势</span>
      </template>
      <div ref="chartContainer" style="width: 100%; height: 400px">
        <v-chart :option="chartOption" autoresize style="width: 100%; height: 100%" />
      </div>
      <el-empty v-if="!loading && chartSeries.length === 0" description="暂无数据" />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent,
} from 'echarts/components'
import VChart from 'vue-echarts'
import { useAmoebaStore } from '@/stores/amoeba'
import { useTauri } from '@/composables/useTauri'
import { formatMoney, formatPercent } from '@/utils/format'
import type { AccountingRecord } from '@/types/record'

use([
  CanvasRenderer,
  LineChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent,
])

const amoebaStore = useAmoebaStore()
const { listRecords } = useTauri()

const selectedAmoebaIds = ref<number[]>([])
const selectedMetric = ref('unit_value')
const periodType = ref('month')
const loading = ref(false)
const chartContainer = ref<HTMLElement>()

interface SeriesData {
  amoebaId: number
  amoebaName: string
  periods: string[]
  values: number[]
}

const chartSeries = ref<SeriesData[]>([])

const metricLabel = computed(() => {
  const map: Record<string, string> = {
    unit_value: '单位时间附加值',
    value_rate: '附加值率',
    expense_rate: '费用率',
  }
  return map[selectedMetric.value] || selectedMetric.value
})

const chartColors = ['#4472C4', '#ed7d31', '#a5a5a5', '#ffc000', '#5b9bd5', '#70ad47']

const chartOption = computed(() => {
  if (chartSeries.value.length === 0) return {}

  const allPeriods = chartSeries.value.flatMap((s) => s.periods)
  const uniquePeriods = [...new Set(allPeriods)].sort()

  const isPercent = selectedMetric.value === 'value_rate' || selectedMetric.value === 'expense_rate'

  return {
    tooltip: {
      trigger: 'axis',
      formatter: (params: any) => {
        let html = `<strong>${params[0].axisValue}</strong><br/>`
        params.forEach((p: any) => {
          const val = isPercent ? formatPercent(p.value) : formatMoney(p.value)
          html += `${p.marker} ${p.seriesName}: ${val}<br/>`
        })
        return html
      },
    },
    legend: {
      top: 0,
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: uniquePeriods,
    },
    yAxis: {
      type: 'value',
      axisLabel: {
        formatter: isPercent ? '{value}%' : undefined,
      },
    },
    series: chartSeries.value.map((s, index) => ({
      name: s.amoebaName,
      type: 'line',
      smooth: true,
      data: uniquePeriods.map((period) => {
        const idx = s.periods.indexOf(period)
        return idx !== -1 ? s.values[idx] : null
      }),
      itemStyle: {
        color: chartColors[index % chartColors.length],
      },
    })),
  }
})

async function handleFilterChange() {
  if (selectedAmoebaIds.value.length === 0) {
    chartSeries.value = []
    return
  }

  loading.value = true
  try {
    const seriesData: SeriesData[] = []

    for (const amoebaId of selectedAmoebaIds.value) {
      const amoeba = amoebaStore.amoebas.find((a) => a.id === amoebaId)
      if (!amoeba) continue

      const records = await listRecords(amoebaId)
      const filtered = records
        .filter((r) => r.period_type === periodType.value && r.result)
        .sort((a, b) => a.period_start.localeCompare(b.period_start))

      seriesData.push({
        amoebaId,
        amoebaName: amoeba.name,
        periods: filtered.map((r) => r.period_start),
        values: filtered.map((r) => {
          const result = r.result!
          return result[selectedMetric.value as keyof typeof result] as number
        }),
      })
    }

    chartSeries.value = seriesData
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  await amoebaStore.fetchList()
  if (amoebaStore.amoebas.length > 0) {
    selectedAmoebaIds.value = [amoebaStore.amoebas[0].id!]
    await handleFilterChange()
  }
})
</script>

<style scoped>
.trend-analysis {
  max-width: 1400px;
}
</style>
