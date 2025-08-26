<template>
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
    <!-- 左侧：参数配置 -->
    <div class="space-y-6">
      <!-- 基本信息 -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">密钥信息</h2>

        <form @submit.prevent="generateKey" class="space-y-4">
          <BaseInput v-model="keyParams.name" label="密钥名称" required placeholder="例如：github-work" :error="errors.name" />

          <BaseInput v-model="keyParams.comment" label="注释 (可选)" placeholder="例如：user@hostname" />
        </form>
      </div>

      <!-- 密钥类型配置 -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">密钥类型</h2>

        <div class="space-y-4">
          <!-- 密钥类型选择 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-3">选择类型</label>
            <div class="space-y-2">
              <label v-for="type in keyTypes" :key="type.value"
                class="flex items-center p-3 border rounded-lg cursor-pointer hover:bg-gray-50"
                :class="keyParams.key_type === type.value ? 'border-blue-500 bg-blue-50' : 'border-gray-200'">
                <input v-model="keyParams.key_type" :value="type.value" type="radio" class="mr-3" />
                <div>
                  <div class="flex items-center space-x-2">
                    <span class="font-medium">{{ type.name }}</span>
                    <span v-if="type.recommended" class="px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full">
                      推荐
                    </span>
                  </div>
                  <p class="text-sm text-gray-600 mt-1">{{ type.description }}</p>
                </div>
              </label>
            </div>
          </div>

          <!-- 密钥长度 -->
          <div v-if="keyParams.key_type !== 'Ed25519'">
            <label class="block text-sm font-medium text-gray-700 mb-2">密钥长度 (bits)</label>
            <select v-model="keyParams.key_size"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
              <option v-for="size in availableKeySizes" :key="size" :value="size">
                {{ size }} bits
              </option>
            </select>
            <p class="text-xs text-gray-500 mt-1">
              更高的位数提供更强的安全性，但会增加计算开销
            </p>
          </div>
        </div>
      </div>

      <!-- 高级选项 -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">高级选项</h2>

        <div class="space-y-4">
          <!-- 密码保护 -->
          <div>
            <label class="flex items-center mb-3">
              <input v-model="advancedOptions.usePassphrase" type="checkbox" class="mr-3" />
              <span class="text-sm font-medium">使用密码保护私钥（推荐）</span>
            </label>

            <div v-if="advancedOptions.usePassphrase" class="ml-6 space-y-3">
              <BaseInput v-model="keyParams.passphrase" label="密钥密码" type="password" placeholder="请输入一个强密码来保护私钥"
                :error="errors.passphrase" hint="密码长度建议不少于8位，包含字母、数字和特殊字符" />

              <BaseInput v-model="passphraseConfirm" label="确认密码" type="password" placeholder="请再次输入密码"
                :error="errors.passphraseConfirm" />

              <div class="bg-blue-50 border border-blue-200 rounded-md p-3">
                <div class="flex">
                  <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd"
                        d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                        clip-rule="evenodd" />
                    </svg>
                  </div>
                  <div class="ml-3">
                    <h3 class="text-sm font-medium text-blue-800">安全提示</h3>
                    <div class="mt-2 text-sm text-blue-700">
                      <p>密码保护可以防止私钥文件被盗用。即使文件被获取，没有密码也无法使用。</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 生成按钮 -->
      <BaseButton type="submit" :disabled="isGenerating || !isFormValid" class="w-full" @click="generateKey">
        <span v-if="isGenerating">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none"
            viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
            </path>
          </svg>
          生成中...
        </span>
        <span v-else>
          <KeyIcon class="h-5 w-5 mr-2" />
          生成密钥
        </span>
      </BaseButton>
    </div>

    <!-- 右侧：生成结果 -->
    <div class="space-y-6">
      <!-- 进度显示 -->
      <div v-if="isGenerating" class="bg-white rounded-lg shadow-sm p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">生成进度</h3>

        <div class="space-y-4">
          <div>
            <div class="flex justify-between text-sm mb-2">
              <span>{{ progressText }}</span>
              <span>{{ progress }}%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div class="bg-blue-600 h-2 rounded-full transition-all duration-300" :style="{ width: progress + '%' }">
              </div>
            </div>
          </div>

          <div class="text-sm text-gray-600">
            <p>正在生成 {{ keyParams.key_type.toUpperCase() }} 密钥...</p>
          </div>
        </div>
      </div>

      <!-- 生成结果 -->
      <div v-else-if="generatedKey" class="bg-white rounded-lg shadow-sm p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">生成成功</h3>

        <div class="space-y-4">
          <!-- 密钥信息 -->
          <div class="bg-green-50 border border-green-200 rounded-lg p-4">
            <div class="flex items-center mb-2">
              <CheckCircleIcon class="h-5 w-5 text-green-600 mr-2" />
              <span class="font-medium text-green-800">密钥生成成功</span>
            </div>
            <div class="text-sm text-green-700">
              <p><strong>名称:</strong> {{ generatedKey.name }}</p>
              <p><strong>类型:</strong> {{ generatedKey.key_type.toUpperCase() }}</p>
              <p><strong>长度:</strong> {{ generatedKey.key_size }} bits</p>
            </div>
          </div>

          <!-- 指纹 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">密钥指纹</label>
            <div class="flex items-center space-x-2">
              <code class="flex-1 text-xs font-mono bg-gray-100 px-3 py-2 rounded border">
                    {{ generatedKey.fingerprint }}
                  </code>
              <button @click="copyFingerprint" class="p-2 text-gray-400 hover:text-gray-600 transition-colors"
                title="复制指纹">
                <ClipboardIcon class="h-4 w-4" />
              </button>
            </div>
          </div>

          <!-- 公钥 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">公钥</label>
            <textarea :value="generatedKey.public_key" readonly rows="3"
              class="w-full text-xs font-mono bg-gray-50 border border-gray-300 rounded-md p-3 resize-none"></textarea>
            <div class="flex space-x-2 mt-2">
              <BaseButton size="sm" @click="copyPublicKey">
                <ClipboardIcon class="h-4 w-4 mr-1" />
                复制公钥
              </BaseButton>
              <BaseButton size="sm" variant="secondary" @click="saveToFile">
                <ArrowDownTrayIcon class="h-4 w-4 mr-1" />
                保存文件
              </BaseButton>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex space-x-3 pt-4">
            <BaseButton @click="goToKeyManager" class="flex-1">
              <EyeIcon class="h-4 w-4 mr-2" />
              查看所有密钥
            </BaseButton>
            <BaseButton variant="secondary" @click="generateAnother" class="flex-1">
              <PlusIcon class="h-4 w-4 mr-2" />
              再生成一个
            </BaseButton>
          </div>
        </div>
      </div>

      <!-- 默认状态 -->
      <div v-else class="bg-white rounded-lg shadow-sm p-6">
        <div class="text-center text-gray-500 py-8">
          <KeyIcon class="mx-auto h-12 w-12 text-gray-400 mb-4" />
          <h3 class="text-lg font-medium text-gray-900 mb-2">准备生成SSH密钥</h3>
          <p class="text-sm">设置密钥参数后点击“生成密钥”按钮</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useKeyStore } from '@/stores/key'
