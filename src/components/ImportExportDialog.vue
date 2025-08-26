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
              accept=".json,.key,.pub,.pem,application/json,text/plain"
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
            <p class="mt-1 text-xs text-gray-500">
              JSON格式：适合应用备份和恢复 | OpenSSH/PEM格式：适合系统使用
            </p>
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
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'
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
  } else if (exportFormat.value === 'openssh') {
    let preview = ''
    keysToExport.slice(0, 2).forEach((key, index) => {
      if (index > 0) preview += '\n\n'
      preview += `# 密钥: ${key.name}\n`
      preview += `# 类型: ${key.key_type}\n`
      preview += `# 公钥文件 (${key.name}.pub):\n`
      preview += key.public_key.substring(0, 50) + '...'  
      if (includePrivateKeys.value) {
        preview += `\n\n# 私钥文件 (${key.name}):\n`
        preview += key.private_key.substring(0, 50) + '...'
      }
    })
    if (keysToExport.length > 2) {
      preview += `\n\n... 还有 ${keysToExport.length - 2} 个密钥`
    }
    return preview
  } else if (exportFormat.value === 'pem') {
    let preview = ''
    keysToExport.slice(0, 2).forEach((key, index) => {
      if (index > 0) preview += '\n\n'
      preview += `# Key: ${key.name}\n`
      preview += `# Type: ${key.key_type}\n`
      preview += `# Fingerprint: ${key.fingerprint}\n`
      preview += `# Public Key:\n${key.public_key.substring(0, 50)}...\n`
      if (includePrivateKeys.value) {
        preview += `# Private Key:\n${key.private_key.substring(0, 50)}...\n`
      }
    })
    if (keysToExport.length > 2) {
      preview += `\n... 还有 ${keysToExport.length - 2} 个密钥`
    }
    return preview
  }
  
  return `将导出 ${keysToExport.length} 个密钥`
})

// 文件选择处理
const handleFileSelect = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) {
    previewKeys.value = []
    return
  }
  
  const reader = new FileReader()
  reader.onload = (e) => {
    try {
      const content = e.target?.result as string
      const fileName = file.name.toLowerCase()
      
      let keysArray: any[] = []
      
      // 根据文件扩展名和内容判断文件类型
      if (fileName.endsWith('.pub')) {
        // SSH公钥文件
        keysArray = parsePublicKeyFile(content, fileName)
      } else if (fileName.endsWith('.key') || fileName.includes('id_') || content.includes('BEGIN') && content.includes('PRIVATE KEY')) {
        // SSH私钥文件
        keysArray = parsePrivateKeyFile(content, fileName)
      } else {
        // JSON格式文件
        keysArray = parseJsonFile(content)
      }
      
      // 验证密钥对象的必要字段
      const validKeys = keysArray.filter(key => 
        key && 
        typeof key === 'object' && 
        key.id && 
        key.name && 
        key.key_type
      )
      
      if (validKeys.length === 0) {
        emit('error', '文件中没有找到有效的密钥数据')
        previewKeys.value = []
        return
      }
      
      previewKeys.value = validKeys
      
      // 成功解析后给用户反馈
      if (validKeys.length !== keysArray.length) {
        emit('error', `解析成功，但有 ${keysArray.length - validKeys.length} 个无效密钥被忽略`)
      }
    } catch (error) {
      console.error('文件解析错误:', error)
      emit('error', `文件解析失败：${error instanceof Error ? error.message : '未知错误'}`)
      previewKeys.value = []
    }
  }
  
  reader.onerror = () => {
    emit('error', '文件读取失败')
    previewKeys.value = []
  }
  
  reader.readAsText(file)
}

