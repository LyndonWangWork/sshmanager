<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 导航栏 -->
    <nav class="bg-white shadow-sm border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-semibold text-gray-900">SSH密钥管理器</h1>
          </div>
          <div class="flex items-center space-x-4">
            <BaseButton variant="secondary" @click="handleLogout">
              退出登录
            </BaseButton>
          </div>
        </div>
      </div>
    </nav>

    <div class="flex">
      <!-- 侧边栏 -->
      <div class="w-64 bg-white shadow-sm h-screen">
        <nav class="mt-5 px-2">
          <div class="space-y-1">
            <router-link
              v-for="item in navigation"
              :key="item.name"
              :to="item.href"
              :class="[
                $route.name === item.name
                  ? 'bg-blue-100 text-blue-900'
                  : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900',
                'group flex items-center px-2 py-2 text-sm font-medium rounded-md'
              ]"
            >
              <component
                :is="item.icon"
                :class="[
                  $route.name === item.name ? 'text-blue-500' : 'text-gray-400 group-hover:text-gray-500',
                  'mr-3 h-5 w-5'
                ]"
                aria-hidden="true"
              />
              {{ item.label }}
            </router-link>
          </div>
        </nav>
      </div>

      <!-- 主内容区 -->
      <div class="flex-1 overflow-hidden">
        <main class="p-6">
          <div class="max-w-7xl mx-auto">
            <div class="mb-8">
              <h2 class="text-2xl font-bold text-gray-900">仪表板</h2>
              <p class="mt-1 text-sm text-gray-600">
                管理您的SSH密钥和配置
              </p>
            </div>

            <!-- 统计卡片 -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
              <div class="bg-white overflow-hidden shadow rounded-lg">
                <div class="p-5">
                  <div class="flex items-center">
                    <div class="flex-shrink-0">
                      <KeyIcon class="h-6 w-6 text-gray-400" aria-hidden="true" />
                    </div>
                    <div class="ml-5 w-0 flex-1">
                      <dl>
                        <dt class="text-sm font-medium text-gray-500 truncate">
                          密钥总数
                        </dt>
                        <dd class="text-lg font-medium text-gray-900">
                          {{ keyStore.keys.length }}
                        </dd>
                      </dl>
                    </div>
                  </div>
                </div>
              </div>

              <div class="bg-white overflow-hidden shadow rounded-lg">
                <div class="p-5">
                  <div class="flex items-center">
                    <div class="flex-shrink-0">
                      <CogIcon class="h-6 w-6 text-gray-400" aria-hidden="true" />
                    </div>
                    <div class="ml-5 w-0 flex-1">
                      <dl>
                        <dt class="text-sm font-medium text-gray-500 truncate">
                          SSH配置
                        </dt>
                        <dd class="text-lg font-medium text-gray-900">
                          已加载
                        </dd>
                      </dl>
                    </div>
                  </div>
                </div>
              </div>

              <div class="bg-white overflow-hidden shadow rounded-lg">
                <div class="p-5">
                  <div class="flex items-center">
                    <div class="flex-shrink-0">
                      <CheckCircleIcon class="h-6 w-6 text-green-400" aria-hidden="true" />
                    </div>
                    <div class="ml-5 w-0 flex-1">
                      <dl>
                        <dt class="text-sm font-medium text-gray-500 truncate">
                          状态
                        </dt>
                        <dd class="text-lg font-medium text-gray-900">
                          正常
                        </dd>
                      </dl>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 快速操作 -->
            <div class="bg-white shadow rounded-lg">
              <div class="px-4 py-5 sm:p-6">
                <h3 class="text-lg leading-6 font-medium text-gray-900 mb-4">
                  快速操作
                </h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <BaseButton
                    @click="$router.push({ name: 'KeyGenerator' })"
                    class="justify-center"
                  >
                    生成新密钥
                  </BaseButton>
                  <BaseButton
                    variant="secondary"
                    @click="$router.push({ name: 'ConfigEditor' })"
                    class="justify-center"
                  >
                    编辑SSH配置
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
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useKeyStore } from '@/stores/key'
import BaseButton from '@/components/BaseButton.vue'
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
const authStore = useAuthStore()
const keyStore = useKeyStore()

const navigation = [
  { name: 'Dashboard', label: '仪表板', href: '/', icon: HomeIcon },
  { name: 'KeyManager', label: '密钥管理', href: '/keys', icon: KeyIcon },
  { name: 'KeyGenerator', label: '生成密钥', href: '/generator', icon: PlusIcon },
  { name: 'ConfigEditor', label: '配置编辑', href: '/config', icon: DocumentTextIcon },
  { name: 'Settings', label: '设置', href: '/settings', icon: Cog6ToothIcon },
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