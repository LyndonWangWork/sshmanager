<template>
  <div v-if="show" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg w-full max-w-lg mx-4">
      <!-- 对话框头部 -->
      <div class="px-6 py-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-900">
            {{ mode === 'import' ? '导入密钥' : '导出密钥' }}
          </h3>
          <button
            @click="$emit('close')"
            class="p-2 hover:bg-gray-100 rounded-full transition-colors"
          >
            <XMarkIcon class="h-5 w-5 text-gray-400" />
          </button>
        </div>
      </div>
      
      <!-- 对话框内容 -->
      <div class="px-6 py-4">
        <div v-if="mode === 'import'" class="space-y-4">
          <!-- 导入方式选择 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">导入方式</label>
            <div class="space-y-2">
              <label class="flex items-center">
                <input
                  v-model="importMethod"
                  type="radio"
                  value="file"
                  class="mr-3"
                />
                <span class="text-sm">从文件导入</span>
              </label>
              <label class="flex items-center">
                <input
                  v-model="importMethod"
                  type="radio"
                  value="text"
                  class="mr-3"
                />
                <span class="text-sm">从文本导入</span>
              </label>
            </div>
          </div>
          
          <!-- 文件选择 -->
          <div v-if="importMethod === 'file'">
            <label class="block text-sm font-medium text-gray-700 mb-2">选择密钥文件</label>
            <input
              ref="fileInput"
              type="file"
              accept=".json,.key,.pub"
              @change="handleFileSelect"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          
          <!-- 文本输入 -->
          <div v-if="importMethod === 'text'">
            <label class="block text-sm font-medium text-gray-700 mb-2">粘贴密钥数据</label>
            <textarea
              v-model="importText"
              rows="8"
              placeholder="请粘贴导出的密钥数据 (JSON格式)"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-sm"
            ></textarea>
          </div>
          
          <!-- 导入预览 -->
          <div v-if="previewKeys.length > 0" class="bg-gray-50 rounded-lg p-4">
            <h4 class="text-sm font-medium text-gray-900 mb-2">即将导入的密钥 ({{ previewKeys.length }} 个)</h4>
            <div class="space-y-1">
              <div v-for="key in previewKeys" :key="key.id" class="text-sm text-gray-600">
                <span class="font-medium">{{ key.name }}</span>
                <span class="text-xs bg-gray-200 px-2 py-1 rounded ml-2">{{ key.key_type.toUpperCase() }}</span>
              </div>
            </div>
          </div>
        </div>
        
        <div v-else class="space-y-4">
          <!-- 导出选项 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">导出范围</label>
            <div class="space-y-2">
              <label class="flex items-center">
                <input
                  v-model="exportScope"
                  type="radio"
                  value="all"
                  class="mr-3"
                />
                <span class="text-sm">导出所有密钥 ({{ keyStore.keys.length }} 个)</span>
              </label>
              <label class="flex items-center">
                <input
                  v-model="exportScope"
                  type="radio"
                  value="selected"
                  class="mr-3"
                />
                <span class="text-sm">导出选中的密钥</span>
              </label>
            </div>
          </div>
          
          <!-- 导出格式 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">导出格式</label>
            <select
              v-model="exportFormat"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="json">JSON 格式 (.json)</option>
              <option value="openssh">OpenSSH 格式</option>
              <option value="pem">PEM 格式</option>
            </select>
          </div>
          
          <!-- 安全选项 -->
          <div>
            <label class="flex items-center">
              <input
                v-model="includePrivateKeys"
                type="checkbox"
                class="mr-3"
              />
              <span class="text-sm">包含私钥 (不推荐，仅在安全环境中使用)</span>
            </label>
          </div>
          
          <!-- 导出预览 -->
          <div v-if="exportPreview" class="bg-gray-50 rounded-lg p-4">
            <h4 class="text-sm font-medium text-gray-900 mb-2">导出预览</h4>
            <textarea
              :value="exportPreview"
              readonly
              rows="6"
              class="w-full text-xs font-mono bg-white border rounded p-2 resize-none"
            ></textarea>
          </div>
        </div>
      </div>
      
      <!-- 对话框底部 -->
      <div class="px-6 py-4 border-t border-gray-200 flex justify-end space-x-3">
        <BaseButton variant="secondary" @click="$emit('close')">
          取消
        </BaseButton>
        <BaseButton
          v-if="mode === 'import'"
          @click="handleImport"
          :disabled="!canImport || isLoading"
        >
          <span v-if="isLoading">导入中...</span>
          <span v-else>导入密钥</span>
        </BaseButton>
        <BaseButton
          v-else
          @click="handleExport"
          :disabled="!canExport || isLoading"
        >
          <span v-if="isLoading">导出中...</span>
          <span v-else>导出密钥</span>
        </BaseButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useKeyStore } from '@/stores/key'