import type { KeyGenerationParams, SshKeyType, SshKeyPair } from '@/types'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import {
  KeyIcon,
  ArrowLeftIcon,
  CheckCircleIcon,
  ClipboardIcon,
  ArrowDownTrayIcon,
  EyeIcon,
  PlusIcon,
} from '@heroicons/vue/24/outline'

const router = useRouter()
const keyStore = useKeyStore()

// 密钥生成参数
const keyParams = reactive<KeyGenerationParams>({
  name: '',
  key_type: 'Ed25519' as SshKeyType,
  key_size: 256,
  comment: '',
  passphrase: ''
})

// 高级选项
const advancedOptions = reactive({
  usePassphrase: false
})

// 额外状态
const passphraseConfirm = ref('')

// 状态管理
const isGenerating = ref(false)
const progress = ref(0)
const progressText = ref('')
const generatedKey = ref<SshKeyPair | null>(null)
const errors = reactive({
  name: '',
  passphrase: '',
  passphraseConfirm: ''
})

// 密钥类型配置
const keyTypes = [
  {
    value: 'Ed25519' as SshKeyType,
    name: 'Ed25519',
    description: '现代的椒圆曲线算法，安全性高、性能优秀',
    recommended: true
  },
  {
    value: 'Rsa' as SshKeyType,
    name: 'RSA',
    description: '传统的非对称加密算法，广泛支持但相对较慢',
    recommended: false
  },
  {
    value: 'Ecdsa' as SshKeyType,
    name: 'ECDSA',
    description: '椒圆曲线数字签名算法，平衡了安全性和性能',
    recommended: false
  }
]

