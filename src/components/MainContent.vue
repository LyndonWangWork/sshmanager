<template>
  <!-- 主内容区 -->
  <div class="flex-1 overflow-hidden">
    <main class="p-4 sm:p-6 lg:p-8">
      <div class="max-w-7xl mx-auto"> 
          <slot /> 
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
// 主内容容器组件，提供页面过渡动画和统一的内容布局
</script>

<style scoped>
/* 现代科技感的页面切换动画 */
.page-enter-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-enter-from {
  opacity: 0;
  transform: translateX(30px) scale(0.95);
  filter: blur(4px);
}

.page-leave-to {
  opacity: 0;
  transform: translateX(-20px) scale(0.98);
  filter: blur(2px);
}

.page-enter-to,
.page-leave-from {
  opacity: 1;
  transform: translateX(0) scale(1);
  filter: blur(0);
}

/* 移动端优化 */
@media (max-width: 768px) {
  .page-enter-from {
    transform: translateY(20px) scale(0.96);
  }
  
  .page-leave-to {
    transform: translateY(-15px) scale(0.98);
  }
  
  .page-enter-to,
  .page-leave-from {
    transform: translateY(0) scale(1);
  }
}

/* 高性能动画优化 */
@media (prefers-reduced-motion: reduce) {
  .page-enter-active,
  .page-leave-active {
    transition: opacity 0.2s ease;
  }
  
  .page-enter-from,
  .page-leave-to {
    opacity: 0;
    transform: none;
    filter: none;
  }
  
  .page-enter-to,
  .page-leave-from {
    opacity: 1;
    transform: none;
    filter: none;
  }
}
</style>