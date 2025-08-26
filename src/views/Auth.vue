<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 relative">
    <!-- 右上角语言选择 -->
    <div class="absolute top-4 right-4">
      <LanguageSelector variant="compact" />
    </div>
    
    <div class="max-w-md w-full space-y-8 p-8">
      <div class="text-center">
        <h2 class="text-3xl font-bold text-gray-900">
          {{ isSetupMode ? $t('auth.setup.title') : $t('auth.login.title') }}
        </h2>
        <p class="mt-2 text-sm text-gray-600">
          {{ isSetupMode ? $t('auth.setup.subtitle') : $t('auth.login.subtitle') }}
        </p>
      </div>
      
      <div class="bg-white rounded-lg shadow-md p-6">
        <form @submit.prevent="handleSubmit" class="space-y-6">
          <BaseInput
            v-model="password"
            :label="$t('auth.masterPassword')"
            type="password"
            required
            :placeholder="isSetupMode ? $t('auth.setupPlaceholder') : $t('auth.loginPlaceholder')"
            :error="passwordError"
          />
          
          <BaseInput
            v-if="isSetupMode"
            v-model="confirmPassword"
            :label="$t('auth.confirmPassword')"
            type="password"
            required
            :placeholder="$t('auth.confirmPlaceholder')"
            :error="confirmPasswordError"
          />
          
          <!-- 安全提示 -->
          <div v-if="isSetupMode" class="bg-yellow-50 border border-yellow-200 rounded-md p-4">
            <div class="flex">
              <div class="flex-shrink-0">
                <svg class="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
              </div>
              <div class="ml-3">
                <h3 class="text-sm font-medium text-yellow-800">{{ $t('auth.securityTips.title') }}</h3>
                <div class="mt-2 text-sm text-yellow-700">
                  <ul class="list-disc list-inside space-y-1">
                    <li v-for="tip in $t('auth.securityTips.items')" :key="tip">{{ tip }}</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          
          <div v-if="error" class="text-red-600 text-sm bg-red-50 p-3 rounded-md">
            {{ error }}
          </div>
          
          <BaseButton 
            type="submit" 
            :disabled="authStore.isLoading || (isSetupMode && password !== confirmPassword)"
            class="w-full"
          >
            <span v-if="authStore.isLoading">{{ isSetupMode ? $t('auth.setup.initializing') : $t('auth.login.loggingIn') }}</span>
            <span v-else>{{ isSetupMode ? $t('auth.setup.button') : $t('auth.login.button') }}</span>
          </BaseButton>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import LanguageSelector from '@/components/LanguageSelector.vue'

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