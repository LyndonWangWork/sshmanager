<template>
  <div 
    class="group card hover:shadow-elevation-3 hover:scale-[1.02] transition-all duration-300 p-6 animate-fade-in"
    :class="{ 'ring-2 ring-primary-500': isSelected }"
  >
    <!-- 密钥头部信息 -->
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center space-x-3">
        <div class="p-3 rounded-xl bg-gradient-to-br from-primary-100 to-primary-200 group-hover:shadow-glow transition-all duration-300">
          <KeyIcon class="h-6 w-6 text-primary-600 icon-glow" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-tech-900 group-hover:text-primary-600 transition-colors duration-300">
            {{ keyData.name }}
          </h3>
          <div class="flex items-center space-x-2 text-sm text-tech-500">
            <span class="px-3 py-1 bg-gradient-to-r from-primary-100 to-primary-200 text-primary-700 rounded-full text-xs font-semibold uppercase tracking-wide">
              {{ keyData.key_type }}
            </span>
            <span class="px-2 py-1 bg-tech-100 text-tech-600 rounded-md text-xs font-medium">
              {{ keyData.key_size }} bits
            </span>
          </div>
        </div>
      </div>
      
      <!-- 操作菜单 -->
      <div class="relative">
        <button
          @click="showMenu = !showMenu"
          class="p-2 hover:bg-white/80 hover:backdrop-blur-sm rounded-full transition-all duration-300 hover:shadow-elevation-1 group-hover:bg-primary-50"
        >
          <EllipsisVerticalIcon class="h-5 w-5 text-tech-400 group-hover:text-primary-500 transition-colors" />
        </button>
        
        <!-- 下拉菜单 -->
        <div
          v-if="showMenu"
          class="absolute right-0 mt-2 w-52 bg-white/90 backdrop-blur-xl rounded-xl shadow-elevation-3 border border-white/20 z-20"
        >
            <div class="py-2">
              <button
                @click="copyPublicKey"
                class="flex items-center w-full px-4 py-3 text-sm text-tech-700 hover:bg-primary-50 hover:text-primary-700 transition-all duration-200 group/item"
              >
                <ClipboardIcon class="h-4 w-4 mr-3 group-hover/item:text-primary-500 transition-colors" />
                {{ $t('keyCard.actions.copyPublicKey') }}
              </button>
              <button
                @click="exportKey"
                class="flex items-center w-full px-4 py-3 text-sm text-tech-700 hover:bg-primary-50 hover:text-primary-700 transition-all duration-200 group/item"
              >
                <ArrowDownTrayIcon class="h-4 w-4 mr-3 group-hover/item:text-primary-500 transition-colors" />
                {{ $t('keyCard.actions.exportKey') }}
              </button>
              <button
                @click="editKey"
                class="flex items-center w-full px-4 py-3 text-sm text-tech-700 hover:bg-primary-50 hover:text-primary-700 transition-all duration-200 group/item"
              >
                <PencilIcon class="h-4 w-4 mr-3 group-hover/item:text-primary-500 transition-colors" />
                {{ $t('keyCard.actions.editInfo') }}
              </button>
              <hr class="my-2 border-tech-200">
              <button
                @click="deleteKey"
                class="flex items-center w-full px-4 py-3 text-sm text-error-600 hover:bg-error-50 hover:text-error-700 transition-all duration-200 group/item"
              >
                <TrashIcon class="h-4 w-4 mr-3 group-hover/item:text-error-500 transition-colors" />
                {{ $t('keyCard.actions.deleteKey') }}
              </button>
            </div>
          </div>
        </div>
    </div>
    
    <!-- 密钥详细信息 -->
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-tech-700 mb-2">{{ $t('keyCard.labels.fingerprint') }}</label>
        <div class="flex items-center space-x-2 p-3 bg-tech-50/50 rounded-xl border border-tech-200/50">
          <code class="text-xs font-mono text-tech-700 flex-1 truncate">
            {{ keyData.fingerprint }}
          </code>
          <button
            @click="copyFingerprint"
            class="p-2 hover:bg-white/80 rounded-lg transition-all duration-200 hover:shadow-elevation-1"
            :title="$t('keyCard.actions.copyFingerprint')"
          >
            <ClipboardIcon class="h-4 w-4 text-tech-400 hover:text-primary-500 transition-colors" />
          </button>
        </div>
      </div>
      
      <div>
        <label class="block text-sm font-medium text-tech-700 mb-2">{{ $t('keyCard.labels.comment') }}</label>
        <p class="text-sm text-tech-600 p-3 bg-tech-50/30 rounded-xl border border-tech-200/30">
          {{ keyData.comment || $t('keyCard.labels.noComment') }}
        </p>
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
        <div class="flex items-center space-x-2 text-tech-500">
          <div class="w-2 h-2 bg-success-400 rounded-full"></div>
          <span>{{ $t('keyCard.labels.createdTime') }}: {{ formatDate(keyData.created_at) }}</span>
        </div>
        <div v-if="keyData.last_used" class="flex items-center space-x-2 text-tech-500">
          <div class="w-2 h-2 bg-primary-400 rounded-full animate-pulse"></div>
          <span>{{ $t('keyCard.labels.lastUsed') }}: {{ formatDate(keyData.last_used) }}</span>
        </div>
      </div>
    </div>
    
    <!-- 公钥预览 -->
    <div v-if="showPublicKey" class="mt-6 pt-6 border-t border-tech-200/50">
        <label class="block text-sm font-medium text-tech-700 mb-3">{{ $t('keyCard.labels.publicKeyContent') }}</label>
        <div class="relative">
          <textarea
            :value="keyData.public_key"
            readonly
            rows="4"
            class="w-full text-xs font-mono bg-tech-50/30 border border-tech-200/50 rounded-xl p-4 resize-none focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all duration-300"
          ></textarea>
          <button
            @click="copyPublicKey"
            class="absolute top-2 right-2 p-2 bg-white/80 hover:bg-white rounded-lg transition-all duration-200 hover:shadow-elevation-1"
            :title="$t('keyCard.actions.copyPublicKey')"
          >
            <ClipboardIcon class="h-4 w-4 text-tech-400 hover:text-primary-500 transition-colors" />
          </button>
        </div>
      </div>
    
    <!-- 展开/收起按钮 -->
    <button
      @click="showPublicKey = !showPublicKey"
      class="mt-4 px-4 py-2 text-sm text-primary-600 hover:text-primary-700 hover:bg-primary-50 rounded-lg font-medium transition-all duration-200 flex items-center space-x-2"
    >
      <span>{{ showPublicKey ? $t('keyCard.labels.hidePublicKey') : $t('keyCard.labels.showPublicKey') }}</span>
      <svg 
        class="w-4 h-4 transition-transform duration-200" 
        :class="{ 'rotate-180': showPublicKey }"
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { SshKeyPair } from '@/types'
import {
  KeyIcon,
  EllipsisVerticalIcon,
  ClipboardIcon,
  ArrowDownTrayIcon,
  PencilIcon,
  TrashIcon,
} from '@heroicons/vue/24/outline'

