<template>
  <div class="data-entry">
    <h2 style="margin-top: 0">{{ isEdit ? '编辑核算记录' : '新建核算记录' }}</h2>

    <!-- 顶部选择区 -->
    <el-card shadow="never" class="page-card">
      <el-form :model="formData" label-width="100px" size="default" :inline="false">
        <el-row :gutter="16">
          <el-col :span="8">
            <el-form-item label="阿米巴">
              <el-select
                v-model="formData.amoeba_id"
                placeholder="请选择阿米巴"
                style="width: 100%"
                :disabled="isEdit"
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
          <el-col :span="6">
            <el-form-item label="核算周期">
              <el-select v-model="formData.period_type" placeholder="请选择" style="width: 100%">
                <el-option
                  v-for="p in PERIOD_TYPES"
                  :key="p.value"
                  :label="p.label"
                  :value="p.value"
                />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="10">
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
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="24">
            <el-form-item label="备注">
              <el-input v-model="formData.remark" type="textarea" :rows="2" placeholder="可选备注" />
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
    </el-card>

    <!-- 销售收入 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <span>销售收入</span>
      </template>
      <el-row :gutter="16">
        <el-col :span="8">
          <el-form-item label="对外销售额">
            <el-input-number
              v-model="formData.external_sales"
              :min="0"
              :precision="2"
              :controls="false"
              placeholder="0.00"
              style="width: 100%"
            />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="内部交易额">
            <el-input-number
              v-model="formData.internal_sales"
              :min="0"
              :precision="2"
              :controls="false"
              placeholder="0.00"
              style="width: 100%"
            />
          </el-form-item>
        </el-col>
        <el-col :span="8">
          <el-form-item label="合计">
            <div class="total-display">
              {{ formatMoney(formData.external_sales + formData.internal_sales) }} 元
            </div>
          </el-form-item>
        </el-col>
      </el-row>
    </el-card>

    <!-- 费用明细 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <span>费用明细</span>
      </template>
      <ExpenseEditor v-model="formData.expenses" />
    </el-card>

    <!-- 劳动时间 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <span>劳动时间</span>
      </template>
      <LaborTimeEditor v-model="formData.labor" />
    </el-card>

    <!-- 核算结果预览 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <span>核算结果预览</span>
      </template>
      <ResultPreview :result="previewResult" />
    </el-card>

    <!-- 底部按钮 -->
    <div class="action-bar">
      <el-button @click="handleCancel">取消</el-button>
      <el-button @click="handleReset">重置</el-button>
      <el-button type="primary" :loading="saving" @click="handleSave">保存</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useAmoebaStore } from '@/stores/amoeba'
import { useRecordStore } from '@/stores/record'
import { useAccounting } from '@/composables/useAccounting'
import { PERIOD_TYPES } from '@/utils/constants'
import { formatMoney } from '@/utils/format'
import ExpenseEditor from '@/components/ExpenseEditor.vue'
import LaborTimeEditor from '@/components/LaborTimeEditor.vue'
import ResultPreview from '@/components/ResultPreview.vue'
import type { RecordInput, ExpenseDetailInput, LaborTimeInput } from '@/types/record'
import type { AccountingResult } from '@/types/accounting'

const route = useRoute()
const router = useRouter()
const amoebaStore = useAmoebaStore()
const recordStore = useRecordStore()
const { calculate } = useAccounting()

const isEdit = computed(() => !!route.params.id)
const saving = ref(false)
const dateRange = ref<string[]>([])

const formData = reactive({
  amoeba_id: undefined as number | undefined,
  period_type: 'month' as string,
  period_start: '' as string,
  period_end: '' as string,
  external_sales: 0,
  internal_sales: 0,
  remark: '',
  expenses: [] as ExpenseDetailInput[],
  labor: {
    normal_hours: 0,
    overtime_hours: 0,
    public_hours: 0,
    headcount: 1,
  } as LaborTimeInput,
})

