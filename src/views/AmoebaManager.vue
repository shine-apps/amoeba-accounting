<template>
  <div class="amoeba-manager">
    <el-card shadow="never">
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center">
          <span>阿米巴管理</span>
          <el-button type="primary" @click="openDialog(null)">
            <el-icon><Plus /></el-icon>
            新增阿米巴
          </el-button>
        </div>
      </template>

      <el-table :data="amoebaStore.amoebas" border stripe v-loading="amoebaStore.loading" style="width: 100%">
        <el-table-column prop="name" label="名称" min-width="160" />
        <el-table-column prop="amoeba_type" label="类型" width="120">
          <template #default="{ row }">
            <el-tag size="small">{{ row.amoeba_type }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="leader" label="负责人" width="120" />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.status === 'active' ? 'success' : 'info'" size="small">
              {{ row.status === 'active' ? '启用' : '停用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="创建时间" width="180" />
        <el-table-column label="操作" width="160" fixed="right" align="center">
          <template #default="{ row }">
            <el-button type="primary" link size="small" @click="openDialog(row)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button type="danger" link size="small" @click="handleDelete(row)">
              <el-icon><Delete /></el-icon>
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
      <el-empty v-if="!amoebaStore.loading && amoebaStore.amoebas.length === 0" description="暂无数据" />
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑阿米巴' : '新增阿米巴'"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-width="100px"
      >
        <el-form-item label="名称" prop="name">
          <el-input v-model="formData.name" placeholder="请输入阿米巴名称" />
        </el-form-item>
        <el-form-item label="类型" prop="amoeba_type">
          <el-select v-model="formData.amoeba_type" placeholder="请选择类型" style="width: 100%">
            <el-option
              v-for="t in AMOEBA_TYPES"
              :key="t"
              :label="t"
              :value="t"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="负责人" prop="leader">
          <el-input v-model="formData.leader" placeholder="请输入负责人" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="saving" @click="handleSave">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useAmoebaStore } from '@/stores/amoeba'
import { AMOEBA_TYPES } from '@/utils/constants'
import type { Amoeba, AmoebaInput } from '@/types/amoeba'

const amoebaStore = useAmoebaStore()

const dialogVisible = ref(false)
const isEdit = ref(false)
const editId = ref<number | undefined>(undefined)
const saving = ref(false)
const formRef = ref<FormInstance>()

const formData = reactive<AmoebaInput>({
  name: '',
  amoeba_type: '',
  leader: '',
})

const formRules: FormRules = {
  name: [{ required: true, message: '请输入名称', trigger: 'blur' }],
  amoeba_type: [{ required: true, message: '请选择类型', trigger: 'change' }],
  leader: [{ required: true, message: '请输入负责人', trigger: 'blur' }],
}

function openDialog(amoeba: Amoeba | null) {
  if (amoeba) {
    isEdit.value = true
    editId.value = amoeba.id
    formData.name = amoeba.name
    formData.amoeba_type = amoeba.amoeba_type
    formData.leader = amoeba.leader
  } else {
    isEdit.value = false
    editId.value = undefined
    formData.name = ''
    formData.amoeba_type = ''
    formData.leader = ''
  }
  dialogVisible.value = true
}

async function handleSave() {
  const valid = await formRef.value?.validate().catch(() => false)
  if (!valid) return

  saving.value = true
  try {
    if (isEdit.value && editId.value) {
      await amoebaStore.update(editId.value, { ...formData })
      ElMessage.success('更新成功')
    } else {
      await amoebaStore.create({ ...formData })
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
  } catch (error: any) {
    ElMessage.error(error.message || '操作失败')
  } finally {
    saving.value = false
  }
}

async function handleDelete(amoeba: Amoeba) {
  try {
    await ElMessageBox.confirm(
      `确定要删除阿米巴「${amoeba.name}」吗？删除后不可恢复。`,
      '确认删除',
      { type: 'warning', confirmButtonText: '确定删除', cancelButtonText: '取消' }
    )
    await amoebaStore.remove(amoeba.id!)
    ElMessage.success('删除成功')
  } catch {
    // 用户取消
  }
}

onMounted(() => {
  amoebaStore.fetchList()
})
</script>

<style scoped>
.amoeba-manager {
  max-width: 1200px;
}
</style>
