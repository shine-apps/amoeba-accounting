<template>
  <div class="labor-time-editor">
    <el-form label-width="120px" size="small">
      <el-row :gutter="16">
        <el-col :span="12">
          <el-form-item label="正常工作时间">
            <el-input-number
              v-model="modelValue.normal_hours"
              :min="0"
              :precision="1"
              :controls="false"
              placeholder="0.0"
              style="width: 100%"
            />
            <span class="unit">小时</span>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="加班时间">
            <el-input-number
              v-model="modelValue.overtime_hours"
              :min="0"
              :precision="1"
              :controls="false"
              placeholder="0.0"
              style="width: 100%"
            />
            <span class="unit">小时</span>
          </el-form-item>
        </el-col>
      </el-row>
      <el-row :gutter="16">
        <el-col :span="12">
          <el-form-item label="公共时间分摊">
            <el-input-number
              v-model="modelValue.public_hours"
              :min="0"
              :precision="1"
              :controls="false"
              placeholder="0.0"
              style="width: 100%"
            />
            <span class="unit">小时</span>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="当期人数">
            <el-input-number
              v-model="modelValue.headcount"
              :min="1"
              :precision="0"
              :controls="false"
              placeholder="0"
              style="width: 100%"
            />
            <span class="unit">人</span>
          </el-form-item>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="24">
          <div class="labor-total">
            总劳动时间：<strong>{{ totalHours.toFixed(1) }}</strong> 小时
          </div>
        </el-col>
      </el-row>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { LaborTimeInput } from '@/types/record'

const modelValue = defineModel<LaborTimeInput>({ required: true })

const totalHours = computed(() => {
  return (
    (modelValue.value.normal_hours || 0) +
    (modelValue.value.overtime_hours || 0) +
    (modelValue.value.public_hours || 0)
  )
})
</script>

<style scoped>
.unit {
  margin-left: 4px;
  color: #909399;
  font-size: 12px;
}

.labor-total {
  padding: 8px 12px;
  background-color: #f5f7fa;
  border-radius: 4px;
  font-size: 14px;
  color: #606266;
}

.labor-total strong {
  color: #4472C4;
  font-size: 16px;
}
</style>
