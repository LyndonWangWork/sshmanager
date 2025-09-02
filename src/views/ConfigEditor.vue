<template>
  <div class="container mx-auto px-4 py-8">
    <!-- 页面标题和操作按钮 -->
    <div class="mb-8 flex justify-between items-center">
      <h1 class="text-2xl font-semibold text-gray-900">{{ $t('configEditor.title') }}</h1>
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
                <button @click.stop="deleteHost(index)" class="p-1 text-gray-400 hover:text-red-600 transition-colors">
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
              <div v-if="selectedHostIndex >= 0" class="space-y-4">
                <!-- Host 模式 -->
                <BaseInput v-model="selectedHost.host_pattern" :label="$t('configEditor.hostConfig.hostPattern')" required
                  :placeholder="$t('configEditor.hostConfig.hostPatternPlaceholder')"
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
                      class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
                      <option value="">{{ $t('configEditor.hostConfig.selectKey') }}</option>
                      <option v-for="key in keyStore.keys" :key="key.id" :value="`~/.ssh/${key.name}`">
                        {{ key.name }} ({{ key.key_type.toUpperCase() }})
                      </option>
                    </select>
                    <BaseButton size="sm" variant="secondary" @click="browseIdentityFile">
                      <FolderOpenIcon class="h-4 w-4" />
                    </BaseButton>
                  </div>
                </div>

                <!-- 其他选项 -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    {{ $t('configEditor.hostConfig.otherOptions') }}
                  </label>
                  <div class="space-y-2">
                    <div v-for="(option, index) in otherOptionsArray" :key="index" class="flex items-center space-x-2">
                      <input v-model="option.key" @input="updateOptionKey(index, ($event.target as HTMLInputElement).value)"
                        :placeholder="$t('configEditor.hostConfig.optionName')"
                        class="w-32 min-w-0 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm" />
                      <input v-model="option.value"
                        @input="updateOptionValue(index, ($event.target as HTMLInputElement).value)"
                        :placeholder="$t('configEditor.hostConfig.optionValue')"
                        class="flex-1 min-w-0 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm" />
                      <button @click="removeOptionByIndex(index)"
                        class="flex-shrink-0 p-2 text-gray-400 hover:text-red-600 transition-colors"
                        :title="$t('configEditor.hostConfig.deleteOption')">
                        <TrashIcon class="h-4 w-4" />
                      </button>
                    </div>
                    <BaseButton size="sm" variant="secondary" @click="addOption">
                      <PlusIcon class="h-4 w-4 mr-1" />
                      {{ $t('configEditor.hostConfig.addOption') }}
                    </BaseButton>
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useKeyStore } from '@/stores/key'
import type { SshConfig, SshHostConfig } from '@/types'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import {
  ArrowLeftIcon,
  ArrowPathIcon,
  DocumentCheckIcon,
  PlusIcon,
  TrashIcon,
  ServerIcon,
  Cog6ToothIcon,
  ClipboardIcon,
  FolderOpenIcon,
  CheckIcon,
  DocumentTextIcon
} from '@heroicons/vue/24/outline'

const { t } = useI18n()
const keyStore = useKeyStore()

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

// 切换编辑模式
const toggleEditMode = () => {
  // 在切换到文本编辑模式时，更新原始配置文本
  if (!showRawEditor.value) {
    rawConfigText.value = generatedConfig.value
  }
  showRawEditor.value = !showRawEditor.value
}

// 选中的主机配置
const selectedHost = computed(() => {
  if (selectedHostIndex.value >= 0 && selectedHostIndex.value < sshConfig.hosts.length) {
    return sshConfig.hosts[selectedHostIndex.value]
  }
  return {
    host_pattern: '',
    hostname: '',
    user: '',
    port: 22,
    identity_file: '',
    other_options: {}
  } as SshHostConfig
})

// 将other_options转换为可编辑的数组格式
const otherOptionsArray = computed(() => {
  if (selectedHostIndex.value >= 0 && selectedHost.value.other_options) {
    return Object.entries(selectedHost.value.other_options).map(([key, value]) => ({
      key,
      value
    }))
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
  if (confirm(t('configEditor.hostConfig.deleteConfirm'))) {
    sshConfig.hosts.splice(index, 1)

    if (selectedHostIndex.value === index) {
      selectedHostIndex.value = -1
    } else if (selectedHostIndex.value > index) {
      selectedHostIndex.value--
    }

    hasChanges.value = true
  }
}

// 添加选项
const addOption = () => {
  if (selectedHostIndex.value >= 0) {
    selectedHost.value.other_options[''] = ''
    hasChanges.value = true
  }
}

// 移除选项
const removeOption = (key: string) => {
  if (selectedHostIndex.value >= 0) {
    delete selectedHost.value.other_options[key]
    hasChanges.value = true
  }
}

// 通过索引移除选项
const removeOptionByIndex = (index: number) => {
  if (selectedHostIndex.value >= 0) {
    const option = otherOptionsArray.value[index]
    if (option) {
      delete selectedHost.value.other_options[option.key]
      hasChanges.value = true
    }
  }
}

// 更新选项键名
const updateOptionKey = (index: number, newKey: string) => {
  if (selectedHostIndex.value >= 0) {
    const option = otherOptionsArray.value[index]
    if (option && option.key !== newKey) {
      const oldKey = option.key
      const value = selectedHost.value.other_options[oldKey]

      // 删除旧键，添加新键
      delete selectedHost.value.other_options[oldKey]
      selectedHost.value.other_options[newKey] = value
      hasChanges.value = true
    }
  }
}

// 更新选项值
const updateOptionValue = (index: number, newValue: string) => {
  if (selectedHostIndex.value >= 0) {
    const option = otherOptionsArray.value[index]
    if (option) {
      selectedHost.value.other_options[option.key] = newValue
      hasChanges.value = true
    }
  }
}

// 浏览身份文件
const browseIdentityFile = () => {
  // TODO: 实现文件浏览器
  console.log(t('configEditor.messages.featureNotImplemented'))
}

// 加载配置
const loadConfig = async () => {
  isLoading.value = true

  try {
    // TODO: 从后端加载SSH配置
    // 模拟数据
    sshConfig.hosts = [
      {
        host_pattern: 'github.com',
        hostname: 'github.com',
        user: 'git',
        port: 22,
        identity_file: '~/.ssh/id_ed25519',
        other_options: {
          'PreferredAuthentications': 'publickey'
        }
      }
    ]

    rawConfigText.value = generatedConfig.value
    hasChanges.value = false
  } catch (error) {
    console.error(t('configEditor.messages.loadError'), error)
  } finally {
    isLoading.value = false
  }
}

// 保存配置
const saveConfig = async () => {
  isLoading.value = true

  try {
    // TODO: 保存SSH配置到后端
    console.log('保存配置:', sshConfig)
    hasChanges.value = false
  } catch (error) {
    console.error(t('configEditor.messages.saveError'), error)
  } finally {
    isLoading.value = false
  }
}

// 复制配置
const copyConfig = async () => {
  try {
    await navigator.clipboard.writeText(rawConfigText.value)
    // TODO: 显示成功提示
  } catch (error) {
    console.error(t('configEditor.messages.copyError'), error)
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
})
</script>