// 直接解析文本输入的函数
const parseTextInput = (text: string): SshKeyPair[] => {
  if (!text.trim()) return []
  
  try {
    const data = JSON.parse(text)
    let keysArray: any[] = []
    
    if (data.keys && Array.isArray(data.keys)) {
      // 标准导出格式：{ keys: [...] }
      keysArray = data.keys
    } else if (Array.isArray(data)) {
      // 直接是密钥数组格式：[...]
      keysArray = data
    } else if (data.id && data.name && data.key_type) {
      // 单个密钥对象格式：{ id, name, ... }
      keysArray = [data]
    } else {
      return []
    }
    
    // 验证密钥对象的必要字段
    return keysArray.filter(key => 
      key && 
      typeof key === 'object' && 
      key.id && 
      key.name && 
      key.key_type
    )
  } catch {
    return []
  }
}

// 解析JSON文件
const parseJsonFile = (content: string): any[] => {
  const data = JSON.parse(content)
  
  if (data.keys && Array.isArray(data.keys)) {
    // 标准导出格式：{ keys: [...] }
    return data.keys
  } else if (Array.isArray(data)) {
    // 直接是密钥数组格式：[...]
    return data
  } else if (data.id && data.name && data.key_type) {
    // 单个密钥对象格式：{ id, name, ... }
    return [data]
  } else {
    throw new Error('无效的JSON文件格式。支持的格式：标准导出文件、密钥数组或单个密钥对象')
  }
}

// 解析SSH公钥文件 (.pub)
const parsePublicKeyFile = (content: string, fileName: string): any[] => {
  const lines = content.split('\n').filter(line => line.trim())
  const keys: any[] = []
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim()
    if (!line) continue
    
    try {
      const parts = line.split(' ')
      if (parts.length < 2) continue
      
      const keyType = parts[0]
      const keyData = parts[1]
      const comment = parts.slice(2).join(' ') || ''
      
      // 推断密钥类型
      let normalizedKeyType = 'Unknown'
      if (keyType.includes('rsa')) {
        normalizedKeyType = 'Rsa'
      } else if (keyType.includes('ed25519')) {
        normalizedKeyType = 'Ed25519'
      } else if (keyType.includes('ecdsa')) {
        normalizedKeyType = 'Ecdsa'
      }
      
      // 生成密钥名称
      const baseName = fileName.replace(/\.pub$/, '').replace(/^.*[\\\/]/, '')
      const keyName = comment || baseName || `imported_key_${Date.now()}`
      
      keys.push({
        id: `imported_${Date.now()}_${i}`,
        name: keyName,
        key_type: normalizedKeyType,
        key_size: getKeySizeFromType(normalizedKeyType),
        comment: comment,
        public_key: line,
        private_key: '', // 公钥文件不包含私钥
        fingerprint: generateFingerprint(keyData),
        created_at: new Date().toISOString()
      })
    } catch (error) {
      console.warn(`解析公钥第 ${i + 1} 行失败:`, error)
    }
  }
  
  return keys
}

// 解析SSH私钥文件 (.key 或无扩展名)
const parsePrivateKeyFile = (content: string, fileName: string): any[] => {
  const keys: any[] = []
  
  try {
    // 检测私钥类型
    let keyType = 'Unknown'
    if (content.includes('BEGIN RSA PRIVATE KEY') || content.includes('BEGIN OPENSSH PRIVATE KEY') && content.includes('rsa')) {
      keyType = 'Rsa'
    } else if (content.includes('BEGIN OPENSSH PRIVATE KEY') && content.includes('ed25519')) {
      keyType = 'Ed25519'
    } else if (content.includes('BEGIN EC PRIVATE KEY') || content.includes('ecdsa')) {
      keyType = 'Ecdsa'
    }
    
    // 生成密钥名称
    const baseName = fileName.replace(/\.(key|pem)$/, '').replace(/^.*[\\\/]/, '')
    const keyName = baseName || `imported_private_key_${Date.now()}`
    
    keys.push({
      id: `imported_${Date.now()}`,
      name: keyName,
      key_type: keyType,
      key_size: getKeySizeFromType(keyType),
      comment: '',
      public_key: '', // 私钥文件通常不包含公钥，需要用户后续添加
      private_key: content,
      fingerprint: 'Unknown', // 需要公钥才能生成指纹
      created_at: new Date().toISOString()
    })
  } catch (error) {
    console.warn('解析私钥文件失败:', error)
  }
  
  return keys
}