interface Props {
  keyData: SshKeyPair
  isSelected?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isSelected: false,
})

const emit = defineEmits<{
  delete: [keyId: string]
  edit: [keyData: SshKeyPair]
  export: [keyId: string]
}>()

const { t } = useI18n()
const showMenu = ref(false)
const showPublicKey = ref(false)

// 格式化日期
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}

// 复制公钥到剪贴板
const copyPublicKey = async () => {
  try {
    await navigator.clipboard.writeText(props.keyData.public_key)
    showMenu.value = false
    // TODO: 显示成功提示
  } catch (error) {
    console.error('复制失败:', error)
  }
}

// 复制指纹到剪贴板
const copyFingerprint = async () => {
  try {
    await navigator.clipboard.writeText(props.keyData.fingerprint)
    // TODO: 显示成功提示
  } catch (error) {
    console.error('复制失败:', error)
  }
}

// 导出密钥
const exportKey = () => {
  emit('export', props.keyData.id)
  showMenu.value = false
}

// 编辑密钥信息
const editKey = () => {
  emit('edit', props.keyData)
  showMenu.value = false
}

// 删除密钥
const deleteKey = () => {
  if (confirm(t('keyCard.confirmDelete', { name: props.keyData.name }))) {
    emit('delete', props.keyData.id)
  }
  showMenu.value = false
}

// 点击外部关闭菜单
const handleClickOutside = (event: MouseEvent) => {
  if (showMenu.value && !(event.target as Element)?.closest('.relative')) {
    showMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>