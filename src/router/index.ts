import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Dashboard',
      component: () => import('@/views/Dashboard.vue'),
    },
    {
      path: '/amoeba',
      name: 'AmoebaManager',
      component: () => import('@/views/AmoebaManager.vue'),
    },
    {
      path: '/entry',
      name: 'DataEntryNew',
      component: () => import('@/views/DataEntry.vue'),
    },
    {
      path: '/entry/:id',
      name: 'DataEntryEdit',
      component: () => import('@/views/DataEntry.vue'),
    },
    {
      path: '/report',
      name: 'ReportView',
      component: () => import('@/views/ReportView.vue'),
    },
    {
      path: '/trend',
      name: 'TrendAnalysis',
      component: () => import('@/views/TrendAnalysis.vue'),
    },
    {
      path: '/export',
      name: 'ExportPage',
      component: () => import('@/views/ExportPage.vue'),
    },
    {
      path: '/settings',
      name: 'Settings',
      component: () => import('@/views/Settings.vue'),
    },
  ],
})

export default router
