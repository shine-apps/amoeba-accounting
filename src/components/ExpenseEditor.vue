<template>
  <div class="expense-editor">
    <el-table :data="modelValue" border size="small" style="width: 100%">
      <el-table-column label="费用类别" min-width="150">
        <template #default="{ row }">
          <el-select v-model="row.category" placeholder="选择类别" size="small" style="width: 100%">
            <el-option
              v-for="cat in expenseOptions"
              :key="cat.id"
              :label="cat.name"
              :value="cat.id"
            />
          </el-select>
        </template>
      </el-table-column>
      <el-table-column label="金额" width="160" align="right">
        <template #default="{ row }">
          <el-input-number
            v-model="row.amount"
            :min="0"
            :precision="2"
            :controls="false"
            size="small"
            placeholder="0.00"
            style="width: 100%"
          />
        </template>
      </el-table-column>
      <el-table-column label="说明" min-width="200">
        <template #default="{ row }">
          <el-input v-model="row.description" size="small" placeholder="费用说明" />
        </template>
      </el-table-column>
      <el-table-column label="操作" width="70" align="center">
        <template #default="{ $index }">
          <el-button type="danger" link size="small" @click="removeRow($index)">
            <el-icon><Delete /></el-icon>
          </el-button>
        </template>
      </el-table-column>
    </el-table>
    <div class="expense-editor__footer">
      <el-button type="primary" link size="small" @click="addCustomRow">
        <el-icon><Plus /></el-icon>
        添加自定义费用
      </el-button>
      <span class="expense-editor__total">
        费用合计：<strong>{{ formatMoney(totalExpense) }}</strong> 元
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ExpenseDetailInput } from '@/types/record'
import { formatMoney } from '@/utils/format'

const props = defineProps<{
  categories: Array<{ id: number; name: string }>
}>()

const modelValue = defineModel<ExpenseDetailInput[]>({ required: true })

const expenseOptions = computed(() => props.categories)

const totalExpense = computed(() => {
  return modelValue.value.reduce((sum, e) => sum + (e.amount || 0), 0)
})

function addCustomRow() {
  const defaultId = expenseOptions.value.length > 0 ? expenseOptions.value[0].id : 10
  modelValue.value.push({
    category: defaultId,
    amount: 0,
    description: '',
  })
}

function removeRow(index: number) {
  modelValue.value.splice(index, 1)
}
</script>

<style scoped>
.expense-editor__footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
  padding: 8px 0;
}

.expense-editor__total {
  font-size: 14px;
  color: #606266;
}

.expense-editor__total strong {
  color: #4472C4;
  font-size: 16px;
}
</style>
