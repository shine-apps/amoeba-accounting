<template>
  <div class="accounting-table">
    <el-table :data="tableData" border stripe show-summary :summary-method="getSummary" style="width: 100%">
      <el-table-column prop="item" label="项目" min-width="180" />
      <el-table-column prop="current" label="本期" min-width="140" align="right">
        <template #default="{ row }">
          <span :class="{ negative: row.current < 0 && row.isValue }">
            {{ formatCellValue(row.current, row.type) }}
          </span>
        </template>
      </el-table-column>
      <el-table-column v-if="showComparison" prop="previous" label="上期" min-width="140" align="right">
        <template #default="{ row }">
          {{ formatCellValue(row.previous, row.type) }}
        </template>
      </el-table-column>
      <el-table-column v-if="showComparison" prop="change" label="增减" min-width="140" align="right">
        <template #default="{ row }">
          <span v-if="row.change !== null && row.change !== undefined" :class="getChangeClass(row)">
            {{ formatChange(row) }}
          </span>
          <span v-else>-</span>
        </template>
      </el-table-column>
      <el-table-column v-if="showComparison" prop="changeRate" label="增减率" min-width="120" align="right">
        <template #default="{ row }">
          <span v-if="row.changeRate !== null && row.changeRate !== undefined" :class="getChangeClass(row)">
            {{ formatPercent(row.changeRate) }}
          </span>
          <span v-else>-</span>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { AccountingRecord } from '@/types/record'
import { formatMoney, formatPercent } from '@/utils/format'
import { EXPENSE_CATEGORIES } from '@/utils/constants'

interface TableRow {
  item: string
  current: number
  previous?: number
  change?: number
  changeRate?: number
  type: 'money' | 'hours' | 'percent' | 'count'
  isValue?: boolean
  rowType?: 'section' | 'total' | 'key' | 'normal'
}

const props = withDefaults(defineProps<{
  records: AccountingRecord[]
  showComparison?: boolean
}>(), {
  showComparison: false,
})

function getExpenseByCategory(record: AccountingRecord, categoryCode: string): number {
  if (!record.expenses || record.expenses.length === 0) return 0
  const expense = record.expenses.find((e) => e.category === categoryCode)
  return expense ? expense.amount : 0
}

function getLaborHours(record: AccountingRecord): { normal: number; overtime: number; public: number; total: number } {
  if (!record.labor) return { normal: 0, overtime: 0, public: 0, total: 0 }
  const normal = record.labor.normal_hours || 0
  const overtime = record.labor.overtime_hours || 0
  const pub = record.labor.public_hours || 0
  return { normal, overtime, public: pub, total: normal + overtime + pub }
}

function buildRow(
  item: string,
  current: number,
  previous: number | undefined,
  type: 'money' | 'hours' | 'percent' | 'count',
  isValue = false,
  rowType: 'section' | 'total' | 'key' | 'normal' = 'normal'
): TableRow {
  const row: TableRow = { item, current, type, isValue, rowType }
  if (previous !== undefined && props.showComparison) {
    row.previous = previous
    if (previous !== 0) {
      row.change = current - previous
      row.changeRate = ((current - previous) / Math.abs(previous)) * 100
    }
  }
  return row
}