// 可用的密钥长度
const availableKeySizes = computed(() => {
  switch (keyParams.key_type) {
    case 'Rsa':
      return [2048, 4096]
    case 'Ecdsa':
      return [256, 384, 521]
    default:
      return [256]
  }
})

// 表单有效性验证
const isFormValid = computed(() => {
  const basicValid = keyParams.name.trim().length > 0

  if (!advancedOptions.usePassphrase) {
    return basicValid
  }

  const passphraseValid = keyParams.passphrase &&
    keyParams.passphrase.length >= 8 &&
    keyParams.passphrase === passphraseConfirm.value

  return basicValid && passphraseValid
})

// 密钥类型变化时更新密钥长度
const onKeyTypeChange = () => {
  keyParams.key_size = availableKeySizes.value[0]
}

// 生成密钥
const generateKey = async () => {
  // 清除错误
  errors.name = ''
  errors.passphrase = ''
  errors.passphraseConfirm = ''

  // 验证表单
  if (!keyParams.name.trim()) {
    errors.name = '请输入密钥名称'
    return
  }

  // 验证密码
  if (advancedOptions.usePassphrase) {
    if (!keyParams.passphrase || keyParams.passphrase.length < 8) {
      errors.passphrase = '密码长度不能少于8位'
      return
    }

    if (keyParams.passphrase !== passphraseConfirm.value) {
      errors.passphraseConfirm = '两次输入的密码不一致'
      return
    }
  } else {
    // 如果不使用密码，清空密码字段
    keyParams.passphrase = ''
  }

  isGenerating.value = true
  progress.value = 0
  generatedKey.value = null

  try {
    // 模拟生成进度
    const progressSteps = [
      { progress: 20, text: '初始化随机数生成器...' },
      { progress: 40, text: '生成密钥对...' },
      { progress: 70, text: '计算密钥指纹...' },
      { progress: 90, text: '格式化密钥...' },
      { progress: 100, text: '密钥生成完成' }
    ]

    for (const step of progressSteps) {
      await new Promise(resolve => setTimeout(resolve, 800))
      progress.value = step.progress
      progressText.value = step.text
    }

    // 调用后端生成密钥
    const result = await keyStore.generateKey(keyParams)
    if (result) {
      generatedKey.value = result
    }
  } catch (error) {
    console.error('生成密钥失败:', error)
    // TODO: 显示错误提示
  } finally {
    isGenerating.value = false
  }
}

// 复制公钥
const copyPublicKey = async () => {
  if (generatedKey.value) {
    try {
      await navigator.clipboard.writeText(generatedKey.value.public_key)
      // TODO: 显示成功提示
    } catch (error) {
      console.error('复制失败:', error)
    }
  }
}

// 复制指纹
const copyFingerprint = async () => {
  if (generatedKey.value) {
    try {
      await navigator.clipboard.writeText(generatedKey.value.fingerprint)
      // TODO: 显示成功提示
    } catch (error) {
      console.error('复制失败:', error)
    }
  }
}

// 保存到文件
const saveToFile = () => {
  if (generatedKey.value) {
    // TODO: 实现文件保存功能
    console.log('保存到文件功能待实现')
  }
}

// 跳转到密钥管理
const goToKeyManager = () => {
  router.push({ name: 'KeyManager' })
}

// 再生成一个密钥
const generateAnother = () => {
  generatedKey.value = null
  keyParams.name = ''
  keyParams.comment = ''
  keyParams.passphrase = ''
  passphraseConfirm.value = ''
  progress.value = 0
  progressText.value = ''

  // 清除错误信息
  errors.name = ''
  errors.passphrase = ''
  errors.passphraseConfirm = ''
}
</script>