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
            <label v-for="language in languageStore.availableLanguages" :key="language.code"
              class="flex items-center p-3 border rounded-lg cursor-pointer hover:bg-gray-50 transition-colors"
              :class="languageStore.currentLanguage === language.code ? 'border-blue-500 bg-blue-50 ring-2 ring-blue-200' : 'border-gray-200'">
              <input :value="language.code" :checked="languageStore.currentLanguage === language.code" type="radio"
                name="language" class="mr-3" @change="handleLanguageChange(language.code)" />
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
        @click="confirmReset">
        {{ $t('settings.reset.button') }}
      </button>
    </div>
    <!-- 确认对话框 -->
    <ConfirmDialog :visible="showResetConfirm" :title="$t('settings.reset.title')"
      :message="$t('settings.reset.confirm')" :confirm-button-text="$t('settings.reset.button')"
      :require-password="true" @confirm="handleResetConfirm" @cancel="handleResetCancel" />
  </div>

</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLanguageStore } from '@/stores/language'
import { useAuthStore } from '@/stores/auth'
import { useKeyStore } from '@/stores/key'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { relaunch } from '@tauri-apps/plugin-process'
import ConfirmDialog from '@/components/ConfirmDialog.vue'

const languageStore = useLanguageStore()
const authStore = useAuthStore()
const keyStore = useKeyStore()

const { locale: i18nLocale, t: $t } = useI18n()

// 显示确认对话框
const showResetConfirm = ref(false)

// 切换语言
const handleLanguageChange = (locale: string) => {
  // 更新store
  languageStore.setLanguage(locale)
  // 更新i18n locale
  i18nLocale.value = locale
}

// 重置所有数据
const resetAllData = async (masterKey: string) => {
  try {
    // 调用Tauri命令验证密码并重置数据
    const result = await invoke('reset_all_data', { masterKey })

    if (result) {
      // 重置语言
      languageStore.resetLanguage()

      // 重置认证状态
      authStore.reset()

      // 清除所有密钥
      keyStore.clearKeys()

      // 关闭确认对话框
      showResetConfirm.value = false

      // 显示成功提示
      alert($t('settings.reset.success'))

      // 重启应用程序
      await relaunch()
    } else {
      throw new Error($t('auth.errors.wrongPassword'))
    }
  } catch (error: any) {
    throw error
  }
}

// 处理重置确认
const handleResetConfirm = async (password?: string) => {
  try {
    if (!password) {
      throw new Error($t('auth.errors.passwordLength'))
    }
    await resetAllData(password)
  } catch (error: any) {
    // 简化处理：直接alert错误信息
    alert(error.message || $t('auth.errors.operationFailed'))
  }
}

// 处理重置取消
const handleResetCancel = () => {
  showResetConfirm.value = false
}

// 确认重置
const confirmReset = async () => {
  // 显示确认对话框，让用户输入密码
  showResetConfirm.value = true
}
</script>