watch(dateRange, (val) => {
  if (val && val.length === 2) {
    formData.period_start = val[0]
    formData.period_end = val[1]
  } else {
    formData.period_start = ''
    formData.period_end = ''
  }
})

const previewResult = computed<AccountingResult | null>(() => {
  if (formData.expenses.length === 0 && formData.external_sales === 0 && formData.internal_sales === 0) {
    return null
  }
  return calculate(formData.expenses, formData.labor, formData.external_sales, formData.internal_sales)
})

function getDefaultExpenses(): ExpenseDetailInput[] {
  return [
    { category: 'material', amount: 0, description: '' },
    { category: 'outsourcing', amount: 0, description: '' },
    { category: 'electricity', amount: 0, description: '' },
    { category: 'depreciation', amount: 0, description: '' },
    { category: 'transport', amount: 0, description: '' },
    { category: 'maintenance', amount: 0, description: '' },
    { category: 'office', amount: 0, description: '' },
    { category: 'communication', amount: 0, description: '' },
    { category: 'travel', amount: 0, description: '' },
    { category: 'other', amount: 0, description: '' },
  ]
}

function resetForm() {
  formData.amoeba_id = undefined
  formData.period_type = 'month'
  formData.period_start = ''
  formData.period_end = ''
  formData.external_sales = 0
  formData.internal_sales = 0
  formData.remark = ''
  formData.expenses = getDefaultExpenses()
  formData.labor = {
    normal_hours: 0,
    overtime_hours: 0,
    public_hours: 0,
    headcount: 1,
  }
  dateRange.value = []
}

function handleReset() {
  if (isEdit.value) {
    loadRecord()
  } else {
    resetForm()
  }
}

function handleCancel() {
  router.push('/')
}

async function handleSave() {
  if (!formData.amoeba_id) {
    ElMessage.warning('请选择阿米巴')
    return
  }
  if (!formData.period_start || !formData.period_end) {
    ElMessage.warning('请选择日期范围')
    return
  }

  const input: RecordInput = {
    amoeba_id: formData.amoeba_id,
    period_type: formData.period_type,
    period_start: formData.period_start,
    period_end: formData.period_end,
    external_sales: formData.external_sales,
    internal_sales: formData.internal_sales,
    remark: formData.remark,
    expenses: formData.expenses,
    labor: formData.labor,
  }

  saving.value = true
  try {
    await recordStore.save(input)
    ElMessage.success('保存成功')
    router.push('/')
  } catch (error: any) {
    ElMessage.error(error.message || '保存失败')
  } finally {
    saving.value = false
  }
}

async function loadRecord() {
  const id = Number(route.params.id)
  if (!id) return

  try {
    const record = await recordStore.fetchById(id)
    if (record) {
      formData.amoeba_id = record.amoeba_id
      formData.period_type = record.period_type
      formData.period_start = record.period_start
      formData.period_end = record.period_end
      formData.external_sales = record.external_sales
      formData.internal_sales = record.internal_sales
      formData.remark = record.remark
      formData.expenses = record.expenses.map((e) => ({
        category: e.category,
        amount: e.amount,
        description: e.description,
      }))
      if (record.labor) {
        formData.labor = {
          normal_hours: record.labor.normal_hours,
          overtime_hours: record.labor.overtime_hours,
          public_hours: record.labor.public_hours,
          headcount: record.labor.headcount,
        }
      }
      dateRange.value = [record.period_start, record.period_end]
    }
  } catch (error: any) {
    ElMessage.error(error.message || '加载记录失败')
  }
}

onMounted(async () => {
  await amoebaStore.fetchList()
  if (isEdit.value) {
    await loadRecord()
  } else {
    formData.expenses = getDefaultExpenses()
  }
})
</script>

<style scoped>
.data-entry {
  max-width: 1200px;
}

.total-display {
  font-size: 18px;
  font-weight: bold;
  color: #4472C4;
  line-height: 32px;
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 0;
}
</style>
