<template>
  <!-- 现代导航栏 -->
  <nav class="nav-glass sticky top-0 z-50">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-16">
        <div class="flex items-center">
          <!-- 移动端菜单按钮 -->
          <button
            @click="toggleMobileMenu"
            class="md:hidden p-2 rounded-lg text-tech-600 hover:bg-white/60 transition-colors mr-3"
          >
            <Bars3Icon class="h-6 w-6" />
          </button>
          
          <div class="flex items-center space-x-3">
            <div class="p-2 rounded-xl bg-gradient-to-br from-primary-500 to-primary-600 shadow-glow">
              <KeyIcon class="h-6 w-6 text-white" />
            </div>
            <h1 class="text-xl font-semibold text-gradient hidden sm:block">{{ $t('dashboard.title') }}</h1>
            <h1 class="text-lg font-semibold text-gradient sm:hidden">SSH</h1>
          </div>
        </div>
        <div class="flex items-center space-x-2 sm:space-x-4">
          <LanguageSelector variant="compact" class="hidden sm:block" />
          <LanguageSelector variant="icon" class="sm:hidden" />
          <BaseButton variant="secondary" @click="$emit('logout')" size="sm" class="sm:size-md">
            <span class="hidden sm:inline">{{ $t('nav.logout') }}</span>
            <ArrowRightOnRectangleIcon class="h-4 w-4 sm:hidden" />
          </BaseButton>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { inject } from 'vue'
import { useI18n } from 'vue-i18n'
import BaseButton from '@/components/BaseButton.vue'
import LanguageSelector from '@/components/LanguageSelector.vue'
import { 
  KeyIcon, 
  Bars3Icon,
  ArrowRightOnRectangleIcon 
} from '@heroicons/vue/24/outline'

const { t } = useI18n()

// 注入移动端菜单控制函数
const toggleMobileMenu = inject('toggleMobileMenu', () => {})

// 定义事件
defineEmits<{
  logout: []
}>()
</script>