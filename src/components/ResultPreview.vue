<template>
  <div class="result-preview" v-if="result">
    <el-row :gutter="16">
      <el-col :span="6">
        <div class="result-item">
          <div class="result-label">总销售额</div>
          <div class="result-value">{{ formatMoney(result.total_sales) }}</div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="result-item">
          <div class="result-label">总费用</div>
          <div class="result-value">{{ formatMoney(result.total_expense) }}</div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="result-item">
          <div class="result-label">附加价值</div>
          <div class="result-value" :class="{ negative: result.added_value < 0 }">
            {{ formatMoney(result.added_value) }}
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="result-item">
          <div class="result-label">总劳动时间</div>
          <div class="result-value">{{ result.total_hours.toFixed(1) }}h</div>
        </div>
      </el-col>
    </el-row>
    <el-row :gutter="16" style="margin-top: 16px">
      <el-col :span="6">
        <div class="result-item key">
          <div class="result-label">单位时间附加值</div>
          <div class="result-value highlight" :class="{ negative: result.unit_value < 0 }">
            {{ formatMoney(result.unit_value) }}
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="result-item">
          <div class="result-label">人均销售额</div>
          <div class="result-value">{{ formatMoney(result.sales_per_person) }}</div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="result-item">
          <div class="result-label">人均附加值</div>
          <div class="result-value" :class="{ negative: result.value_per_person < 0 }">
            {{ formatMoney(result.value_per_person) }}
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="result-item">
          <div class="result-label">附加值率 / 费用率</div>
          <div class="result-value">
            <span :class="{ negative: result.value_rate < 0 }">{{ formatPercent(result.value_rate) }}</span>
            /
            <span>{{ formatPercent(result.expense_rate) }}</span>
          </div>
        </div>
      </el-col>
    </el-row>
  </div>
  <div v-else class="result-preview--empty">
    <el-empty description="请填写销售、费用和劳动时间数据以预览核算结果" :image-size="60" />
  </div>
</template>

<script setup lang="ts">
import type { AccountingResult } from '@/types/accounting'
import { formatMoney, formatPercent } from '@/utils/format'

defineProps<{
  result: AccountingResult | null
}>()
</script>

<style scoped>
.result-preview {
  padding: 16px;
  background-color: #fafbfc;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
}

.result-item {
  padding: 12px;
  background-color: #fff;
  border-radius: 6px;
  text-align: center;
  border: 1px solid #ebeef5;
}

.result-item.key {
  background-color: #fff2cc;
  border-color: #f5d76e;
}

.result-label {
  font-size: 12px;
  color: #909399;
  margin-bottom: 8px;
}

.result-value {
  font-size: 18px;
  font-weight: bold;
  color: #303133;
}

.result-value.highlight {
  font-size: 22px;
  color: #4472C4;
}

.result-value.negative {
  color: #f56c6c;
}

.result-preview--empty {
  padding: 20px;
  text-align: center;
}
</style>
