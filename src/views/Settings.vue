<template>
  <div class="settings-page">
    <el-card shadow="never">
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center">
          <span>类别设置</span>
        </div>
      </template>

      <el-form label-width="100px">
        <el-form-item label="阿米巴组织">
          <el-select
            v-model="selectedAmoebaId"
            placeholder="请选择阿米巴"
            style="width: 320px"
            @change="handleAmoebaChange"
          >
            <el-option
              v-for="a in amoebaStore.amoebas"
              :key="a.id"
              :label="a.name"
              :value="a.id!"
            />
          </el-select>
        </el-form-item>
      </el-form>

      <el-empty
        v-if="!selectedAmoebaId"
        description="请先选择阿米巴组织"
        style="padding: 40px 0"
      />
    </el-card>

    <template v-if="selectedAmoebaId">
      <el-card shadow="never" style="margin-top: 16px">
        <template #header>
          <span>收入类别</span>
        </template>
        <el-table :data="incomeCategories" border size="small" style="width: 100%">
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
              <el-button type="danger" link size="small" @click="removeIncomeRow($index)">
                <el-icon><Delete /></el-icon>
              </el-button>
            </template>
          </el-table-column>
        </el-table>
        <div style="margin-top: 12px">
          <el-button type="primary" link size="small" @click="addIncomeRow">
            <el-icon><Plus /></el-icon>
            添加收入类别
          </el-button>
        </div>
      </el-card>

      <el-card shadow="never" style="margin-top: 16px">
        <template #header>
          <span>费用类别</span>
        </template>
        <el-table :data="expenseCategories" border size="small" style="width: 100%">
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
              <el-button type="danger" link size="small" @click="removeExpenseRow($index)">
                <el-icon><Delete /></el-icon>
              </el-button>
            </template>
          </el-table-column>
        </el-table>
        <div style="margin-top: 12px">
          <el-button type="primary" link size="small" @click="addExpenseRow">
            <el-icon><Plus /></el-icon>
            添加费用类别
          </el-button>
        </div>
      </el-card>

      <div style="margin-top: 16px; display: flex; gap: 12px; justify-content: flex-end">
        <el-button @click="handleReset">恢复默认</el-button>
        <el-button type="primary" :loading="saving" :disabled="!modified" @click="handleSave">
          保存设置
        </el-button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useAmoebaStore } from '@/stores/amoeba'
import { useCategoryStore } from '@/stores/category'
import type { AmoebaCategoryInput } from '@/types/category'

const amoebaStore = useAmoebaStore()
const categoryStore = useCategoryStore()

const selectedAmoebaId = ref<number | null>(null)
const incomeCategories = ref<AmoebaCategoryInput[]>([])
const expenseCategories = ref<AmoebaCategoryInput[]>([])
const saving = ref(false)
const modified = ref(false)
const originalIncome = ref<string>('')
const originalExpense = ref<string>('')

function markModified() {
  const curIncome = JSON.stringify(incomeCategories.value.map(({ category_type: _, ...c }) => c))
  const curExpense = JSON.stringify(expenseCategories.value.map(({ category_type: _, ...c }) => c))
  modified.value = curIncome !== originalIncome.value || curExpense !== originalExpense.value
}

function cloneCategories() {
  incomeCategories.value.forEach((c) => (c.category_type = 'income'))
  expenseCategories.value.forEach((c) => (c.category_type = 'expense'))
  originalIncome.value = JSON.stringify(incomeCategories.value.map(({ category_type: _, ...c }) => c))
  originalExpense.value = JSON.stringify(expenseCategories.value.map(({ category_type: _, ...c }) => c))
  modified.value = false
}

async function handleAmoebaChange() {
  if (modified.value) {
    try {
      await ElMessageBox.confirm('切换阿米巴将丢失未保存的修改，是否继续？', '提示', {
        type: 'warning',
        confirmButtonText: '继续',
        cancelButtonText: '取消',
      })
    } catch {
      selectedAmoebaId.value = null
      return
    }
  }

  if (!selectedAmoebaId.value) return

  const result = await categoryStore.fetchByAmoeba(selectedAmoebaId.value)
  incomeCategories.value = result.income.map((c) => ({
    category_type: 'income' as const,
    name: c.name,
    desc: c.desc,
    sort_order: c.sort_order,
  }))
  expenseCategories.value = result.expense.map((c) => ({
    category_type: 'expense' as const,
    name: c.name,
    desc: c.desc,
    sort_order: c.sort_order,
  }))
  cloneCategories()
}

function addIncomeRow() {
  const maxSort = incomeCategories.value.reduce((max, c) => Math.max(max, c.sort_order), 0)
  incomeCategories.value.push({
    category_type: 'income',
    name: '',
    desc: '',
    sort_order: maxSort + 1,
  })
  markModified()
}

function removeIncomeRow(index: number) {
  incomeCategories.value.splice(index, 1)
  markModified()
}

function addExpenseRow() {
  const maxSort = expenseCategories.value.reduce((max, c) => Math.max(max, c.sort_order), 0)
  expenseCategories.value.push({
    category_type: 'expense',
    name: '',
    desc: '',
    sort_order: maxSort + 1,
  })
  markModified()
}

function removeExpenseRow(index: number) {
  expenseCategories.value.splice(index, 1)
  markModified()
}

async function handleSave() {
  if (!selectedAmoebaId.value) return

  for (const c of incomeCategories.value) {
    if (!c.name.trim()) {
      ElMessage.warning('收入类别名称不能为空')
      return
    }
  }
  for (const c of expenseCategories.value) {
    if (!c.name.trim()) {
      ElMessage.warning('费用类别名称不能为空')
      return
    }
  }

  saving.value = true
  try {
    // Re-assign sort_order based on array position
    incomeCategories.value.forEach((c, i) => (c.sort_order = i + 1))
    expenseCategories.value.forEach((c, i) => (c.sort_order = i + 1))
    await categoryStore.save(selectedAmoebaId.value, {
      income: incomeCategories.value,
      expense: expenseCategories.value,
    })
    ElMessage.success('保存成功')
    cloneCategories()
  } catch (error: any) {
    ElMessage.error(error.message || '保存失败')
  } finally {
    saving.value = false
  }
}

async function handleReset() {
  if (!selectedAmoebaId.value) return
  try {
    await ElMessageBox.confirm('确定要恢复默认类别吗？当前设置将被覆盖。', '确认恢复', {
      type: 'warning',
      confirmButtonText: '确定',
      cancelButtonText: '取消',
    })
  } catch {
    return
  }

  const result = await categoryStore.reset(selectedAmoebaId.value)
  incomeCategories.value = result.income.map((c) => ({
    category_type: 'income' as const,
    name: c.name,
    desc: c.desc,
    sort_order: c.sort_order,
  }))
  expenseCategories.value = result.expense.map((c) => ({
    category_type: 'expense' as const,
    name: c.name,
    desc: c.desc,
    sort_order: c.sort_order,
  }))
  cloneCategories()
  ElMessage.success('已恢复默认类别')
}

onMounted(async () => {
  await amoebaStore.fetchList()
  if (amoebaStore.amoebas.length > 0) {
    selectedAmoebaId.value = amoebaStore.amoebas[0].id!
    await handleAmoebaChange()
  }
})
</script>

<style scoped>
.settings-page {
  max-width: 900px;
}
</style>
