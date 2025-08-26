import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

// 路由组件懒加载
const Auth = () => import('@/views/Auth.vue')
const Dashboard = () => import('@/views/Dashboard.vue')
const KeyManager = () => import('@/views/KeyManager.vue')
const KeyGenerator = () => import('@/views/KeyGenerator.vue')
const ConfigEditor = () => import('@/views/ConfigEditor.vue')
const Settings = () => import('@/views/Settings.vue')

const routes: RouteRecordRaw[] = [
  {
    path: '/auth',
    name: 'Auth',
    component: Auth,
    meta: { requiresAuth: false }
  },
  {
    path: '/',
    name: 'Dashboard',
    component: Dashboard,
    meta: { requiresAuth: true }
  },
  {
    path: '/keys',
    name: 'KeyManager',
    component: KeyManager,
    meta: { requiresAuth: true }
  },
  {
    path: '/generator',
    name: 'KeyGenerator', 
    component: KeyGenerator,
    meta: { requiresAuth: true }
  },
  {
    path: '/config',
    name: 'ConfigEditor',
    component: ConfigEditor,
    meta: { requiresAuth: true }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: Settings,
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 路由守卫
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()
  
  // 检查是否需要认证
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    // 检查是否已初始化
    const isInitialized = await authStore.checkInitialization()
    
    if (!isInitialized) {
      next({ name: 'Auth', query: { mode: 'setup' } })
    } else {
      next({ name: 'Auth', query: { mode: 'login' } })
    }
    return
  }
  
  // 已认证用户访问认证页面，重定向到主页
  if (to.name === 'Auth' && authStore.isAuthenticated) {
    next({ name: 'Dashboard' })
    return
  }
  
  next()
})

export default router