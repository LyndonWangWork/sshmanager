<template>
  <div class="bg-white rounded-lg shadow-md border border-gray-200 p-6 hover:shadow-lg transition-shadow">
    <!-- 密钥头部信息 -->
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center space-x-3">
        <div class="p-2 rounded-lg bg-blue-100">
          <KeyIcon class="h-5 w-5 text-blue-600" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-gray-900">{{ keyData.name }}</h3>
          <div class="flex items-center space-x-2 text-sm text-gray-500">
            <span class="px-2 py-1 bg-gray-100 rounded text-xs font-medium">
              {{ keyData.key_type.toUpperCase() }}
            </span>
            <span>{{ keyData.key_size }} bits</span>
          </div>
        </div>
      </div>
      
      <!-- 操作菜单 -->
      <div class="relative">
        <button
          @click="showMenu = !showMenu"
          class="p-2 hover:bg-gray-100 rounded-full transition-colors"
        >
          <EllipsisVerticalIcon class="h-5 w-5 text-gray-400" />
        </button>
        
        <!-- 下拉菜单 -->
        <div
          v-if="showMenu"
          class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg border border-gray-200 z-10"
        >
          <div class="py-1">
            <button
              @click="copyPublicKey"
              class="flex items-center w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-50"
            >
              <ClipboardIcon class="h-4 w-4 mr-3" />
              复制公钥
            </button>
            <button
              @click="exportKey"
              class="flex items-center w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-50"
            >
              <ArrowDownTrayIcon class="h-4 w-4 mr-3" />
              导出密钥
            </button>
            <button
              @click="editKey"
              class="flex items-center w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-50"
            >
              <PencilIcon class="h-4 w-4 mr-3" />
              编辑信息
            </button>
            <hr class="my-1">
            <button
              @click="deleteKey"
              class="flex items-center w-full px-4 py-2 text-sm text-red-600 hover:bg-red-50"
            >
              <TrashIcon class="h-4 w-4 mr-3" />
              删除密钥
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 密钥详细信息 -->
    <div class="space-y-3">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">指纹</label>
        <div class="flex items-center space-x-2">
          <code class="text-xs font-mono bg-gray-100 px-2 py-1 rounded flex-1 truncate">
            {{ keyData.fingerprint }}
          </code>
          <button
            @click="copyFingerprint"
            class="p-1 hover:bg-gray-100 rounded transition-colors"
            title="复制指纹"
          >
            <ClipboardIcon class="h-4 w-4 text-gray-400" />
          </button>
        </div>
      </div>
      
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">注释</label>
        <p class="text-sm text-gray-600">{{ keyData.comment || '无注释' }}</p>
      </div>
      
      <div class="flex justify-between text-sm text-gray-500">
        <span>创建时间: {{ formatDate(keyData.created_at) }}</span>
        <span v-if="keyData.last_used">
          最后使用: {{ formatDate(keyData.last_used) }}
        </span>
      </div>
    </div>
    
    <!-- 公钥预览 -->
    <div v-if="showPublicKey" class="mt-4 pt-4 border-t border-gray-200">
      <label class="block text-sm font-medium text-gray-700 mb-2">公钥内容</label>
      <textarea
        :value="keyData.public_key"
        readonly
        rows="3"
        class="w-full text-xs font-mono bg-gray-50 border border-gray-300 rounded-md p-2 resize-none"
      ></textarea>
    </div>
    
    <!-- 展开/收起按钮 -->
    <button
      @click="showPublicKey = !showPublicKey"
      class="mt-3 text-sm text-blue-600 hover:text-blue-800 font-medium"
    >
      {{ showPublicKey ? '收起公钥' : '显示公钥' }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
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
}

const props = defineProps<Props>()

const emit = defineEmits<{
  delete: [keyId: string]
  edit: [keyData: SshKeyPair]
  export: [keyId: string]
}>()

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
  if (confirm(`确定要删除密钥 "${props.keyData.name}" 吗？此操作无法撤销。`)) {
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