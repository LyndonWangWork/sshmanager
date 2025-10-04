<template>
  <div class="container mx-auto px-4 py-8">
    <!-- 页面标题和操作按钮 -->
    <div class="mb-8 flex justify-between items-center">
      <div>
        <h1 class="text-2xl font-semibold text-gray-900">{{ $t('configEditor.title') }}</h1>
        <p class="text-xs text-gray-500 mt-1 break-all" v-if="currentConfigPath">{{ currentConfigPath }}</p>
      </div>
      <div class="flex items-center space-x-4">
        <BaseButton variant="secondary" @click="loadConfig" :disabled="isLoading">
          <ArrowPathIcon class="h-4 w-4 mr-2" />
          {{ $t('configEditor.reload') }}
        </BaseButton>
        <BaseButton @click="saveConfig" :disabled="!hasChanges || isLoading">
          <DocumentCheckIcon class="h-4 w-4 mr-2" />
          {{ $t('configEditor.saveConfig') }}
        </BaseButton>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <!-- 左侧：主机列表 -->
      <div class="lg:col-span-1">
        <div class="bg-white rounded-lg shadow-sm">
          <!-- 主机列表头部 -->
          <div class="px-6 py-4 border-b border-gray-200">
            <div class="flex items-center justify-between">
              <h2 class="text-lg font-semibold text-gray-900">{{ $t('configEditor.hostConfig.title') }}</h2>
              <BaseButton size="sm" @click="addNewHost">
                <PlusIcon class="h-4 w-4 mr-1" />
                {{ $t('configEditor.hostConfig.add') }}
              </BaseButton>
            </div>
          </div>

          <!-- 主机列表 -->
          <div class="divide-y divide-gray-200">
            <div v-for="(host, index) in sshConfig.hosts" :key="index" class="px-6 py-4 cursor-pointer hover:bg-gray-50"
              :class="selectedHostIndex === index ? 'bg-blue-50 border-r-2 border-blue-500' : ''"
              @click="selectHost(index)">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-sm font-medium text-gray-900">{{ host.host_pattern }}</h3>
                  <p class="text-xs text-gray-500 mt-1">
                    {{ host.hostname || $t('configEditor.hostConfig.hostnameHint') }}
                  </p>
                </div>
                <button @click.stop="openDeleteHostConfirm(index)"
                  class="p-1 text-gray-400 hover:text-red-600 transition-colors">
                  <TrashIcon class="h-4 w-4" />
                </button>
              </div>
            </div>

            <div v-if="sshConfig.hosts.length === 0" class="px-6 py-8 text-center text-gray-500">
              <ServerIcon class="mx-auto h-8 w-8 text-gray-400 mb-2" />
              <p class="text-sm">{{ $t('configEditor.hostConfig.noHostsMessage') }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧：配置表单或文本编辑器 -->
      <div class="lg:col-span-1">
        <div class="bg-white rounded-lg shadow-sm">
          <!-- 表单头部 -->
          <div class="px-6 py-4 border-b border-gray-200">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-semibold text-gray-900">
                {{ selectedHostIndex >= 0 ? $t('configEditor.hostConfig.title') : $t('configEditor.empty') }}
              </h3>
              <BaseButton size="sm" variant="secondary" @click="toggleEditMode">
                <DocumentTextIcon class="h-4 w-4 mr-1" />
                {{ showRawEditor ? $t('configEditor.preview.hideEditor') : $t('configEditor.preview.showEditor') }}
              </BaseButton>
            </div>
          </div>

          <div class="p-6">
            <!-- 文本编辑模式 -->
            <div v-if="showRawEditor">
              <textarea v-model="rawConfigText"
                class="w-full h-96 text-xs font-mono bg-gray-50 border rounded-lg p-4 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
                :placeholder="$t('configEditor.preview.rawPlaceholder')"></textarea>
              <div class="mt-4 flex justify-end space-x-2">
                <BaseButton size="sm" variant="secondary" @click="copyConfig">
                  <ClipboardIcon class="h-4 w-4 mr-1" />
                  {{ $t('common.copy') }}
                </BaseButton>
                <BaseButton size="sm" @click="parseRawConfig">
                  <CheckIcon class="h-4 w-4 mr-1" />
                  {{ $t('configEditor.preview.applyChanges') }}
                </BaseButton>
              </div>
            </div>

            <!-- 表单编辑模式 -->
            <div v-else>
              <div v-if="selectedHostIndex >= 0 && selectedHost" class="space-y-4">
                <!-- Host 模式 -->
                <BaseInput v-model="selectedHost.host_pattern" :label="$t('configEditor.hostConfig.hostPattern')"
                  required :placeholder="$t('configEditor.hostConfig.hostPatternPlaceholder')"
                  :hint="$t('configEditor.hostConfig.hostPatternHint')" />

                <!-- 主机名 -->
                <BaseInput v-model="selectedHost.hostname" :label="$t('configEditor.hostConfig.hostname')"
                  :placeholder="$t('configEditor.hostConfig.hostnamePlaceholder')"
                  :hint="$t('configEditor.hostConfig.hostnameHint')" />

                <!-- 用户名 -->
                <BaseInput v-model="selectedHost.user" :label="$t('configEditor.hostConfig.user')"
                  :placeholder="$t('configEditor.hostConfig.userPlaceholder')" />

                <!-- 端口 -->
                <BaseInput v-model="selectedHost.port" :label="$t('configEditor.hostConfig.port')" type="number"
                  :placeholder="$t('configEditor.hostConfig.portPlaceholder')" />

                <!-- 身份文件 -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    {{ $t('configEditor.hostConfig.identityFile') }}
                  </label>
                  <div class="flex space-x-2">
                    <select v-model="selectedHost.identity_file"
                      class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                      :class="(selectedHost.identity_file && selectedHost.identity_file.length > 0) ? 'text-gray-900' : 'text-gray-400'">
                      <option value="">{{ $t('configEditor.hostConfig.selectKey') }}</option>
                      <optgroup :label="$t('configEditor.hostConfig.identityGroups.softwareKeys')">
                        <option v-for="key in keyStore.keys" :key="`soft-${key.id}`" :value="`~/.ssh/${key.name}`">
                          {{ key.name }} ({{ key.key_type.toUpperCase() }})
                        </option>
                      </optgroup>
                      <optgroup :label="$t('configEditor.hostConfig.identityGroups.sshDirKeys')">
                        <option v-for="f in sshDirKeys" :key="`fs-${f}`" :value="`~/.ssh/${f}`">
                          {{ f }}
                        </option>
                      </optgroup>
                    </select>
                    <BaseButton size="sm" variant="secondary" @click="refreshSshDirKeys" :disabled="sshDirLoading">
                      <ArrowPathIcon class="h-4 w-4" />
                    </BaseButton>
                  </div>
                </div>

                <!-- 其他选项 -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    {{ $t('configEditor.hostConfig.otherOptions') }}
                  </label>
                  <div class="space-y-2">
                    <div v-for="(option, index) in otherOptionsArray" :key="option.key" class="space-y-1">
                      <div class="flex items-center space-x-2">
                        <div class="w-1/2 min-w-0 text-sm text-gray-700 truncate">{{ option.key }}</div>
                        <template v-if="SSH_OPTION_SPECS[option.key]?.type === 'boolean'">
                          <select :value="normalizeBool(option.value)"
                            @change="updateOptionValue(index, ($event.target as HTMLSelectElement).value)"
                            class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                            :class="normalizeBool(option.value) === '' ? 'text-gray-400' : 'text-gray-900'">
                            <option value="">{{ $t('common.select') || 'Select' }}</option>
                            <option value="yes">yes</option>
                            <option value="no">no</option>
                          </select>
                        </template>
                        <template v-else-if="SSH_OPTION_SPECS[option.key]?.type === 'enum'">
                          <select :value="normalizeEnum(option.key, option.value)"
                            @change="updateOptionValue(index, ($event.target as HTMLSelectElement).value)"
                            class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                            :class="normalizeEnum(option.key, option.value) === '' ? 'text-gray-400' : 'text-gray-900'">
                            <option value="">{{ $t('common.select') || 'Select' }}</option>
                            <option v-for="v in SSH_OPTION_SPECS[option.key].values" :key="v" :value="v">{{ v }}
                            </option>
                          </select>
                        </template>
                        <template v-else>
                          <input :value="option.value"
                            @input="updateOptionValue(index, ($event.target as HTMLInputElement).value)"
                            :placeholder="$t('configEditor.hostConfig.optionValue')"
                            class="flex-1 min-w-0 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm" />
                        </template>
                      </div>
                      <p class="text-xs text-gray-500">
                        {{ $t('configEditor.hostConfig.optionDescriptions.' + option.key) }}
                      </p>
                    </div>
                  </div>
                </div>
              </div>

              <div v-else class="text-center py-8 text-gray-500">
                <Cog6ToothIcon class="mx-auto h-8 w-8 text-gray-400 mb-2" />
                <p class="text-sm">{{ $t('configEditor.hostConfig.selectHostMessage') }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- 删除确认对话框 -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 z-50 overflow-y-auto">
      <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        <div class="fixed inset-0 transition-opacity">
          <div class="absolute inset-0 bg-gray-500 opacity-75"></div>
        </div>

        <span class="hidden sm:inline-block sm:align-middle sm:h-screen"></span>&#8203;
        <div
          class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
          <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
            <div class="sm:flex sm:items-start">
              <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                <h3 class="text-lg leading-6 font-medium text-gray-900">
                  {{ $t('configEditor.hostConfig.title') }}
                </h3>
                <div class="mt-2">
                  <p class="text-sm text-gray-500">
                    {{ $t('configEditor.hostConfig.deleteConfirm') }}
                  </p>
                </div>
              </div>
            </div>
          </div>
          <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
            <button type="button"
              class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
              @click="confirmDeleteHost">
              {{ $t('common.delete') }}
            </button>
            <button type="button"
              class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
              @click="cancelDeleteHost">
              {{ $t('common.cancel') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted, onUnmounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useKeyStore } from '@/stores/key'
import { buildFullOptionsFrom, SSH_OPTION_SPECS } from '@/utils/sshOptions'
import type { SshConfig, SshHostConfig } from '@/types'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import { useToast } from '@/composables/useToast'
import { invoke } from '@tauri-apps/api/core'
import { homeDir, join } from '@tauri-apps/api/path'
import {
  ArrowPathIcon,
  DocumentCheckIcon,
  PlusIcon,
  TrashIcon,
  ServerIcon,
  Cog6ToothIcon,
  ClipboardIcon,
  CheckIcon,
  DocumentTextIcon
} from '@heroicons/vue/24/outline'

const { t } = useI18n()
const keyStore = useKeyStore()

// 布尔与枚举值规范化，统一小写，空值返回空字符串
const normalizeBool = (val: string | undefined) => {
  const v = (val || '').trim().toLowerCase()
  if (v === 'yes' || v === 'no') return v
  return ''
}

const normalizeEnum = (key: string, val: string | undefined) => {
  const spec = (SSH_OPTION_SPECS as any)[key]
  if (!spec || !spec.values) return (val || '').trim()
  const v = (val || '').trim().toLowerCase()
  return spec.values.includes(v) ? v : ''
}

// SSH配置状态
const sshConfig = reactive<SshConfig>({
  hosts: [],
  global_settings: {}
})

const selectedHostIndex = ref(-1)
const isLoading = ref(false)
const hasChanges = ref(false)
const showRawEditor = ref(false)
const rawConfigText = ref('')
// Toast 提示状态（使用通用组件）
const { success: toastSuccess, error: toastError } = useToast()

// 当前配置文件路径
const currentConfigPath = ref('')

// 删除确认对话框状态
const showDeleteConfirm = ref(false)
const pendingDeleteIndex = ref<number | null>(null)

// ~/.ssh 目录已有密钥
const sshDirKeys = ref<string[]>([])
const sshDirLoading = ref(false)
const refreshSshDirKeys = async () => {
  try {
    sshDirLoading.value = true
    const list = await invoke<string[]>('list_identity_files', { dirPath: undefined })
    sshDirKeys.value = Array.isArray(list) ? list : []
  } catch (e) {
    console.error('列出 ~/.ssh 密钥失败', e)
    sshDirKeys.value = []
  } finally {
    sshDirLoading.value = false
  }
}

// 选中的主机配置
const selectedHost = computed(() => {
  if (selectedHostIndex.value >= 0 && selectedHostIndex.value < sshConfig.hosts.length) {
    return sshConfig.hosts[selectedHostIndex.value]
  }
  return null
})
// 深度监听配置对象，忽略加载/保存阶段，用户修改时标记为已变更
watch(
  () => [selectedHost.value, selectedHostIndex.value],
  (newVal, oldVal) => {
    if (isLoading.value || !oldVal) return
    if (newVal[1] === oldVal[1]) {
      hasChanges.value = true
      return
    }

  },
  { deep: true }
)


// 切换编辑模式
const toggleEditMode = () => {
  // 在切换到文本编辑模式时，更新原始配置文本
  if (!showRawEditor.value) {
    rawConfigText.value = generatedConfig.value
  }
  showRawEditor.value = !showRawEditor.value
}


// 在“其他选项”展示中排除已由专用字段承载的键，避免重复
const EXCLUDED_FIXED_FIELDS = new Set(['HostName', 'User', 'Port', 'IdentityFile'])

// 使用全量选项+现有值构建展示数组（无值显示为空），并排除固定字段
const otherOptionsArray = computed(() => {
  if (selectedHostIndex.value >= 0 && selectedHost.value) {
    const list = buildFullOptionsFrom(selectedHost.value.other_options)
    return list.filter((item) => !EXCLUDED_FIXED_FIELDS.has(item.key))
  }
  return []
})

// 生成的配置文件内容
const generatedConfig = computed(() => {
  let config = ''

  // 全局设置
  for (const [key, value] of Object.entries(sshConfig.global_settings)) {
    config += `${key} ${value}\n`
  }

  if (Object.keys(sshConfig.global_settings).length > 0) {
    config += '\n'
  }

  // 主机配置
  for (const host of sshConfig.hosts) {
    config += `Host ${host.host_pattern}\n`

    if (host.hostname) {
      config += `    HostName ${host.hostname}\n`
    }

    if (host.user) {
      config += `    User ${host.user}\n`
    }

    if (host.port && host.port !== 22) {
      config += `    Port ${host.port}\n`
    }

    if (host.identity_file) {
      config += `    IdentityFile ${host.identity_file}\n`
    }

    for (const [key, value] of Object.entries(host.other_options)) {
      if (key && value) {
        config += `    ${key} ${value}\n`
      }
    }

    config += '\n'
  }

  return config
})

// 选择主机
const selectHost = (index: number) => {
  selectedHostIndex.value = index
  // 切换到表单编辑模式
  showRawEditor.value = false
}

// 添加新主机
const addNewHost = () => {
  const newHost: SshHostConfig = {
    host_pattern: t('configEditor.hostConfig.newHost'),
    hostname: '',
    user: '',
    port: 22,
    identity_file: '',
    other_options: {}
  }

  sshConfig.hosts.push(newHost)
  selectedHostIndex.value = sshConfig.hosts.length - 1
  hasChanges.value = true
  // 切换到表单编辑模式
  showRawEditor.value = false
}

// 删除主机
const deleteHost = (index: number) => {
  sshConfig.hosts.splice(index, 1)

  if (selectedHostIndex.value === index) {
    selectedHostIndex.value = -1
  } else if (selectedHostIndex.value > index) {
    selectedHostIndex.value--
  }

  hasChanges.value = true
}

// 打开删除确认对话框
const openDeleteHostConfirm = (index: number) => {
  pendingDeleteIndex.value = index
  showDeleteConfirm.value = true
}

// 取消删除
const cancelDeleteHost = () => {
  showDeleteConfirm.value = false
  pendingDeleteIndex.value = null
}

// 确认删除
const confirmDeleteHost = () => {
  if (pendingDeleteIndex.value !== null) {
    deleteHost(pendingDeleteIndex.value)
  }
  cancelDeleteHost()
}

// 移除增删与改名逻辑；仅允许修改值

// 更新选项值
const updateOptionValue = (index: number, newValue: string) => {
  if (selectedHostIndex.value >= 0 && selectedHost.value) {
    const option = otherOptionsArray.value[index]
    if (option) {
      selectedHost.value.other_options[option.key] = newValue
      hasChanges.value = true
    }
  }
}

// 保留占位：浏览身份文件（暂未实现）
// 移除未使用的函数以消除告警

// 加载配置
const loadConfig = async () => {
  isLoading.value = true

  try {
    const result = await invoke<SshConfig>('read_ssh_config', { filePath: undefined })
    // 将后端返回的数据应用到本地状态
    sshConfig.hosts = result.hosts as any
    sshConfig.global_settings = result.global_settings as any

    rawConfigText.value = generatedConfig.value
    hasChanges.value = false
  } catch (error) {
    console.error(t('configEditor.messages.loadError'), error)
    toastError(`${t('configEditor.messages.loadError')} ${error}`)
  } finally {
    isLoading.value = false
  }
}

// 保存前清理空值选项
const cleanupEmptyOptions = () => {
  for (const host of sshConfig.hosts) {
    if (!host.other_options) continue
    for (const [k, v] of Object.entries(host.other_options)) {
      const key = (k || '').trim()
      const val = (v || '').trim()
      if (!key || !val) {
        delete host.other_options[k]
      }
    }
  }
}

// 检查并导出软件内密钥到 ~/.ssh 目录
const exportSoftwareKeys = async () => {
  const home = await homeDir()
  const sshDir = await join(home, '.ssh')

  // 收集所有使用的软件内密钥
  const usedKeys = new Set<string>()
  for (const host of sshConfig.hosts) {
    if (host.identity_file && host.identity_file.startsWith('~/.ssh/')) {
      const fileName = host.identity_file.replace('~/.ssh/', '')
      // 检查是否是软件内密钥（通过 keyStore.keys 查找）
      const key = keyStore.keys.find(k => k.name === fileName)
      if (key) {
        usedKeys.add(key.id)
      }
    }
  }

  // 导出每个使用的密钥
  for (const keyId of usedKeys) {
    const key = keyStore.keys.find(k => k.id === keyId)
    if (!key) continue

    const privateKeyPath = await join(sshDir, key.name)
    const publicKeyPath = await join(sshDir, `${key.name}.pub`)

    // 检查文件是否已存在
    const privateExists = await invoke<boolean>('check_file_exists', { filePath: privateKeyPath })
    const publicExists = await invoke<boolean>('check_file_exists', { filePath: publicKeyPath })

    if (privateExists || publicExists) {
      const existingFiles = []
      if (privateExists) existingFiles.push(key.name)
      if (publicExists) existingFiles.push(`${key.name}.pub`)

      toastError(`${t('configEditor.messages.fileExists')}: ${existingFiles.join(', ')}`)
      throw new Error(`文件已存在: ${existingFiles.join(', ')}`)
    }

    // 导出密钥
    await invoke('export_key', { keyId, exportPath: privateKeyPath })
  }
}

// 保存配置
const saveConfig = async () => {
  isLoading.value = true

  try {
    if (!showRawEditor.value) {
      cleanupEmptyOptions()
      // 在保存前导出软件内密钥
      await exportSoftwareKeys()
    }
    const content = showRawEditor.value ? rawConfigText.value : generatedConfig.value
    // file_path 可留空使用默认 ~/.ssh/config；保留策略可从应用配置获取，这里先用 10
    await invoke('save_ssh_config', { content, filePath: undefined, retention: 10 })
    hasChanges.value = false
    toastSuccess(t('configEditor.messages.saveSuccess'))
  } catch (error) {
    console.error(t('configEditor.messages.saveError'), error)
    toastError(`${t('configEditor.messages.saveError')} ${error}`)
  } finally {
    isLoading.value = false
  }
}

// 复制配置
const copyConfig = async () => {
  try {
    await navigator.clipboard.writeText(rawConfigText.value)
    toastSuccess(t('configEditor.messages.copySuccess'))
  } catch (error) {
    console.error(t('configEditor.messages.copyError'), error)
    toastError(`${t('configEditor.messages.copyError')} ${error}`)
  }
}

// 解析原始配置
const parseRawConfig = () => {
  // TODO: 实现原始配置解析
  console.log(t('configEditor.messages.featureNotImplemented'))
  // 切换回表单编辑模式
  showRawEditor.value = false
}

// 页面初始化
onMounted(async () => {
  await keyStore.loadKeys()
  await loadConfig()
  await refreshSshDirKeys()
  try {
    const home = await homeDir()
    currentConfigPath.value = await join(home, '.ssh', 'config')
  } catch (e) {
    console.log(e)
    currentConfigPath.value = ''
  }
})

onUnmounted(() => { })
</script>