<template>
  <div class="space-y-6">
    <!-- 页面标题 -->
    <div class="mb-8">
      <h1 class="text-2xl font-semibold text-gray-900">{{ $t('settings.title') }}</h1>
    </div>

    <!-- 语言设置 -->
    <div class="bg-white rounded-lg shadow-sm p-6">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('settings.language.title') }}</h2>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            {{ $t('settings.language.select') }}
          </label>
          <div class="space-y-2">
            <label
              v-for="language in languageStore.availableLanguages"
              :key="language.code"
              class="flex items-center p-3 border rounded-lg cursor-pointer hover:bg-gray-50 transition-colors"
              :class="languageStore.currentLanguage === language.code ? 'border-blue-500 bg-blue-50 ring-2 ring-blue-200' : 'border-gray-200'"
            >
              <input
                :value="language.code"
                :checked="languageStore.currentLanguage === language.code"
                type="radio"
                name="language"
                class="mr-3"
                @change="handleLanguageChange(language.code)"
              />
              <div class="flex items-center space-x-3">
                <div>
                  <div class="font-medium text-gray-900">{{ language.nativeName }}</div>
                  <div class="text-sm text-gray-500">{{ language.name }}</div>
                </div>
              </div>
            </label>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 其他设置内容 -->
    <div class="bg-white rounded-lg shadow-sm p-6">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('settings.appSettings') }}</h2>
      <p class="text-gray-600">{{ $t('settings.developing') }}</p>
    </div>
    
    <!-- 重置功能 -->
    <div class="bg-white rounded-lg shadow-sm p-6">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">{{ $t('settings.reset.title') }}</h2>
      <p class="text-gray-600 mb-4">{{ $t('settings.reset.description') }}</p>
      <button
        class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 transition-colors"
        @click="confirmReset"
      >
        {{ $t('settings.reset.button') }}
      </button>
    </div>
  </div>
  
  <!-- 确认对话框 -->
  <div v-if="showResetConfirm" class="fixed inset-0 z-50 overflow-y-auto">
    <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
      <div class="fixed inset-0 transition-opacity">
        <div class="absolute inset-0 bg-gray-500 opacity-75"></div>
      </div>
      
      <span class="hidden sm:inline-block sm:align-middle sm:h-screen"></span>&#8203;
      <div class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
        <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
          <div class="sm:flex sm:items-start">
            <div class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 bg-red-100 rounded-full sm:mx-0 sm:h-10 sm:w-10">
              <svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
              </svg>
            </div>
            <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
              <h3 class="text-lg leading-6 font-medium text-gray-900">
                {{ $t('settings.reset.title') }}
              </h3>
              <div class="mt-2">
                <p class="text-sm text-gray-500">
                  {{ $t('settings.reset.confirm') }}
                </p>
              </div>
              
              <!-- 密码输入框 -->
              <div class="mt-4">
                <label class="block text-sm font-medium text-gray-700 mb-2">
                  {{ $t('auth.masterPassword') }}
                </label>
                <input
                  v-model="resetPassword"
                  type="password"
                  :placeholder="$t('auth.loginPlaceholder')"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                  :class="{ 'border-red-500': resetPasswordError }"
                />
                <p v-if="resetPasswordError" class="mt-1 text-sm text-red-600">
                  {{ resetPasswordError }}
                </p>
              </div>
            </div>
          </div>
        </div>
        <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
          <button
            type="button"
            class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
            @click="resetAllData"
          >
            {{ $t('settings.reset.button') }}
          </button>
          <button
            type="button"
            class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
            @click="showResetConfirm = false"
          >
            {{ $t('common.cancel') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLanguageStore } from '@/stores/language'
import { useAuthStore } from '@/stores/auth'
import { useKeyStore } from '@/stores/key'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const languageStore = useLanguageStore()
const authStore = useAuthStore()
const keyStore = useKeyStore()

const { locale: i18nLocale, t: $t } = useI18n()

// 显示确认对话框
const showResetConfirm = ref(false)
// 密码输入
const resetPassword = ref('')
const resetPasswordError = ref('')

// 切换语言
const handleLanguageChange = (locale: string) => {
  // 更新store
  languageStore.setLanguage(locale)
  // 更新i18n locale
  i18nLocale.value = locale
}

// 验证重置密码
const validateResetPassword = (): boolean => {
  resetPasswordError.value = ''
  
  if (resetPassword.value.length < 8) {
    resetPasswordError.value = $t('auth.errors.passwordLength')
    return false
  }
  
  return true
}

// 重置所有数据
const resetAllData = async () => {
  if (!validateResetPassword()) {
    return
  }
  
  try {
    // 调用Tauri命令验证密码并重置数据
    const result = await invoke('reset_all_data', { masterKey: resetPassword.value })
    
    if (result) {
      // 重置语言
      languageStore.resetLanguage()
      
      // 重置认证状态
      authStore.reset()
      
      // 清除所有密钥
      keyStore.clearKeys()
      
      // 关闭确认对话框
      showResetConfirm.value = false
      
      // 清空密码输入
      resetPassword.value = ''
      
      // 显示成功提示
      alert($t('settings.reset.success'))
    } else {
      resetPasswordError.value = $t('auth.errors.wrongPassword')
    }
  } catch (error) {
    resetPasswordError.value = $t('auth.errors.operationFailed')
    console.error('重置失败:', error)
  }
}

// 确认重置
const confirmReset = async () => {
  // 显示确认对话框，让用户输入密码
  showResetConfirm.value = true
  // 清空之前的错误信息
  resetPasswordError.value = ''
  // 清空密码输入
  resetPassword.value = ''
}
</script>