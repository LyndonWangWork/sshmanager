<template>
  <div v-if="show" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg w-full max-w-lg mx-4">
      <!-- 对话框头部 -->
      <div class="px-6 py-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-900">
            {{ mode === 'import' ? $t('importExport.dialog.importTitle') : $t('importExport.dialog.exportTitle') }}
          </h3>
          <button @click="$emit('close')" class="p-2 hover:bg-gray-100 rounded-full transition-colors">
            <XMarkIcon class="h-5 w-5 text-gray-400" />
          </button>
        </div>
      </div>

      <!-- 对话框内容 -->
      <div class="px-6 py-4">
        <div v-if="mode === 'import'" class="space-y-4">
          <!-- 导入方式选择 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">{{ $t('importExport.import.method.title')
            }}</label>
            <div class="space-y-2">
              <label class="flex items-center">
                <input v-model="importMethod" type="radio" value="file" class="mr-3" />
                <span class="text-sm">{{ $t('importExport.import.method.file') }}</span>
              </label>
              <label class="flex items-center">
                <input v-model="importMethod" type="radio" value="text" class="mr-3" />
                <span class="text-sm">{{ $t('importExport.import.method.text') }}</span>
              </label>
            </div>
          </div>

          <!-- 文件选择 -->
          <div v-if="importMethod === 'file'">
            <label class="block text-sm font-medium text-gray-700 mb-2">{{ $t('importExport.import.file.label')
            }}</label>
            <input ref="fileInput" type="file" accept=".json,.key,.pub,.pem,application/json,text/plain"
              @change="handleFileSelect"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500" />
          </div>

          <!-- 文本输入 -->
          <div v-if="importMethod === 'text'">
            <label class="block text-sm font-medium text-gray-700 mb-2">{{ $t('importExport.import.text.label')
            }}</label>
            <textarea v-model="importText" rows="8" :placeholder="$t('importExport.import.text.placeholder')"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-sm"></textarea>
          </div>

          <!-- 导入预览 -->
          <div v-if="previewKeys.length > 0" class="bg-gray-50 rounded-lg p-4">
            <h4 class="text-sm font-medium text-gray-900 mb-2">{{ $t('importExport.import.preview.title') }} ({{
              previewKeys.length }} {{ $t('importExport.import.preview.count') }})</h4>
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
            <label class="block text-sm font-medium text-gray-700 mb-2">{{ $t('importExport.export.scope.title')
            }}</label>
            <div class="space-y-2">
              <label class="flex items-center">
                <input v-model="exportScope" type="radio" value="all" class="mr-3" />
                <span class="text-sm">{{ $t('importExport.export.scope.all') }} ({{ keyStore.keys.length }} {{
                  $t('importExport.import.preview.count') }})</span>
              </label>
              <label class="flex items-center">
                <input v-model="exportScope" type="radio" value="selected" class="mr-3" />
                <span class="text-sm">{{ $t('importExport.export.scope.selected') }}</span>
              </label>
            </div>
          </div>

          <!-- 导出加密选项：使用主密码加密导出内容（仅 JSON 可用） -->
          <div>
            <label class="flex items-center">
              <input v-model="encryptWithMaster" type="checkbox" class="mr-3" :disabled="exportFormat !== 'json'" />
              <span class="text-sm">{{ $t('importExport.export.security.encryptWithMaster') }}</span>
            </label>
            <div v-if="encryptWithMaster" class="mt-2">
              <BaseInput v-model="exportPassword" type="password" :label="$t('auth.masterPassword')"
                :placeholder="$t('auth.loginPlaceholder')" />
            </div>
          </div>

          <!-- 导出格式 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">{{ $t('importExport.export.format.title')
            }}</label>
            <select v-model="exportFormat"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
              <option value="json">{{ $t('importExport.export.format.json') }}</option>
              <option value="openssh">{{ $t('importExport.export.format.openssh') }}</option>
              <option value="pem">{{ $t('importExport.export.format.pem') }}</option>
            </select>
            <p class="mt-1 text-xs text-gray-500">
              {{ $t('importExport.export.format.description') }}
            </p>
          </div>

          <!-- 安全选项 -->
          <div>
            <label class="flex items-center">
              <input v-model="includePrivateKeys" type="checkbox" class="mr-3" />
              <span class="text-sm">{{ $t('importExport.export.security.includePrivate') }}</span>
            </label>
          </div>

          <!-- 导出预览 -->
          <div v-if="exportPreview" class="bg-gray-50 rounded-lg p-4">
            <h4 class="text-sm font-medium text-gray-900 mb-2">{{ $t('importExport.export.preview.title') }}</h4>
            <textarea :value="exportPreview" readonly rows="6"
              class="w-full text-xs font-mono bg-white border rounded p-2 resize-none"></textarea>
          </div>
        </div>
      </div>

      <!-- 对话框底部 -->
      <div class="px-6 py-4 border-t border-gray-200 flex justify-end space-x-3">
        <BaseButton variant="secondary" @click="$emit('close')">
          {{ $t('importExport.dialog.cancel') }}
        </BaseButton>
        <BaseButton v-if="mode === 'import'" @click="handleImport" :disabled="!canImport || isLoading">
          <span v-if="isLoading">{{ $t('importExport.dialog.importing') }}</span>
          <span v-else>{{ $t('importExport.dialog.importAction') }}</span>
        </BaseButton>
        <BaseButton v-else @click="handleExport" :disabled="!canExport || isLoading">
          <span v-if="isLoading">{{ $t('importExport.dialog.exporting') }}</span>
          <span v-else>{{ $t('importExport.dialog.exportAction') }}</span>
        </BaseButton>
      </div>
    </div>
  </div>
  <!-- 加密导入密码对话框 -->
  <ConfirmDialog v-model:visible="showPasswordDialog" :title="$t('auth.masterPassword')"
    :message="$t('auth.loginPlaceholder')" :confirm-button-text="$t('common.confirm')"
    :cancel-button-text="$t('common.cancel')" :require-password="true" ref="passwordDialogRef"
    @confirm="onPasswordConfirm" @cancel="onPasswordCancel" />
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'
import { useI18n } from 'vue-i18n'
import { useKeyStore } from '@/stores/key'
import type { SshKeyPair } from '@/types'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import { XMarkIcon } from '@heroicons/vue/24/outline'
import ConfirmDialog from '@/components/ConfirmDialog.vue'

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

