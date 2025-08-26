<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 导航栏 -->
    <nav class="bg-white shadow-sm border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <router-link to="/" class="text-blue-600 hover:text-blue-800 mr-4">
              <ArrowLeftIcon class="h-5 w-5" />
            </router-link>
            <h1 class="text-xl font-semibold text-gray-900">设置</h1>
          </div>
        </div>
      </div>
    </nav>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="space-y-6">
        <!-- 设置内容 -->
        <div class="bg-white rounded-lg shadow-sm p-6">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">应用设置</h2>
          <p class="text-gray-600">设置功能正在开发中...</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ArrowLeftIcon } from '@heroicons/vue/24/outline'

// 设置页面
</script>