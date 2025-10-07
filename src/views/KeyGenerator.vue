<template>
  <div class="space-y-6">
    <!-- 修复生成按钮样式问题 -->
    <div class="flex justify-center py-4">
      <BaseButton type="submit" :disabled="isGenerating || !isFormValid"
        class="relative overflow-hidden group px-8 py-3" :class="{
          'bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 text-white': !isGenerating && isFormValid,
          'bg-gray-300 text-gray-500 cursor-not-allowed': isGenerating || !isFormValid,
          'transform hover:translate-y-[-1px] hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 ease-[cubic-bezier(0.4,0,0.2,1)]': !isGenerating && isFormValid,
          'shadow-lg hover:shadow-xl': !isGenerating && isFormValid
        }" @click="generateKey">
        <!-- 光扫动画效果 -->
        <div v-if="!isGenerating && isFormValid"
          class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent transform -skew-x-12 translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-1000 ease-in-out">
        </div>

        <span v-if="isGenerating" class="flex items-center relative z-10">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none"
            viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
            </path>
          </svg>
          {{ $t('keyGenerator.generate.generating') }}
        </span>
        <span v-else class="flex items-center justify-center relative z-10">
          <KeyIcon class="h-5 w-5 mr-2" />
          {{ $t('keyGenerator.generate.button') }}
        </span>
      </BaseButton>
    </div>

    <!-- 密钥信息、密钥类型、高级选项放在同一行 -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- 左侧：基本信息 -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('keyGenerator.keyInfo.title') }}</h2>

        <form @submit.prevent="generateKey" class="space-y-4">
          <BaseInput v-model="keyParams.name" :label="$t('keyGenerator.keyInfo.name')" required
            :placeholder="$t('keyGenerator.keyInfo.namePlaceholder')" :error="errors.name" show-clear-button />

          <BaseInput v-model="keyParams.comment" :label="$t('keyGenerator.keyInfo.comment')"
            :placeholder="$t('keyGenerator.keyInfo.commentPlaceholder')" show-clear-button />
        </form>
      </div>

      <!-- 中间：密钥类型 -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('keyGenerator.keyType.title') }}</h2>

        <div class="space-y-4">
          <!-- 密钥类型选择 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-3">{{ $t('keyGenerator.keyType.selectType')
              }}</label>
            <div class="space-y-2">
              <label v-for="type in keyTypes" :key="type.value"
                class="flex items-center p-3 border rounded-lg cursor-pointer hover:bg-gray-50"
                :class="keyParams.key_type === type.value ? 'border-blue-500 bg-blue-50' : 'border-gray-200'">
                <input v-model="keyParams.key_type" :value="type.value" type="radio" class="mr-3" />
                <div>
                  <div class="flex items-center space-x-2">
                    <span class="font-medium">{{ type.name }}</span>
                    <span v-if="type.recommended" class="px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full">
                      {{ $t('keyGenerator.keyType.recommended') }}
                    </span>
                  </div>
                  <p class="text-sm text-gray-600 mt-1">{{ type.description }}</p>
                </div>
              </label>
            </div>
          </div>

          <!-- 密钥长度 -->
          <div v-if="keyParams.key_type !== 'Ed25519'">
            <label class="block text-sm font-medium text-gray-700 mb-2">{{ $t('keyGenerator.keyType.keyLength')
              }}</label>
            <select v-model="keyParams.key_size" required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
              <option v-for="size in availableKeySizes" :key="size" :value="size">
                {{ size }} bits
              </option>
            </select>
            <p class="text-xs text-gray-500 mt-1">
              {{ $t('keyGenerator.keyType.lengthHint') }}
            </p>
          </div>
        </div>
      </div>

      <!-- 右侧：高级选项 -->
      <div class="bg-white rounded-lg shadow-sm p-6">
        <h2 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('keyGenerator.advancedOptions.title') }}</h2>

        <!-- 真实密钥生成提示 -->
        <div class="bg-green-50 border border-green-200 rounded-md p-3 mb-4">
          <div class="flex">
            <div class="flex-shrink-0">
              <svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                  clip-rule="evenodd" />
              </svg>
            </div>
            <div class="ml-3">
              <h3 class="text-sm font-medium text-green-800">{{
                $t('keyGenerator.advancedOptions.realKeyGeneration.title') }}</h3>
              <div class="mt-2 text-sm text-green-700">
                <p>{{ $t('keyGenerator.advancedOptions.realKeyGeneration.content') }}</p>
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-4">
          <!-- 密码保护 -->
          <div>
            <label class="flex items-center mb-3">
              <input v-model="advancedOptions.usePassphrase" type="checkbox" class="mr-3" />
              <span class="text-sm font-medium">{{ $t('keyGenerator.advancedOptions.usePassphrase') }}</span>
            </label>

            <div v-if="advancedOptions.usePassphrase" class="ml-6 space-y-3">
              <BaseInput v-model="keyParams.passphrase" :label="$t('keyGenerator.advancedOptions.passphrase')"
                type="password" :placeholder="$t('keyGenerator.advancedOptions.passphrasePlaceholder')"
                :error="errors.passphrase" :hint="$t('keyGenerator.advancedOptions.passphraseHint')"
                show-clear-button />

              <BaseInput v-model="passphraseConfirm" :label="$t('keyGenerator.advancedOptions.confirmPassphrase')"
                type="password" :placeholder="$t('keyGenerator.advancedOptions.confirmPassphrasePlaceholder')"
                :error="errors.passphraseConfirm" show-clear-button />

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
                    <h3 class="text-sm font-medium text-blue-800">{{
                      $t('keyGenerator.advancedOptions.securityTip.title') }}</h3>
                    <div class="mt-2 text-sm text-blue-700">
                      <p>{{ $t('keyGenerator.advancedOptions.securityTip.content') }}</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 生成结果（下方占据整行） -->
    <div class="space-y-6">
      <!-- 进度显示 -->
      <div v-if="isGenerating" class="bg-white rounded-lg shadow-sm p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('keyGenerator.generate.progress.title') }}</h3>

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
        <h3 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('keyGenerator.result.title') }}</h3>

        <div class="space-y-4">
          <!-- 密钥信息 -->
          <div class="bg-green-50 border border-green-200 rounded-lg p-4">
            <div class="flex items-center mb-2">
              <CheckCircleIcon class="h-5 w-5 text-green-600 mr-2" />
              <span class="font-medium text-green-800">{{ $t('keyGenerator.result.successMessage') }}</span>
            </div>
            <div class="text-sm text-green-700">
              <p><strong>{{ $t('keyGenerator.result.name') }}:</strong> {{ generatedKey.name }}</p>
              <p><strong>{{ $t('keyGenerator.result.type') }}:</strong> {{ generatedKey.key_type.toUpperCase() }}</p>
              <p><strong>{{ $t('keyGenerator.result.length') }}:</strong> {{ generatedKey.key_size }} bits</p>
            </div>
          </div>

          <!-- 指纹 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">{{ $t('keyGenerator.result.fingerprint')
              }}</label>
            <div class="flex items-center space-x-2">
              <code class="flex-1 text-xs font-mono bg-gray-100 px-3 py-2 rounded border">
                    {{ generatedKey.fingerprint }}
                  </code>
              <button @click="copyFingerprint" class="p-2 text-gray-400 hover:text-gray-600 transition-colors"
                :title="$t('keyGenerator.result.copyFingerprint')">
                <ClipboardIcon class="h-4 w-4" />
              </button>
            </div>
          </div>

          <!-- 公钥 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">{{ $t('keyGenerator.result.publicKey')
              }}</label>
            <textarea :value="generatedKey.public_key" readonly rows="3"
              class="w-full text-xs font-mono bg-gray-50 border border-gray-300 rounded-md p-3 resize-none"></textarea>
            <div class="flex space-x-2 mt-2">
              <BaseButton size="sm" @click="copyPublicKey">
                <ClipboardIcon class="h-4 w-4 mr-1" />
                {{ $t('keyGenerator.result.copyPublicKey') }}
              </BaseButton>
              <BaseButton size="sm" variant="secondary" @click="saveToFile">
                <ArrowDownTrayIcon class="h-4 w-4 mr-1" />
                {{ $t('keyGenerator.result.saveFile') }}
              </BaseButton>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex space-x-3 pt-4">
            <BaseButton @click="goToKeyManager" class="flex-1">
              <EyeIcon class="h-4 w-4 mr-2" />
              {{ $t('keyGenerator.result.viewAllKeys') }}
            </BaseButton>
            <BaseButton variant="secondary" @click="generateAnother" class="flex-1">
              <PlusIcon class="h-4 w-4 mr-2" />
              {{ $t('keyGenerator.result.generateAnother') }}
            </BaseButton>
          </div>
        </div>
      </div>

      <!-- 默认状态 -->
      <div v-else class="bg-white rounded-lg shadow-sm p-6">
        <div class="text-center text-gray-500 py-8">
          <KeyIcon class="mx-auto h-12 w-12 text-gray-400 mb-4" />
          <h3 class="text-lg font-medium text-gray-900 mb-2">{{ $t('keyGenerator.defaultState.title') }}</h3>
          <p class="text-sm">{{ $t('keyGenerator.defaultState.subtitle') }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useKeyStore } from '@/stores/key'
