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
        <div style="display: flex; justify-content: space-between; align-items: center">
          <span>销售收入</span>
          <el-button type="primary" link size="small" @click="openIncomeDialog">
            <el-icon><Setting /></el-icon>
            管理收入类别
          </el-button>
        </div>
      </template>
      <IncomeEditor v-model="formData.income_details" :categories="incomeCategoryOptions" />
    </el-card>

    <!-- 费用明细 -->
    <el-card shadow="never" class="page-card">
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center">
          <span>费用明细</span>
          <el-button type="primary" link size="small" @click="openExpenseDialog">
            <el-icon><Setting /></el-icon>
            管理费用类别
          </el-button>
        </div>
      </template>
      <ExpenseEditor v-model="formData.expenses" :categories="expenseCategoryOptions" />
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
    <!-- 管理收入类别对话框 -->
    <el-dialog
      v-model="dialogVisible"
      title="管理收入类别"
      width="700px"
      :close-on-click-modal="false"
    >
      <el-table :data="dialogIncomeCategories" border size="small" style="width: 100%">
        <el-table-column label="名称" min-width="140">
          <template #default="{ row }">
            <el-input v-model="row.name" size="small" placeholder="类别名称" />
          </template>
        </el-table-column>
        <el-table-column label="说明" min-width="200">
          <template #default="{ row }">
            <el-input v-model="row.desc" size="small" placeholder="类别说明" />
          </template>
        </el-table-column>
        <el-table-column label="排序" width="80">
          <template #default="{ row }">
            <el-input-number v-model="row.sort_order" :min="1" size="small" :controls="false" />
          </template>
        </el-table-column>
        <el-table-column label="操作" width="70" align="center">
          <template #default="{ $index }">
            <el-button type="danger" link size="small" @click="removeDialogIncomeRow($index)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </template>
        </el-table-column>
      </el-table>
      <div style="margin-top: 12px">
        <el-button type="primary" link size="small" @click="addDialogIncomeRow">
          <el-icon><Plus /></el-icon>
          添加收入类别
        </el-button>
      </div>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="dialogSaving" @click="handleDialogSave">保存</el-button>
      </template>
    </el-dialog>

    <!-- 管理费用类别对话框 -->
    <el-dialog
      v-model="expenseDialogVisible"
      title="管理费用类别"
      width="700px"
      :close-on-click-modal="false"
    >
      <el-table :data="expenseDialogCategories" border size="small" style="width: 100%">
        <el-table-column label="名称" min-width="140">
          <template #default="{ row }">
            <el-input v-model="row.name" size="small" placeholder="类别名称" />
          </template>
        </el-table-column>
        <el-table-column label="说明" min-width="200">
          <template #default="{ row }">
            <el-input v-model="row.desc" size="small" placeholder="类别说明" />
          </template>
        </el-table-column>
        <el-table-column label="排序" width="80">
          <template #default="{ row }">
            <el-input-number v-model="row.sort_order" :min="1" size="small" :controls="false" />
          </template>
        </el-table-column>
        <el-table-column label="操作" width="70" align="center">
          <template #default="{ $index }">
            <el-button type="danger" link size="small" @click="removeDialogExpenseRow($index)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </template>
        </el-table-column>
      </el-table>
      <div style="margin-top: 12px">
        <el-button type="primary" link size="small" @click="addDialogExpenseRow">
          <el-icon><Plus /></el-icon>
          添加费用类别
        </el-button>
      </div>
      <template #footer>
        <el-button @click="expenseDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="expenseDialogSaving" @click="handleExpenseDialogSave">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useAmoebaStore } from '@/stores/amoeba'
