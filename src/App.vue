<template>
  <div id="app" class="min-h-screen bg-gradient-to-br from-tech-50 via-primary-50/30 to-secondary-50 relative overflow-hidden">
    <!-- 背景装饰元素 -->
    <div class="absolute inset-0 overflow-hidden pointer-events-none">
      <!-- 动态光点 -->
      <div class="absolute top-1/4 left-1/4 w-64 h-64 bg-primary-400/20 rounded-full blur-3xl animate-float"></div>
      <div class="absolute top-3/4 right-1/4 w-96 h-96 bg-accent-400/20 rounded-full blur-3xl animate-float" style="animation-delay: -2s;"></div>
      <div class="absolute bottom-1/4 left-1/3 w-48 h-48 bg-secondary-400/20 rounded-full blur-3xl animate-float" style="animation-delay: -4s;"></div>
      
      <!-- 网格背景 -->
      <div 
        class="absolute inset-0 opacity-[0.02]" 
        style="background-image: radial-gradient(circle at 1px 1px, rgba(59, 130, 246, 0.5) 1px, transparent 0); background-size: 20px 20px;"
      ></div>
    </div>
    
    <!-- 主要内容 -->
    <div class="relative z-10">
      <RouterView v-slot="{ Component }">
        <component :is="Component" v-if="Component" />
      </RouterView>
    </div>
  </div>
</template>

<script setup lang="ts">
// 主应用组件，支持页面过渡动画的路由视图
</script>

<style scoped>
/* 样式需要时可以在这里添加 */
</style>