import { useToast } from '@/composables/useToast'
import type { KeyGenerationParams, SshKeyType, SshKeyPair } from '@/types'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import {
  KeyIcon,
  CheckCircleIcon,
  ClipboardIcon,
  ArrowDownTrayIcon,
  EyeIcon,
  PlusIcon,
} from '@heroicons/vue/24/outline'

const router = useRouter()
const { t } = useI18n()
const { success, error: showError } = useToast()
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
const keyTypes = computed(() => [
  {
    value: 'Ed25519' as SshKeyType,
    name: 'Ed25519',
    description: t('keyGenerator.keyType.descriptions.Ed25519'),
    recommended: true
  },
  {
    value: 'Rsa' as SshKeyType,
    name: 'RSA',
    description: t('keyGenerator.keyType.descriptions.Rsa'),
    recommended: false
  },
  {
    value: 'Ecdsa' as SshKeyType,
    name: 'ECDSA',
    description: t('keyGenerator.keyType.descriptions.Ecdsa'),
    recommended: false
  }
])

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
  const keySizeValid = keyParams.key_type === 'Ed25519' || (
    !!keyParams.key_size && availableKeySizes.value.includes(keyParams.key_size)
  )

  if (!advancedOptions.usePassphrase) {
    return basicValid && keySizeValid
  }

  const passphraseValid = keyParams.passphrase &&
    keyParams.passphrase.length >= 8 &&
    keyParams.passphrase === passphraseConfirm.value

  return basicValid && keySizeValid && passphraseValid
})