import { useRecordStore } from '@/stores/record'
import { useCategoryStore } from '@/stores/category'
import { useAccounting } from '@/composables/useAccounting'
import { PERIOD_TYPES } from '@/utils/constants'
import IncomeEditor from '@/components/IncomeEditor.vue'
import ExpenseEditor from '@/components/ExpenseEditor.vue'
import LaborTimeEditor from '@/components/LaborTimeEditor.vue'
import ResultPreview from '@/components/ResultPreview.vue'
import type { RecordInput, ExpenseDetailInput, IncomeDetailInput, LaborTimeInput } from '@/types/record'
import type { AccountingResult } from '@/types/accounting'
import type { AmoebaCategoryInput, CategoryList } from '@/types/category'

const route = useRoute()
const router = useRouter()
const amoebaStore = useAmoebaStore()
const recordStore = useRecordStore()
const categoryStore = useCategoryStore()
const { calculate } = useAccounting()

const currentCategories = ref<CategoryList | null>(null)

const incomeCategoryOptions = computed(() => {
  const cats = currentCategories.value?.income
  if (cats && cats.length > 0) {
    return cats.filter((c) => c.id != null).map((c) => ({ id: c.id!, name: c.name }))
  }
  return []
})

const expenseCategoryOptions = computed(() => {
  const cats = currentCategories.value?.expense
  if (cats && cats.length > 0) {
    return cats.filter((c) => c.id != null).map((c) => ({ id: c.id!, name: c.name }))
  }
  return []
})

// 管理收入类别对话框
const dialogVisible = ref(false)
const dialogIncomeCategories = ref<AmoebaCategoryInput[]>([])
const dialogSaving = ref(false)

function openIncomeDialog() {
  const cats = currentCategories.value?.income ?? []
  dialogIncomeCategories.value = cats.map((c) => ({
    category_type: 'income' as const,
    name: c.name,
    desc: c.desc,
    sort_order: c.sort_order,
  }))
  dialogVisible.value = true
}

function addDialogIncomeRow() {
  const maxSort = dialogIncomeCategories.value.reduce((max, c) => Math.max(max, c.sort_order), 0)
  dialogIncomeCategories.value.push({
    category_type: 'income',
    name: '',
    desc: '',
    sort_order: maxSort + 1,
  })
}

function removeDialogIncomeRow(index: number) {
  dialogIncomeCategories.value.splice(index, 1)
}

async function handleDialogSave() {
  if (!formData.amoeba_id) return

  for (const c of dialogIncomeCategories.value) {
    if (!c.name.trim()) {
      ElMessage.warning('收入类别名称不能为空')
      return
    }
  }

  dialogSaving.value = true
  try {
    dialogIncomeCategories.value.forEach((c, i) => (c.sort_order = i + 1))
    // Keep existing expense categories unchanged
    const expenseCats = currentCategories.value?.expense ?? []
    await categoryStore.save(formData.amoeba_id, {
      income: dialogIncomeCategories.value,
      expense: expenseCats.map((c) => ({
        category_type: 'expense' as const,
        name: c.name,
        desc: c.desc,
        sort_order: c.sort_order,
      })),
    })
    categoryStore.categoriesByAmoeba.delete(formData.amoeba_id)
    currentCategories.value = await categoryStore.fetchByAmoeba(formData.amoeba_id)
    dialogVisible.value = false
    ElMessage.success('保存成功')
  } catch (error: any) {
    ElMessage.error(error.message || '保存失败')
  } finally {
    dialogSaving.value = false
  }
}

// 管理费用类别对话框
const expenseDialogVisible = ref(false)
const expenseDialogCategories = ref<AmoebaCategoryInput[]>([])
const expenseDialogSaving = ref(false)

function openExpenseDialog() {
  const cats = currentCategories.value?.expense ?? []
  expenseDialogCategories.value = cats.map((c) => ({
    category_type: 'expense' as const,
    name: c.name,
    desc: c.desc,
    sort_order: c.sort_order,
  }))
  expenseDialogVisible.value = true
}