const { t } = useI18n()
const keyStore = useKeyStore()

// 导入相关状态
const importMethod = ref<'file' | 'text'>('file')
const importText = ref('')
const previewKeys = ref<SshKeyPair[]>([])
const fileInput = ref<HTMLInputElement | null>(null)
const lastImportRaw = ref('')
const showPasswordDialog = ref(false)
const pendingEncryptedImport = ref('')
const passwordDialogRef = ref<InstanceType<typeof ConfirmDialog> | null>(null)

// 导出相关状态
const exportScope = ref<'all' | 'selected'>('all')
const exportFormat = ref<'json' | 'openssh' | 'pem'>('json')
const includePrivateKeys = ref(false)
const encryptWithMaster = ref(false)
const exportPassword = ref('')

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
      is_encrypted: false,
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
      preview += `# ${t('importExport.export.preview.keyName')} ${key.name}\n`
      preview += `# ${t('importExport.export.preview.keyType')} ${key.key_type}\n`
      preview += `# ${t('importExport.export.preview.publicKeyFile')} (${key.name}.pub):\n`
      preview += key.public_key.substring(0, 50) + '...'
      if (includePrivateKeys.value) {
        preview += `\n\n# ${t('importExport.export.preview.privateKeyFile')} (${key.name}):\n`
        preview += key.private_key.substring(0, 50) + '...'
      }
    })
    if (keysToExport.length > 2) {
      preview += `\n\n... ${t('importExport.export.preview.moreKeys')} ${keysToExport.length - 2} ${t('importExport.export.preview.keys')}`
    }
    return preview
  } else if (exportFormat.value === 'pem') {
    let preview = ''
    keysToExport.slice(0, 2).forEach((key, index) => {
      if (index > 0) preview += '\n\n'
      preview += `# Key: ${key.name}\n`
      preview += `# Type: ${key.key_type}\n`
      preview += `# ${t('importExport.export.preview.fingerprint')} ${key.fingerprint}\n`
      preview += `# Public Key:\n${key.public_key.substring(0, 50)}...\n`
      if (includePrivateKeys.value) {
        preview += `# Private Key:\n${key.private_key.substring(0, 50)}...\n`
      }
    })
    if (keysToExport.length > 2) {
      preview += `\n... ${t('importExport.export.preview.moreKeys')} ${keysToExport.length - 2} ${t('importExport.export.preview.keys')}`
    }
    return preview
  }

  return `${t('importExport.export.preview.willExport')} ${keysToExport.length} ${t('importExport.export.preview.keys')}`
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
      lastImportRaw.value = content || ''
      const fileName = file.name.toLowerCase()

      let keysArray: any[] = []

      // 优先作为 JSON 解析（避免 JSON 文本中包含 "BEGIN ... PRIVATE KEY" 被误判）
      const looksLikeJson = fileName.endsWith('.json') || /^[\s\n\r]*[\[{]/.test(content)
      if (looksLikeJson) {
        try {
          keysArray = parseJsonFile(content)
        } catch {
          // JSON 解析失败则继续按密钥文件启发式处理
          keysArray = []
        }
      }

      // 如果不是 JSON 或 JSON 解析失败，再根据扩展名/内容启发式判断
      if (keysArray.length === 0) {
        if (fileName.endsWith('.pub')) {
          // SSH公钥文件
          keysArray = parsePublicKeyFile(content, fileName)
        } else if (
          fileName.endsWith('.key') ||
          fileName.endsWith('.pem') ||
          fileName.includes('id_') ||
          (content.includes('BEGIN') && content.includes('PRIVATE KEY'))
        ) {
          // SSH私钥文件
          keysArray = parsePrivateKeyFile(content, fileName)
        } else {
          // 最后再尝试 JSON（例如无扩展名但为 JSON 的情况）
          keysArray = parseJsonFile(content)
        }
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
        emit('error', t('importExport.messages.noValidKeys'))
        previewKeys.value = []
        return
      }

      previewKeys.value = validKeys

      // 成功解析后给用户反馈
      if (validKeys.length !== keysArray.length) {
        emit('error', `${t('importExport.messages.parseWarning')} ${keysArray.length - validKeys.length} ${t('importExport.messages.invalidKeysIgnored')}`)
      }
    } catch (error) {
      console.error(t('importExport.messages.parseError'), error)
      emit('error', `${t('importExport.messages.parseError')}${error instanceof Error ? error.message : t('importExport.messages.unknownError')}`)
      previewKeys.value = []
    }
  }

  reader.onerror = () => {
    emit('error', t('importExport.messages.fileReadError'))
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
    throw new Error(t('importExport.messages.invalidFormat'))
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
      console.warn(`${t('importExport.messages.parseError')}${i + 1} ${t('common.line')}:`, error)
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
    console.warn(t('importExport.messages.parseError'), error)
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

// 当导出格式切换为非 JSON（openssh 或 pem）时，自动关闭并禁用“使用主密码加密导出”
watch(exportFormat, (newFormat) => {
  if (newFormat !== 'json') {
    encryptWithMaster.value = false
    exportPassword.value = ''
  }
})

// 处理导入
const handleImport = async () => {
  if (!canImport.value) return

  isLoading.value = true

  try {
    // 检测是否为加密导出格式
    const getRawContent = (): string => {
      if (importMethod.value === 'text') return importText.value
      return lastImportRaw.value
    }

    const looksEncrypted = (() => {
      try {
        const raw = getRawContent()
        if (!raw.trim()) return false
        const v = JSON.parse(raw)
        return v && typeof v === 'object' && v.is_encrypted === true && Array.isArray(v.keys) && v.salt
      } catch { return false }
    })()

    let importedKeys
    if (looksEncrypted) {
      // 弹出密码输入对话框
      pendingEncryptedImport.value = getRawContent()
      showPasswordDialog.value = true
      isLoading.value = false
      return
    } else {
      let keysData: string
      if (importMethod.value === 'file' && previewKeys.value.length > 0) {
        keysData = JSON.stringify(previewKeys.value)
      } else if (importMethod.value === 'text') {
        const parsed = JSON.parse(importText.value)
        keysData = JSON.stringify(parsed.keys || parsed)
      } else {
        throw new Error(t('importExport.messages.noImportData'))
      }
      importedKeys = await keyStore.importKeys(keysData)
    }
    emit('success', `${t('importExport.messages.importSuccess')} ${importedKeys.length} ${t('importExport.export.preview.keys')}`)
    emit('close')
  } catch (error) {
    emit('error', `${t('importExport.messages.importError')} ${error}`)
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
        case 'openssh': return null // OpenSSH 不使用 .txt 后缀
        case 'pem': return 'pem'
        default: return 'txt'
      }
    }

    const extension = getFileExtension(exportFormat.value)
    const baseFileName = `ssh_keys_${new Date().toISOString().split('T')[0]}`
    const defaultFileName = extension ? `${baseFileName}.${extension}` : baseFileName

    // 使用 Tauri 文件保存对话框
    const saveOptions: any = { defaultPath: defaultFileName }
    if (extension) {
      saveOptions.filters = [{
        name: `${exportFormat.value.toUpperCase()} ${t('common.file')}`,
        extensions: [extension]
      }]
    }
    const filePath = await save(saveOptions)

    // 用户取消了保存
    if (!filePath) {
      isLoading.value = false
      return
    }

    // 对于所有格式，使用后端API处理
    if (exportFormat.value === 'json') {
      // JSON格式：在前端生成数据，然后写入文件
      let exportData: string

      if (encryptWithMaster.value) {
        // 加密导出：先验证主密码是否正确
        if (!exportPassword.value || exportPassword.value.length < 1) {
          throw new Error(t('auth.errors.passwordLength'))
        }
        const valid = await invoke<boolean>('verify_master_password', {
          masterKey: exportPassword.value
        })
        if (!valid) {
          emit('error', t('auth.errors.wrongPassword'))
          isLoading.value = false
          return
        }
        // 在后端使用主密码加密公钥/私钥
        if (exportScope.value === 'all') {
          exportData = await invoke<string>('export_all_keys_encrypted', {
            masterKey: exportPassword.value,
            includePrivateKeys: includePrivateKeys.value
          })
        } else {
          const ids = props.selectedKeys.map(k => k.id)
          exportData = await invoke<string>('export_selected_keys_encrypted', {
            keyIds: ids,
            masterKey: exportPassword.value,
            includePrivateKeys: includePrivateKeys.value
          })
        }
      } else {
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

    emit('success', `${t('importExport.messages.exportSuccess')} ${filePath}`)
    emit('close')
  } catch (error) {
    emit('error', `${t('importExport.messages.exportError')} ${error}`)
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
      exportScope.value = props.selectedKeys.length > 0 ? 'selected' : 'all'
      exportFormat.value = 'json'
      includePrivateKeys.value = false
      encryptWithMaster.value = false
      exportPassword.value = ''
    }
    isLoading.value = false
  }
})

// 加密导入：确认密码
const onPasswordConfirm = async (password?: string) => {
  if (!password) {
    passwordDialogRef.value?.setPasswordError?.(t('auth.errors.passwordLength') as string)
    return
  }
  isLoading.value = true
  try {
    const importedKeys = await invoke<SshKeyPair[]>('import_encrypted_keys', {
      keysData: pendingEncryptedImport.value,
      masterKey: password
    })
    // 加密导入成功后刷新 Pinia 中的密钥列表，确保视图立即更新
    try {
      await keyStore.loadKeys()
      // 触发自动导出功能
      await keyStore.maybeAutoExport()
    } catch (e) {
      console.error('加密导入后刷新密钥列表失败:', e)
    }
    emit('success', `${t('importExport.messages.importSuccess')} ${importedKeys.length} ${t('importExport.export.preview.keys')}`)
    emit('close')
    showPasswordDialog.value = false
    pendingEncryptedImport.value = ''
  } catch (e) {
    passwordDialogRef.value?.setPasswordError?.(t('auth.errors.wrongPassword') as string)
  } finally {
    isLoading.value = false
  }
}

// 加密导入：取消密码输入
const onPasswordCancel = () => {
  showPasswordDialog.value = false
}
</script>