const tableData = computed<TableRow[]>(() => {
  const current = props.records[0]
  const previous = props.showComparison && props.records.length > 1 ? props.records[1] : undefined

  if (!current) return []

  const rows: TableRow[] = []

  // 销售收入
  rows.push(buildRow('销售收入', 0, undefined, 'money', false, 'section'))
  rows.push(buildRow('  对外销售额', current.external_sales, previous?.external_sales, 'money'))
  rows.push(buildRow('  内部交易额', current.internal_sales, previous?.internal_sales, 'money'))
  const currentTotalSales = current.external_sales + current.internal_sales
  const prevTotalSales = previous ? previous.external_sales + previous.internal_sales : undefined
  rows.push(buildRow('  总销售额', currentTotalSales, prevTotalSales, 'money', false, 'total'))

  // 费用
  rows.push(buildRow('费用', 0, undefined, 'money', false, 'section'))
  let currentTotalExpense = 0
  let prevTotalExpense = 0
  EXPENSE_CATEGORIES.forEach((cat) => {
    const cur = getExpenseByCategory(current, cat.code)
    currentTotalExpense += cur
    const prev = previous ? getExpenseByCategory(previous, cat.code) : undefined
    if (prev !== undefined) prevTotalExpense += prev
    rows.push(buildRow(`  ${cat.name}`, cur, prev, 'money'))
  })
  rows.push(buildRow('  总费用', currentTotalExpense, previous ? prevTotalExpense : undefined, 'money', false, 'total'))

  // 附加价值
  const currentAddedValue = currentTotalSales - currentTotalExpense
  const prevAddedValue = previous && prevTotalSales !== undefined ? prevTotalSales - prevTotalExpense : undefined
  const currentValueRate = currentTotalSales > 0 ? (currentAddedValue / currentTotalSales) * 100 : 0
  const prevValueRate = previous && prevTotalSales !== undefined && prevTotalSales > 0 ? (prevAddedValue! / prevTotalSales) * 100 : undefined
  rows.push(buildRow('附加价值', currentAddedValue, prevAddedValue, 'money', true, 'section'))
  rows.push(buildRow('  附加值率', currentValueRate, prevValueRate, 'percent'))

  // 劳动时间
  const curLabor = getLaborHours(current)
  const prevLabor = previous ? getLaborHours(previous) : undefined
  rows.push(buildRow('劳动时间', 0, undefined, 'hours', false, 'section'))
  rows.push(buildRow('  正常工作时间', curLabor.normal, prevLabor?.normal, 'hours'))
  rows.push(buildRow('  加班时间', curLabor.overtime, prevLabor?.overtime, 'hours'))
  rows.push(buildRow('  公共时间分摊', curLabor.public, prevLabor?.public, 'hours'))
  rows.push(buildRow('  总时间', curLabor.total, prevLabor?.total, 'hours', false, 'total'))
  rows.push(buildRow('  当期人数', current.labor?.headcount || 0, previous?.labor?.headcount, 'count'))

  // 核心指标
  const currentUnitValue = curLabor.total > 0 ? currentAddedValue / curLabor.total : 0
  const prevUnitValue = prevLabor && prevLabor.total > 0 && prevAddedValue !== undefined ? prevAddedValue / prevLabor.total : undefined
  const headcount = Math.max(current.labor?.headcount || 0, 1)
  const prevHeadcount = Math.max(previous?.labor?.headcount || 0, 1)
  rows.push(buildRow('核心指标', 0, undefined, 'money', false, 'section'))
  rows.push(buildRow('  单位时间附加值', currentUnitValue, prevUnitValue, 'money', false, 'key'))
  rows.push(buildRow('  人均销售额', currentTotalSales / headcount, prevTotalSales !== undefined ? prevTotalSales / prevHeadcount : undefined, 'money'))
  rows.push(buildRow('  人均附加值', currentAddedValue / headcount, prevAddedValue !== undefined ? prevAddedValue / prevHeadcount : undefined, 'money', true))
  rows.push(buildRow('  费用率', currentTotalSales > 0 ? (currentTotalExpense / currentTotalSales) * 100 : 0, prevTotalSales && prevTotalSales > 0 ? (prevTotalExpense / prevTotalSales) * 100 : undefined, 'percent'))

  return rows
})

function formatCellValue(value: number, type: string): string {
  if (value === 0 && type !== 'count') return '-'
  switch (type) {
    case 'money':
      return formatMoney(value)
    case 'hours':
      return value.toFixed(1) + 'h'
    case 'percent':
      return formatPercent(value)
    case 'count':
      return String(value)
    default:
      return String(value)
  }
}

function formatChange(row: TableRow): string {
  if (row.change === undefined || row.change === null) return '-'
  const prefix = row.change > 0 ? '+' : ''
  switch (row.type) {
    case 'money':
      return prefix + formatMoney(row.change)
    case 'hours':
      return prefix + row.change.toFixed(1) + 'h'
    case 'percent':
      return prefix + formatPercent(row.change)
    case 'count':
      return prefix + String(row.change)
    default:
      return prefix + String(row.change)
  }
}

function getChangeClass(row: TableRow): string {
  if (row.change === undefined || row.change === null) return ''
  // 对于费用率，下降是好的
  if (row.item.includes('费用率')) {
    return row.change < 0 ? 'positive-change' : 'negative-change'
  }
  // 对于单位时间附加值、附加值率等，上升是好的
  if (row.change > 0) return 'positive-change'
  if (row.change < 0) return 'negative-change'
  return ''
}

function getSummary({ columns }: { columns: any[] }) {
  return columns.map((col: any, index: number) => {
    if (index === 0) return '合计'
    return ''
  })
}
</script>
