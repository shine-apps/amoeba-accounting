<template>
  <div class="app-container">
    <div class="app-body">
      <el-aside :width="appStore.sidebarCollapsed ? '64px' : '220px'" class="app-aside">
        <div class="sidebar-logo">
          <span v-if="!appStore.sidebarCollapsed">阿米巴核算</span>
          <span v-else>阿</span>
        </div>
        <el-menu
          :default-active="activeMenu"
          :collapse="appStore.sidebarCollapsed"
          :collapse-transition="false"
          router
          background-color="#304156"
          text-color="#bfcbd9"
          active-text-color="#409eff"
        >
          <el-menu-item index="/">
            <el-icon><DataBoard /></el-icon>
            <template #title>首页仪表盘</template>
          </el-menu-item>
          <el-menu-item index="/amoeba">
            <el-icon><OfficeBuilding /></el-icon>
            <template #title>阿米巴管理</template>
          </el-menu-item>
          <el-menu-item index="/entry">
            <el-icon><EditPen /></el-icon>
            <template #title>数据录入</template>
          </el-menu-item>
          <el-menu-item index="/report">
            <el-icon><Document /></el-icon>
            <template #title>核算报表</template>
          </el-menu-item>
          <el-menu-item index="/trend">
            <el-icon><TrendCharts /></el-icon>
            <template #title>趋势分析</template>
          </el-menu-item>
          <el-menu-item index="/export">
            <el-icon><Download /></el-icon>
            <template #title>导出</template>
          </el-menu-item>
          <el-menu-item index="/settings">
            <el-icon><Setting /></el-icon>
            <template #title>类别设置</template>
          </el-menu-item>
        </el-menu>
      </el-aside>
      <el-main class="app-main">
        <router-view />
      </el-main>
    </div>
    <el-footer height="30px" class="app-footer">
      阿米巴单位时间核算系统 v0.1.0
    </el-footer>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAppStore } from '@/stores/app'

const route = useRoute()
const appStore = useAppStore()

const activeMenu = computed(() => {
  const path = route.path
  if (path.startsWith('/entry')) return '/entry'
  return path
})
</script>

<style scoped>
.app-aside {
  background-color: #304156;
  overflow-y: auto;
}

.app-aside::-webkit-scrollbar {
  width: 0;
}

.sidebar-logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: bold;
  color: #fff;
  background-color: #4472C4;
  white-space: nowrap;
  overflow: hidden;
}
</style>