// 根据密钥类型获取默认密钥长度
const getKeySizeFromType = (keyType: string): number => {
  switch (keyType) {
    case 'Ed25519':
      return 256
    case 'Rsa':
      return 2048 // 默认RSA长度
    case 'Ecdsa':
      return 256 // 默认ECDSA长度
    default:
      return 0
  }
}

// 生成简单的指纹 (仅作示例，实际需要更复杂的算法)
const generateFingerprint = (keyData: string): string => {
  // 这里只是一个简单的示例，实际应用中应该使用正确的SHA256算法
  const hash = keyData.substring(0, 43) // 取前43个字符
  return `SHA256:${hash}`
}

// 监听文本输入变化
watch(importText, (newText) => {
  if (importMethod.value === 'text') {
    previewKeys.value = parseTextInput(newText)
  }
})

// 监听导入方法变化，清空相关数据
watch(importMethod, (newMethod) => {
  previewKeys.value = []
  if (newMethod === 'text') {
    // 切换到文本方式时，重新解析文本
    previewKeys.value = parseTextInput(importText.value)
  } else if (newMethod === 'file') {
    // 切换到文件方式时，清除文件选择
    if (fileInput.value) {
      fileInput.value.value = ''
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
    // 准备导出的密钥ID列表
    const keyIds = exportScope.value === 'all' ? [] : props.selectedKeys.map(key => key.id)
    
    // 根据导出格式确定文件扩展名和默认文件名
    const getFileExtension = (format: string) => {
      switch (format) {
        case 'json': return 'json'
        case 'openssh': return 'txt'
        case 'pem': return 'pem'
        default: return 'txt'
      }
    }
    
    const extension = getFileExtension(exportFormat.value)
    const defaultFileName = `ssh_keys_${new Date().toISOString().split('T')[0]}.${extension}`
    
    // 使用 Tauri 文件保存对话框
    const filePath = await save({
      defaultPath: defaultFileName,
      filters: [{
        name: `${exportFormat.value.toUpperCase()} 文件`,
        extensions: [extension]
      }]
    })
    
    // 用户取消了保存
    if (!filePath) {
      isLoading.value = false
      return
    }
    
    // 对于所有格式，使用后端API处理
    if (exportFormat.value === 'json') {
      // JSON格式：在前端生成数据，然后写入文件
      let exportData: string
      
      if (exportScope.value === 'all') {
        exportData = await keyStore.exportAllKeys()
      } else {
        const data = {
          version: '1.0',
          exported_at: new Date().toISOString(),
          keys: props.selectedKeys.map(key => ({
            ...key,
            private_key: includePrivateKeys.value ? key.private_key : '[REDACTED]'
          }))
        }
        exportData = JSON.stringify(data, null, 2)
      }
      
      // 使用写文件命令保存到用户选择的位置
      await invoke<boolean>('write_file_content', {
        filePath,
        content: exportData
      })
    } else {
      // 对于其他格式，直接调用后端API
      await invoke<boolean>('export_keys_to_file', {
        keyIds,
        filePath,
        exportFormat: exportFormat.value,
        includePrivateKeys: includePrivateKeys.value
      })
    }
    
    emit('success', `密钥已成功导出到: ${filePath}`)
    emit('close')
  } catch (error) {
    emit('error', `导出失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}
// 监听显示状态变化，重置表单
watch(() => props.show, (newShow, oldShow) => {
  if (newShow && !oldShow) {
    // 对话框打开时重置状态
    if (props.mode === 'import') {
      importMethod.value = 'file'
      importText.value = ''
      previewKeys.value = []
      if (fileInput.value) {
        fileInput.value.value = ''
      }
    } else {
      exportScope.value = 'all'
      exportFormat.value = 'json'
      includePrivateKeys.value = false
    }
    isLoading.value = false
  }
})
</script>