// 密钥类型变化时更新密钥长度
const onKeyTypeChange = () => {
  const sizes = availableKeySizes.value
  if (keyParams.key_type === 'Ed25519') {
    keyParams.key_size = 256
    return
  }
  keyParams.key_size = Math.max(...sizes)
}

// 监听密钥类型变化，自动选择最大长度；首次也同步一次
watch(() => keyParams.key_type, () => {
  onKeyTypeChange()
}, { immediate: true })

// 生成密钥
const generateKey = async () => {
  // 清除错误
  errors.name = ''
  errors.passphrase = ''
  errors.passphraseConfirm = ''

  // 验证表单
  if (!keyParams.name.trim()) {
    errors.name = t('keyGenerator.errors.nameRequired')
    return
  }

  // 验证密码
  if (advancedOptions.usePassphrase) {
    if (!keyParams.passphrase || keyParams.passphrase.length < 8) {
      errors.passphrase = t('keyGenerator.errors.passphraseLength')
      return
    }

    if (keyParams.passphrase !== passphraseConfirm.value) {
      errors.passphraseConfirm = t('keyGenerator.errors.passphraseConfirm')
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
      { progress: 20, text: t('keyGenerator.generate.progress.init') },
      { progress: 40, text: t('keyGenerator.generate.progress.generate') },
      { progress: 70, text: t('keyGenerator.generate.progress.fingerprint') },
      { progress: 90, text: t('keyGenerator.generate.progress.format') },
      { progress: 100, text: t('keyGenerator.generate.progress.complete') }
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
      success(t('keyGenerator.messages.copyPublicKeySuccess'))
    } catch (err) {
      console.error('复制失败:', err)
      showError(t('keyGenerator.messages.copyPublicKeyError'))
    }
  }
}

// 复制指纹
const copyFingerprint = async () => {
  if (generatedKey.value) {
    try {
      await navigator.clipboard.writeText(generatedKey.value.fingerprint)
      success(t('keyGenerator.messages.copyFingerprintSuccess'))
    } catch (err) {
      console.error('复制失败:', err)
      showError(t('keyGenerator.messages.copyFingerprintError'))
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