import type { SshKeyPair } from '@/types'
import BaseButton from '@/components/BaseButton.vue'
import { XMarkIcon } from '@heroicons/vue/24/outline'

interface Props {
  show: boolean
  mode: 'import' | 'export'
  selectedKeys?: SshKeyPair[]
}

const props = withDefaults(defineProps<Props>(), {
  selectedKeys: () => []
})

const emit = defineEmits<{
  close: []
  success: [message: string]
  error: [error: string]
}>()

const keyStore = useKeyStore()

// 导入相关状态
const importMethod = ref<'file' | 'text'>('file')
const importText = ref('')
const previewKeys = ref<SshKeyPair[]>([])
const fileInput = ref<HTMLInputElement | null>(null)

// 导出相关状态
const exportScope = ref<'all' | 'selected'>('all')
const exportFormat = ref<'json' | 'openssh' | 'pem'>('json')
const includePrivateKeys = ref(false)

// 通用状态
const isLoading = ref(false)

// 计算属性
const canImport = computed(() => {
  if (importMethod.value === 'file') {
    return previewKeys.value.length > 0
  }
  return importText.value.trim().length > 0
})

const canExport = computed(() => {
  if (exportScope.value === 'selected') {
    return props.selectedKeys.length > 0
  }
  return keyStore.keys.length > 0
})

const exportPreview = computed(() => {
  if (!canExport.value) return ''
  
  const keysToExport = exportScope.value === 'all' 
    ? keyStore.keys 
    : props.selectedKeys
  
  if (exportFormat.value === 'json') {
    const data = {
      version: '1.0',
      exported_at: new Date().toISOString(),
      keys: keysToExport.map(key => ({
        ...key,
        private_key: includePrivateKeys.value ? key.private_key : '[REDACTED]'
      }))
    }
    return JSON.stringify(data, null, 2).substring(0, 500) + '...'
  }
  
  return `将导出 ${keysToExport.length} 个密钥`
})

// 文件选择处理
const handleFileSelect = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  
  const reader = new FileReader()
  reader.onload = (e) => {
    try {
      const content = e.target?.result as string
      const data = JSON.parse(content)
      
      if (data.keys && Array.isArray(data.keys)) {
        previewKeys.value = data.keys
      } else {
        emit('error', '无效的密钥文件格式')
      }
    } catch (error) {
      emit('error', '文件解析失败')
    }
  }
  reader.readAsText(file)
}

// 监听文本输入变化
watch(importText, (newText) => {
  if (importMethod.value === 'text' && newText.trim()) {
    try {
      const data = JSON.parse(newText)
      if (data.keys && Array.isArray(data.keys)) {
        previewKeys.value = data.keys
      }
    } catch {
      previewKeys.value = []
    }
  }
})

// 处理导入
const handleImport = async () => {
  if (!canImport.value) return
  
  isLoading.value = true
  
  try {
    let keysData: string
    
    if (importMethod.value === 'file' && previewKeys.value.length > 0) {
      keysData = JSON.stringify(previewKeys.value)
    } else if (importMethod.value === 'text') {
      const parsed = JSON.parse(importText.value)
      keysData = JSON.stringify(parsed.keys || parsed)
    } else {
      throw new Error('没有可导入的数据')
    }
    
    const importedKeys = await keyStore.importKeys(keysData)
    emit('success', `成功导入 ${importedKeys.length} 个密钥`)
    emit('close')
  } catch (error) {
    emit('error', `导入失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

// 处理导出
const handleExport = async () => {
  if (!canExport.value) return
  
  isLoading.value = true
  
  try {
    let exportData: string
    
    if (exportScope.value === 'all') {
      exportData = await keyStore.exportAllKeys()
    } else {
      // 导出选中的密钥
      const data = {
        version: '1.0',
        exported_at: new Date().toISOString(),
        keys: props.selectedKeys
      }
      exportData = JSON.stringify(data, null, 2)
    }
    
    // 下载文件
    const blob = new Blob([exportData], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `ssh_keys_${new Date().toISOString().split('T')[0]}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
    
    emit('success', '密钥导出成功')
    emit('close')
  } catch (error) {
    emit('error', `导出失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}
</script>