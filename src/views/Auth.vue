<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50">
    <div class="max-w-md w-full space-y-8 p-8">
      <div class="text-center">
        <h2 class="text-3xl font-bold text-gray-900">
          {{ isSetupMode ? '初始化应用' : '解锁应用' }}
        </h2>
        <p class="mt-2 text-sm text-gray-600">
          {{ isSetupMode ? '设置主密码来保护您的SSH密钥' : '输入主密码来访问您的密钥' }}
        </p>
      </div>
      
      <div class="bg-white rounded-lg shadow-md p-6">
        <form @submit.prevent="handleSubmit" class="space-y-6">
          <BaseInput
            v-model="password"
            label="主密码"
            type="password"
            required
            :placeholder="isSetupMode ? '请设置一个强密码' : '请输入主密码'"
            :error="passwordError"
          />
          
          <BaseInput
            v-if="isSetupMode"
            v-model="confirmPassword"
            label="确认密码"
            type="password"
            required
            placeholder="请再次输入密码"
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
                <h3 class="text-sm font-medium text-yellow-800">重要提示</h3>
                <div class="mt-2 text-sm text-yellow-700">
                  <ul class="list-disc list-inside space-y-1">
                    <li>请妥善保管您的主密码，一旦丢失无法找回</li>
                    <li>建议使用包含大小写字母、数字和特殊字符的强密码</li>
                    <li>密码长度建议不少于12位</li>
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
            <span v-if="authStore.isLoading">{{ isSetupMode ? '初始化中...' : '登录中...' }}</span>
            <span v-else>{{ isSetupMode ? '初始化应用' : '解锁' }}</span>
          </BaseButton>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

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
    passwordError.value = '密码长度不能少于8位'
    return false
  }
  
  if (isSetupMode.value && password.value !== confirmPassword.value) {
    confirmPasswordError.value = '两次输入的密码不一致'
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
      error.value = isSetupMode.value ? '初始化失败' : '密码错误'
    }
  } catch (err) {
    error.value = '操作失败，请稍后重试'
    console.error('认证失败:', err)
  }
}
</script>