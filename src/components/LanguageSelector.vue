<template>
  <div class="relative">
    <!-- 语言切换按钮 -->
    <button
      @click="showDropdown = !showDropdown"
      class="flex items-center space-x-2 px-3 py-2 text-sm text-gray-700 hover:bg-gray-50 rounded-md transition-colors"
      :class="variant === 'compact' ? 'p-2' : 'px-3 py-2'"
    >
      <LanguageIcon class="h-4 w-4" />
      <span v-if="variant !== 'compact'" class="hidden sm:block">
        {{ currentLanguage.nativeName }}
      </span>
      <ChevronDownIcon class="h-4 w-4" />
    </button>

    <!-- 下拉菜单 -->
    <div
      v-show="showDropdown"
      class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg border border-gray-200 z-50"
      @click.stop
    >
      <div class="py-1">
        <button
          v-for="language in languageStore.availableLanguages"
          :key="language.code"
          @click="selectLanguage(language.code)"
          class="flex items-center w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 transition-colors"
          :class="languageStore.currentLanguage === language.code ? 'bg-blue-50 text-blue-700' : ''"
        >
          <CheckIcon
            v-if="languageStore.currentLanguage === language.code"
            class="h-4 w-4 mr-2 text-blue-600"
          />
          <span v-else class="w-4 mr-2"></span>
          <div class="flex flex-col items-start">
            <span class="font-medium">{{ language.nativeName }}</span>
            <span class="text-xs text-gray-500">{{ language.name }}</span>
          </div>
        </button>
      </div>
    </div>

    <!-- 点击外部关闭下拉菜单 -->
    <div
      v-if="showDropdown"
      class="fixed inset-0 z-40"
      @click="showDropdown = false"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useLanguageStore } from '@/stores/language'
import {
  LanguageIcon,
  ChevronDownIcon,
  CheckIcon
} from '@heroicons/vue/24/outline'

interface Props {
  variant?: 'default' | 'compact'
}

const { variant = 'default' } = defineProps<Props>()

const languageStore = useLanguageStore()
const { locale: i18nLocale } = useI18n()
const showDropdown = ref(false)

// 当前语言信息
const currentLanguage = computed(() => {
  return languageStore.availableLanguages.find(
    lang => lang.code === languageStore.currentLanguage
  ) || languageStore.availableLanguages[0]
})

// 选择语言
const selectLanguage = (locale: string) => {
  // 更新store
  languageStore.setLanguage(locale)
  // 更新i18n locale
  i18nLocale.value = locale
  showDropdown.value = false
}

// 页面加载时关闭下拉菜单
onMounted(() => {
  showDropdown.value = false
})
</script>