function addDialogExpenseRow() {
  const maxSort = expenseDialogCategories.value.reduce((max, c) => Math.max(max, c.sort_order), 0)
  expenseDialogCategories.value.push({
    category_type: 'expense',
    name: '',
    desc: '',
    sort_order: maxSort + 1,
  })
}

function removeDialogExpenseRow(index: number) {
  expenseDialogCategories.value.splice(index, 1)
}

async function handleExpenseDialogSave() {
  if (!formData.amoeba_id) return

  for (const c of expenseDialogCategories.value) {
    if (!c.name.trim()) {
      ElMessage.warning('费用类别名称不能为空')
      return
    }
  }

  expenseDialogSaving.value = true
  try {
    expenseDialogCategories.value.forEach((c, i) => (c.sort_order = i + 1))
    // Keep existing income categories unchanged
    const incomeCats = currentCategories.value?.income ?? []
    await categoryStore.save(formData.amoeba_id, {
      income: incomeCats.map((c) => ({
        category_type: 'income' as const,
        name: c.name,
        desc: c.desc,
        sort_order: c.sort_order,
      })),
      expense: expenseDialogCategories.value,
    })
    categoryStore.categoriesByAmoeba.delete(formData.amoeba_id)
    currentCategories.value = await categoryStore.fetchByAmoeba(formData.amoeba_id)
    expenseDialogVisible.value = false
    ElMessage.success('保存成功')
  } catch (error: any) {
    ElMessage.error(error.message || '保存失败')
  } finally {
    expenseDialogSaving.value = false
  }
}

const isEdit = computed(() => !!route.params.id)
const saving = ref(false)
const dateRange = ref<string[]>([])

const formData = reactive({
  amoeba_id: undefined as number | undefined,
  period_type: 'month' as string,
  period_start: '' as string,
  period_end: '' as string,
  remark: '',
  income_details: [] as IncomeDetailInput[],
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

watch(() => formData.amoeba_id, async (newId) => {
  if (newId) {
    currentCategories.value = await categoryStore.fetchByAmoeba(newId)
    if (!isEdit.value) {
      formData.income_details = getDefaultIncomes()
      formData.expenses = getDefaultExpenses()
    }
  }
})

const previewResult = computed<AccountingResult | null>(() => {
  if (formData.income_details.length === 0 && formData.expenses.length === 0) {
    return null
  }
  return calculate(formData.income_details, formData.expenses, formData.labor)
})

function getDefaultIncomes(): IncomeDetailInput[] {
  const cats = currentCategories.value?.income ?? []
  return cats.map((c) => ({ category: c.id!, amount: 0, description: '' }))
}

function getDefaultExpenses(): ExpenseDetailInput[] {
  const cats = currentCategories.value?.expense ?? []
  return cats.map((c) => ({ category: c.id!, amount: 0, description: '' }))
}

function resetForm() {
  formData.amoeba_id = undefined
  formData.period_type = 'month'
  formData.period_start = ''
  formData.period_end = ''
  formData.remark = ''
  formData.income_details = getDefaultIncomes()
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
    remark: formData.remark,
    income_details: formData.income_details,
    expenses: formData.expenses,
    labor: formData.labor,
  }

  saving.value = true
  try {
    const recordId = isEdit.value ? Number(route.params.id) : null
    await recordStore.save(recordId, input)
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
      formData.remark = record.remark
      formData.income_details = (record.income_details || []).map((i) => ({
        category: i.category,
        amount: i.amount,
        description: i.description,
      }))
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

      if (record.amoeba_id) {
        currentCategories.value = await categoryStore.fetchByAmoeba(record.amoeba_id)
      }
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
    if (amoebaStore.amoebas.length > 0 && !formData.amoeba_id) {
      formData.amoeba_id = amoebaStore.amoebas[0].id!
    }
    if (formData.amoeba_id) {
      currentCategories.value = await categoryStore.fetchByAmoeba(formData.amoeba_id)
    }
    formData.income_details = getDefaultIncomes()
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
