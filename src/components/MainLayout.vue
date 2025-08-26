<template>
  <div class="min-h-screen bg-transparent">
    <!-- 顶部导航 -->
    <TopNavigation @logout="handleLogout" />

    <div class="flex relative">
      <!-- 侧边导航 -->
      <SideNavigation 
        :navigation="navigation"
        :current-route="$route.name"
      />

      <!-- 主内容区 -->
      <MainContent :class="contentClass">
        <RouterView v-slot="{ Component, route }">
          <Transition
            name="page"
            mode="out-in"
            appear
          >
            <component :is="Component" :key="route.path" v-if="Component" />
          </Transition>
        </RouterView>
      </MainContent>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, provide, onMounted, onUnmounted } from 'vue'
import { useRouter, RouterView } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import TopNavigation from './TopNavigation.vue'
import SideNavigation from './SideNavigation.vue'
import MainContent from './MainContent.vue'
import {
  HomeIcon,
  KeyIcon,
  PlusIcon,
  DocumentTextIcon,
  Cog6ToothIcon,
} from '@heroicons/vue/24/outline'

const router = useRouter()
const { t } = useI18n()
const authStore = useAuthStore()

// 响应式状态
const sidebarCollapsed = ref(false)
const mobileMenuOpen = ref(false)
const isMobile = ref(false)

// 检测移动端
const checkMobile = () => {
  isMobile.value = window.innerWidth < 768
  if (isMobile.value) {
    sidebarCollapsed.value = false // 移动端不使用折叠模式
  }
}

// 切换侧边栏折叠状态
const toggleSidebarCollapse = () => {
  if (!isMobile.value) {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }
}

// 切换移动端菜单
const toggleMobileMenu = () => {
  mobileMenuOpen.value = !mobileMenuOpen.value
}

// 关闭移动端菜单
const closeMobileMenu = () => {
  mobileMenuOpen.value = false
}

// 主内容区样式
const contentClass = computed(() => {
  if (isMobile.value) {
    return 'flex-1 transition-all duration-300'
  }
  return sidebarCollapsed.value 
    ? 'flex-1 ml-16 transition-all duration-300' 
    : 'flex-1 transition-all duration-300'
})

// 向子组件提供状态和方法
provide('sidebarCollapsed', sidebarCollapsed)
provide('mobileMenuOpen', mobileMenuOpen)
provide('isMobile', isMobile)
provide('toggleSidebarCollapse', toggleSidebarCollapse)
provide('toggleMobileMenu', toggleMobileMenu)
provide('closeMobileMenu', closeMobileMenu)

// 导航菜单配置
const navigation = [
  { name: 'Dashboard', label: computed(() => t('nav.dashboard')), href: '/', icon: HomeIcon },
  { name: 'KeyManager', label: computed(() => t('nav.keyManager')), href: '/keys', icon: KeyIcon },
  { name: 'KeyGenerator', label: computed(() => t('nav.keyGenerator')), href: '/generator', icon: PlusIcon },
  { name: 'ConfigEditor', label: computed(() => t('nav.configEditor')), href: '/config', icon: DocumentTextIcon },
  { name: 'Settings', label: computed(() => t('nav.settings')), href: '/settings', icon: Cog6ToothIcon },
]

// 处理退出登录
const handleLogout = async () => {
  try {
    authStore.logout()
    await router.push({ name: 'Auth', query: { mode: 'login' } })
  } catch (error) {
    console.error('退出登录失败:', error)
  }
}

// 生命周期钩子
onMounted(() => {
  checkMobile()
  window.addEventListener('resize', checkMobile)
})

onUnmounted(() => {
  window.removeEventListener('resize', checkMobile)
})
</script>