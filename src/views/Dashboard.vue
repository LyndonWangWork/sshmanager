<template>
  <div class="min-h-screen bg-transparent">
    <!-- 现代导航栏 -->
    <nav class="nav-glass sticky top-0 z-50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-xl bg-gradient-to-br from-primary-500 to-primary-600 shadow-glow">
                <KeyIcon class="h-6 w-6 text-white" />
              </div>
              <h1 class="text-xl font-semibold text-gradient">{{ $t('dashboard.title') }}</h1>
            </div>
          </div>
          <div class="flex items-center space-x-4">
            <LanguageSelector variant="compact" />
            <BaseButton variant="secondary" @click="handleLogout">
              {{ $t('nav.logout') }}
            </BaseButton>
          </div>
        </div>
      </div>
    </nav>

    <div class="flex">
      <!-- 现代侧边栏 -->
      <div class="w-64 sidebar min-h-screen">
        <nav class="mt-8 px-4">
          <div class="space-y-2">
            <router-link
              v-for="item in navigation"
              :key="item.name"
              :to="item.href"
              :class="[
                $route.name === item.name
                  ? 'bg-gradient-to-r from-primary-500 to-primary-600 text-white shadow-glow'
                  : 'text-tech-600 hover:bg-white/60 hover:text-primary-600',
                'group flex items-center px-4 py-3 text-sm font-medium rounded-xl transition-all duration-300 hover:scale-105'
              ]"
            >
              <component
                :is="item.icon"
                :class="[
                  $route.name === item.name ? 'text-white' : 'text-tech-400 group-hover:text-primary-500',
                  'mr-3 h-5 w-5 transition-colors duration-300'
                ]"
                aria-hidden="true"
              />
              {{ item.label.value }}
            </router-link>
          </div>
        </nav>
      </div>

      <!-- 主内容区 -->
      <div class="flex-1 overflow-hidden">
        <main class="p-8">
          <div class="max-w-7xl mx-auto">
            <!-- 欢迎标题 -->
            <div class="mb-10 animate-fade-in">
              <h2 class="text-3xl font-bold text-gradient mb-2">{{ $t('nav.dashboard') }}</h2>
              <p class="text-tech-600 text-lg">
                {{ $t('dashboard.subtitle') }}
              </p>
            </div>

            <!-- 统计卡片 -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
              <div class="stat-card animate-slide-up" style="animation-delay: 0.1s">
                <div class="flex items-center">
                  <div class="p-4 rounded-2xl bg-gradient-to-br from-primary-100 to-primary-200 mr-4">
                    <KeyIcon class="h-8 w-8 text-primary-600 icon-glow" aria-hidden="true" />
                  </div>
                  <div class="flex-1">
                    <dt class="text-sm font-medium text-tech-600 mb-1">
                      {{ $t('dashboard.stats.totalKeys') }}
                    </dt>
                    <dd class="text-3xl font-bold text-tech-900">
                      {{ keyStore.keys.length }}
                    </dd>
                  </div>
                </div>
              </div>

              <div class="stat-card animate-slide-up" style="animation-delay: 0.2s">
                <div class="flex items-center">
                  <div class="p-4 rounded-2xl bg-gradient-to-br from-success-100 to-success-200 mr-4">
                    <CogIcon class="h-8 w-8 text-success-600 icon-glow" aria-hidden="true" />
                  </div>
                  <div class="flex-1">
                    <dt class="text-sm font-medium text-tech-600 mb-1">
                      {{ $t('dashboard.stats.sshConfig') }}
                    </dt>
                    <dd class="text-3xl font-bold text-tech-900">
                      {{ $t('dashboard.stats.loaded') }}
                    </dd>
                  </div>
                </div>
              </div>

              <div class="stat-card animate-slide-up" style="animation-delay: 0.3s">
                <div class="flex items-center">
                  <div class="p-4 rounded-2xl bg-gradient-to-br from-accent-100 to-accent-200 mr-4">
                    <CheckCircleIcon class="h-8 w-8 text-accent-600 icon-glow" aria-hidden="true" />
                  </div>
                  <div class="flex-1">
                    <dt class="text-sm font-medium text-tech-600 mb-1">
                      {{ $t('dashboard.stats.status') }}
                    </dt>
                    <dd class="text-3xl font-bold text-tech-900">
                      {{ $t('dashboard.stats.normal') }}
                    </dd>
                  </div>
                </div>
              </div>
            </div>

            <!-- 快速操作 -->
            <div class="card animate-slide-up" style="animation-delay: 0.4s">
              <div class="card-body">
                <h3 class="text-xl font-semibold text-tech-900 mb-6 flex items-center">
                  <div class="w-1 h-6 bg-gradient-to-b from-primary-500 to-primary-600 rounded-full mr-3"></div>
                  {{ $t('dashboard.quickActions.title') }}
                </h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <BaseButton
                    @click="$router.push({ name: 'KeyGenerator' })"
                    class="justify-center h-14"
                    size="lg"
                  >
                    <PlusIcon class="w-5 h-5 mr-2" />
                    {{ $t('dashboard.quickActions.generateKey') }}
                  </BaseButton>
                  <BaseButton
                    variant="secondary"
                    @click="$router.push({ name: 'ConfigEditor' })"
                    class="justify-center h-14"
                    size="lg"
                  >
                    <DocumentTextIcon class="w-5 h-5 mr-2" />
                    {{ $t('dashboard.quickActions.editConfig') }}
                  </BaseButton>
                </div>
              </div>
            </div>
          </div>
        </main>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { useKeyStore } from '@/stores/key'
import BaseButton from '@/components/BaseButton.vue'
import LanguageSelector from '@/components/LanguageSelector.vue'
import {
  KeyIcon,
  CogIcon,
  CheckCircleIcon,
  HomeIcon,
  PlusIcon,
  DocumentTextIcon,
  Cog6ToothIcon,
} from '@heroicons/vue/24/outline'

const router = useRouter()
const { t } = useI18n()
const authStore = useAuthStore()
const keyStore = useKeyStore()

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

onMounted(async () => {
  try {
    await keyStore.loadKeys()
  } catch (error) {
    console.error('加载密钥失败:', error)
  }
})
</script>