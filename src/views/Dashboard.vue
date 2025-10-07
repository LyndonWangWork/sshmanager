<template>
  <div class="container mx-auto px-4 py-8">
    <!-- 欢迎标题 -->
    <div class="mb-10 animate-fade-in">
      <h2 class="text-3xl font-bold text-gradient mb-2">{{ $t('nav.dashboard') }}</h2>
      <p class="text-tech-600 text-lg">
        {{ $t('dashboard.subtitle') }}
      </p>
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
      <div class="stat-card animate-slide-up cursor-pointer hover:shadow-lg hover:scale-105 transition-all duration-200"
        style="animation-delay: 0.1s" @click="goToKeyManager">
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

      <div class="stat-card animate-slide-up cursor-pointer hover:shadow-lg hover:scale-105 transition-all duration-200"
        style="animation-delay: 0.2s" @click="goToConfigEditor">
        <div class="flex items-center">
          <div class="p-4 rounded-2xl bg-gradient-to-br from-success-100 to-success-200 mr-4">
            <CogIcon class="h-8 w-8 text-success-600 icon-glow" aria-hidden="true" />
          </div>
          <div class="flex-1">
            <dt class="text-sm font-medium text-tech-600 mb-1">
              {{ $t('dashboard.stats.sshConfig') }}
            </dt>
            <dd class="text-3xl font-bold text-tech-900">
              {{ sshHostCount }}{{ $t('dashboard.stats.hosts') }}
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
          <BaseButton @click="$router.push({ name: 'KeyGenerator' })" class="justify-center h-14" size="lg">
            <PlusIcon class="w-5 h-5 mr-2" />
            {{ $t('dashboard.quickActions.generateKey') }}
          </BaseButton>
          <BaseButton variant="secondary" @click="$router.push({ name: 'ConfigEditor' })" class="justify-center h-14"
            size="lg">
            <DocumentTextIcon class="w-5 h-5 mr-2" />
            {{ $t('dashboard.quickActions.editConfig') }}
          </BaseButton>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useKeyStore } from '@/stores/key'
import BaseButton from '@/components/BaseButton.vue'
import { invoke } from '@tauri-apps/api/core'
import type { SshConfig } from '@/types'
import {
  KeyIcon,
  CogIcon,
  PlusIcon,
  DocumentTextIcon,
} from '@heroicons/vue/24/outline'

const router = useRouter()
const keyStore = useKeyStore()
const sshHostCount = ref(0)

// 跳转到密钥管理页面
const goToKeyManager = () => {
  router.push({ name: 'KeyManager' })
}

// 跳转到配置编辑页面
const goToConfigEditor = () => {
  router.push({ name: 'ConfigEditor' })
}

onMounted(async () => {
  try {
    await keyStore.loadKeys()

    // 加载SSH配置获取主机数量
    try {
      const sshConfig = await invoke<SshConfig>('read_ssh_config', { filePath: undefined })
      sshHostCount.value = sshConfig.hosts.length
    } catch (sshError) {
      console.error('加载SSH配置失败:', sshError)
      sshHostCount.value = 0
    }
  } catch (error) {
    console.error('加载密钥失败:', error)
  }
})
</script>