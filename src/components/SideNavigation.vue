<template>
  <!-- 现代侧边栏 -->
  <div 
    :class="[
      'sidebar transition-all duration-300 ease-in-out',
      'w-fit',
      'min-h-screen',
      'md:relative absolute md:translate-x-0',
      isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0',
      'z-40 md:z-auto'
    ]"
  >
    <!-- 移动端蒙版 -->
    <div 
      v-if="isMobileMenuOpen" 
      class="fixed inset-0 bg-black bg-opacity-50 md:hidden" 
      @click="closeMobileMenu"
    ></div>
    
    <nav class="mt-8 px-4 relative z-50 w-fit">
      <!-- 折叠按钮 -->
      <div class="flex justify-end mb-4 md:hidden">
        <button
          @click="closeMobileMenu"
          class="p-2 rounded-lg text-tech-600 hover:bg-white/60 transition-colors"
        >
          <XMarkIcon class="h-5 w-5" />
        </button>
      </div>
      
      <!-- 折叠按钮（桌面端） -->
      <div class="hidden md:flex justify-center mb-4">
        <button
          @click="toggleCollapse"
          :class="[
            'p-2 rounded-lg text-tech-600 hover:bg-white/60 transition-colors',
            isCollapsed ? 'w-12 h-12 flex items-center justify-center' : ''
          ]"
        >
          <ChevronLeftIcon v-if="!isCollapsed" class="h-5 w-5" />
          <ChevronRightIcon v-else class="h-5 w-5" />
        </button>
      </div>
      
      <div class="space-y-2 w-fit">
        <router-link
          v-for="item in navigation"
          :key="item.name"
          :to="item.href"
          :class="[
            currentRoute === item.name
              ? 'bg-gradient-to-r from-primary-500 to-primary-600 text-white shadow-glow'
              : 'text-tech-600 hover:bg-white/60 hover:text-primary-600',
            'group flex items-center text-sm font-medium rounded-xl transition-all duration-300 hover:scale-105',
            isCollapsed ? 'w-12 h-12 p-3 justify-center flex-shrink-0' : 'px-4 py-3'
          ]"
          :title="isCollapsed ? item.label.value : ''"
        >
          <component
            :is="item.icon"
            :class="[
              currentRoute === item.name ? 'text-white' : 'text-tech-400 group-hover:text-primary-500',
              isCollapsed ? 'h-6 w-6' : 'mr-3 h-5 w-5',
              'transition-colors duration-300 flex-shrink-0'
            ]"
            aria-hidden="true"
          />
          <span 
            v-show="!isCollapsed" 
            class="transition-all duration-300 overflow-hidden whitespace-nowrap"
          >
            {{ item.label.value }}
          </span>
        </router-link>
      </div>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { inject, computed, type Component, type ComputedRef } from 'vue'
import {
  XMarkIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
} from '@heroicons/vue/24/outline'

// 定义导航项类型
interface NavigationItem {
  name: string
  label: ComputedRef<string>
  href: string
  icon: Component
}

// 定义属性
defineProps<{
  navigation: NavigationItem[]
  currentRoute: string | symbol | undefined
}>()

// 注入布局状态
const isCollapsed = inject('sidebarCollapsed', false)
const isMobileMenuOpen = inject('mobileMenuOpen', false)
const toggleCollapse = inject('toggleSidebarCollapse', () => {})
const closeMobileMenu = inject('closeMobileMenu', () => {})
</script>