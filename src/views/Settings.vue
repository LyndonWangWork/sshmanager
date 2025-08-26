<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 导航栏 -->
    <nav class="bg-white shadow-sm border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <router-link to="/" class="text-blue-600 hover:text-blue-800 mr-4">
              <ArrowLeftIcon class="h-5 w-5" />
            </router-link>
            <h1 class="text-xl font-semibold text-gray-900">{{ $t('settings.title') }}</h1>
          </div>
        </div>
      </div>
    </nav>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="space-y-6">
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
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useLanguageStore } from '@/stores/language'
import { ArrowLeftIcon } from '@heroicons/vue/24/outline'

const languageStore = useLanguageStore()
const { locale: i18nLocale } = useI18n()

// 切换语言
const handleLanguageChange = (locale: string) => {
  // 更新store
  languageStore.setLanguage(locale)
  // 更新i18n locale
  i18nLocale.value = locale
}
</script>