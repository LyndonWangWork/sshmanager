<template>
  <div class="min-h-screen bg-transparent">
    <!-- 导航栏 -->
    <nav class="bg-white shadow-sm border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <router-link to="/" class="text-blue-600 hover:text-blue-800 mr-4">
              <ArrowLeftIcon class="h-5 w-5" />
            </router-link>
            <h1 class="text-xl font-semibold text-gray-900">SSH配置编辑</h1>
          </div>
          <div class="flex items-center space-x-4">
            <BaseButton variant="secondary" @click="loadConfig" :disabled="isLoading">
              <ArrowPathIcon class="h-4 w-4 mr-2" />
              重新加载
            </BaseButton>
            <BaseButton @click="saveConfig" :disabled="!hasChanges || isLoading">
              <DocumentCheckIcon class="h-4 w-4 mr-2" />
              保存配置
            </BaseButton>
          </div>
        </div>
      </div>
    </nav>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- 左侧：主机列表 -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-lg shadow-sm">
            <!-- 主机列表头部 -->
            <div class="px-6 py-4 border-b border-gray-200">
              <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold text-gray-900">主机配置</h2>
                <BaseButton size="sm" @click="addNewHost">
                  <PlusIcon class="h-4 w-4 mr-1" />
                  添加
                </BaseButton>
              </div>
            </div>
            
            <!-- 主机列表 -->
            <div class="divide-y divide-gray-200">
              <div
                v-for="(host, index) in sshConfig.hosts"
                :key="index"
                class="px-6 py-4 cursor-pointer hover:bg-gray-50"
                :class="selectedHostIndex === index ? 'bg-blue-50 border-r-2 border-blue-500' : ''"
                @click="selectHost(index)"
              >
                <div class="flex items-center justify-between">
                  <div>
                    <h3 class="text-sm font-medium text-gray-900">{{ host.host_pattern }}</h3>
                    <p class="text-xs text-gray-500 mt-1">
                      {{ host.hostname || '未设置主机名' }}
                    </p>
                  </div>
                  <button
                    @click.stop="deleteHost(index)"
                    class="p-1 text-gray-400 hover:text-red-600 transition-colors"
                  >
                    <TrashIcon class="h-4 w-4" />
                  </button>
                </div>
              </div>
              
              <div v-if="sshConfig.hosts.length === 0" class="px-6 py-8 text-center text-gray-500">
                <ServerIcon class="mx-auto h-8 w-8 text-gray-400 mb-2" />
                <p class="text-sm">暂无主机配置</p>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 中间：配置表单 -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-lg shadow-sm p-6">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">
              {{ selectedHostIndex >= 0 ? '编辑主机配置' : '选择主机进行编辑' }}
            </h3>
            
            <div v-if="selectedHostIndex >= 0" class="space-y-4">
              <!-- Host 模式 -->
              <BaseInput
                v-model="selectedHost.host_pattern"
                label="Host 模式"
                required
                placeholder="例如：github.com 或 server-*"
                hint="支持通配符，如 * 和 ?"
              />
              
              <!-- 主机名 -->
              <BaseInput
                v-model="selectedHost.hostname"
                label="主机名"
                placeholder="例如：192.168.1.100 或 server.example.com"
                hint="实际连接的主机地址"
              />
              
              <!-- 用户名 -->
              <BaseInput
                v-model="selectedHost.user"
                label="用户名"
                placeholder="例如：root 或 ubuntu"
              />
              
              <!-- 端口 -->
              <BaseInput
                v-model="selectedHost.port"
                label="端口"
                type="number"
                placeholder="默认 22"
              />
              
              <!-- 身份文件 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">
                  身份文件 (私钥)
                </label>
                <div class="flex space-x-2">
                  <select
                    v-model="selectedHost.identity_file"
                    class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="">选择密钥</option>
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
                  其他选项
                </label>
                <div class="space-y-2">
                  <div
                    v-for="(option, index) in otherOptionsArray"
                    :key="index"
                    class="flex items-center space-x-2"
                  >
                    <input
                      v-model="option.key"
                      @input="updateOptionKey(index, ($event.target as HTMLInputElement).value)"
                      placeholder="选项名"
                      class="w-32 min-w-0 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                    />
                    <input
                      v-model="option.value"
                      @input="updateOptionValue(index, ($event.target as HTMLInputElement).value)"
                      placeholder="选项值"
                      class="flex-1 min-w-0 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                    />
                    <button
                      @click="removeOptionByIndex(index)"
                      class="flex-shrink-0 p-2 text-gray-400 hover:text-red-600 transition-colors"
                      title="删除选项"
                    >
                      <TrashIcon class="h-4 w-4" />
                    </button>
                  </div>
                  <BaseButton size="sm" variant="secondary" @click="addOption">
                    <PlusIcon class="h-4 w-4 mr-1" />
                    添加选项
                  </BaseButton>
                </div>
              </div>
            </div>
            
            <div v-else class="text-center py-8 text-gray-500">
              <Cog6ToothIcon class="mx-auto h-8 w-8 text-gray-400 mb-2" />
              <p class="text-sm">请选择一个主机进行配置</p>
            </div>
          </div>
        </div>
        
        <!-- 右侧：原始配置预览 -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-lg shadow-sm">
            <div class="px-6 py-4 border-b border-gray-200">
              <div class="flex items-center justify-between">
                <h3 class="text-lg font-semibold text-gray-900">配置预览</h3>
                <div class="flex items-center space-x-2">
                  <button
                    @click="showRawEditor = !showRawEditor"
                    class="text-sm text-blue-600 hover:text-blue-800"
                  >
                    {{ showRawEditor ? '隐藏编辑器' : '显示编辑器' }}
                  </button>
                  <BaseButton size="sm" variant="secondary" @click="copyConfig">
                    <ClipboardIcon class="h-4 w-4" />
                  </BaseButton>
                </div>
              </div>
            </div>
            
            <div class="p-6">
              <div v-if="!showRawEditor">
                <pre class="text-xs font-mono bg-gray-50 border rounded-lg p-4 overflow-auto max-h-96 whitespace-pre-wrap">{{ generatedConfig }}</pre>
              </div>
              
              <div v-else>
                <textarea
                  v-model="rawConfigText"
                  class="w-full h-96 text-xs font-mono bg-gray-50 border rounded-lg p-4 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="# SSH 配置文件\n# 请直接编辑原始配置"
                ></textarea>
                <div class="mt-2 flex justify-end">
                  <BaseButton size="sm" @click="parseRawConfig">
                    <CheckIcon class="h-4 w-4 mr-1" />
                    应用更改
                  </BaseButton>
                </div>
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
} from '@heroicons/vue/24/outline'

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
}

// 添加新主机
const addNewHost = () => {
  const newHost: SshHostConfig = {
    host_pattern: '新主机',
    hostname: '',
    user: '',
    port: 22,
    identity_file: '',
    other_options: {}
  }
  
  sshConfig.hosts.push(newHost)
  selectedHostIndex.value = sshConfig.hosts.length - 1
  hasChanges.value = true
}

// 删除主机
const deleteHost = (index: number) => {
  if (confirm('确定要删除这个主机配置吗？')) {
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
  console.log('文件浏览器功能待实现')
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
    console.error('加载配置失败:', error)
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
    console.error('保存配置失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 复制配置
const copyConfig = async () => {
  try {
    await navigator.clipboard.writeText(generatedConfig.value)
    // TODO: 显示成功提示
  } catch (error) {
    console.error('复制失败:', error)
  }
}

// 解析原始配置
const parseRawConfig = () => {
  // TODO: 实现原始配置解析
  console.log('原始配置解析功能待实现')
}

// 页面初始化
onMounted(async () => {
  await keyStore.loadKeys()
  await loadConfig()
})
</script>