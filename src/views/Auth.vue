<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import LanguageSelector from '@/components/LanguageSelector.vue'
import { KeyIcon } from '@heroicons/vue/24/outline'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const { t } = useI18n()

const password = ref('')
const confirmPassword = ref('')
const error = ref('')
const passwordError = ref('')
const confirmPasswordError = ref('')

const isSetupMode = computed(() => route.query.mode === 'setup')
// 直接获取安全提示数组
const securityTips = [
  t('auth.securityTips.items.0'),
  t('auth.securityTips.items.1'),
  t('auth.securityTips.items.2')
]

onMounted(async () => {
  // 检查初始化状态
  try {
    const initialized = await authStore.checkInitialization()
    if (!initialized && route.query.mode !== 'setup') {
      router.push({ name: 'Auth', query: { mode: 'setup' } })
    }
  } catch (err) {
    console.error('检查初始化状态失败:', err)
  }
})

const validateForm = (): boolean => {
  passwordError.value = ''
  confirmPasswordError.value = ''
  error.value = ''

  if (password.value.length < 8) {
    passwordError.value = t('auth.errors.passwordLength')
    return false
  }

  if (isSetupMode.value && password.value !== confirmPassword.value) {
    confirmPasswordError.value = t('auth.errors.passwordMismatch')
    return false
  }

  return true
}

const handleSubmit = async () => {
  if (!validateForm()) {
    return
  }

  try {
    let success = false

    if (isSetupMode.value) {
      success = await authStore.initializeApp(password.value)
    } else {
      success = await authStore.login(password.value)
    }

    if (success) {
      router.push({ name: 'Dashboard' })
    } else {
      error.value = isSetupMode.value ? t('auth.errors.initializationFailed') : t('auth.errors.wrongPassword')
    }
  } catch (err) {
    error.value = t('auth.errors.operationFailed')
    console.error('认证失败:', err)
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-transparent relative overflow-hidden">
    <!-- 背景装饰 -->
    <div class="absolute inset-0 pointer-events-none">
      <div
        class="absolute top-1/4 left-1/4 w-96 h-96 bg-gradient-to-r from-primary-400/30 to-accent-400/30 rounded-full blur-3xl animate-float">
      </div>
      <div
        class="absolute bottom-1/4 right-1/4 w-80 h-80 bg-gradient-to-r from-secondary-400/20 to-primary-400/20 rounded-full blur-3xl animate-float"
        style="animation-delay: -3s;"></div>
    </div>

    <!-- 右上角语言选择 -->
    <div class="absolute top-6 right-6 z-20">
      <LanguageSelector variant="compact" />
    </div>

    <div class="max-w-lg w-full space-y-8 p-8 relative z-10">
      <!-- 头部区域 -->
      <div class="text-center animate-fade-in">
        <div
          class="mx-auto w-20 h-20 bg-gradient-to-br from-primary-500 to-primary-600 rounded-2xl flex items-center justify-center shadow-glow mb-6">
          <KeyIcon class="w-10 h-10 text-white" />
        </div>
        <h2 class="text-4xl font-bold text-gradient mb-3">
          {{ isSetupMode ? $t('auth.setup.title') : $t('auth.login.title') }}
        </h2>
        <p class="text-tech-600 text-lg">
          {{ isSetupMode ? $t('auth.setup.subtitle') : $t('auth.login.subtitle') }}
        </p>
      </div>

      <!-- 表单卡片 -->
      <div class="card animate-slide-up" style="animation-delay: 0.2s">
        <div class="card-body">
          <form @submit.prevent="handleSubmit" class="space-y-6">
            <BaseInput v-model="password" :label="$t('auth.masterPassword')" type="password" required
              :placeholder="isSetupMode ? $t('auth.setupPlaceholder') : $t('auth.loginPlaceholder')"
              :error="passwordError" size="lg" />

            <BaseInput v-if="isSetupMode" v-model="confirmPassword" :label="$t('auth.confirmPassword')" type="password"
              required :placeholder="$t('auth.confirmPlaceholder')" :error="confirmPasswordError" size="lg" />

            <!-- 安全提示 -->
            <div v-if="isSetupMode"
              class="bg-gradient-to-r from-warning-50 to-warning-100 border border-warning-200 rounded-xl p-5 animate-slide-down">
              <div class="flex">
                <div class="flex-shrink-0">
                  <svg class="h-6 w-6 text-warning-500" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd"
                      d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                      clip-rule="evenodd" />
                  </svg>
                </div>
                <div class="ml-4">
                  <h3 class="text-sm font-semibold text-warning-800 mb-2">{{ $t('auth.securityTips.title') }}</h3>
                  <div class="text-sm text-warning-700">
                    <ul class="list-disc list-inside space-y-1">
                      <li v-for="(tip, index) in securityTips" :key="index">{{ tip }}</li>
                    </ul>
                  </div>
                </div>
              </div>
            </div>

            <!-- 错误信息 -->
            <div v-if="error"
              class="bg-gradient-to-r from-error-50 to-error-100 border border-error-200 text-error-700 rounded-xl p-4 animate-slide-down">
              <div class="flex items-center">
                <svg class="w-5 h-5 text-error-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd"
                    d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
                    clip-rule="evenodd" />
                </svg>
                {{ error }}
              </div>
            </div>

            <BaseButton type="submit" :disabled="authStore.isLoading || (isSetupMode && password !== confirmPassword)"
              :loading="authStore.isLoading" class="w-full" size="lg">
              <span v-if="!authStore.isLoading">{{ isSetupMode ? $t('auth.setup.button') : $t('auth.login.button')
                }}</span>
              <span v-else>{{ isSetupMode ? $t('auth.setup.initializing') : $t('auth.login.loggingIn') }}</span>
            </BaseButton>
          </form>
        </div>
      </div>

      <!-- 底部信息 -->
      <div class="text-center text-tech-500 text-sm animate-fade-in" style="animation-delay: 0.4s">
        <p>Powered by SSH Manager v1.0</p>
      </div>
    </div>
  </div>
</template>