import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getUserLanguage, saveUserLanguage, SUPPORTED_LANGUAGES, SupportedLanguage } from '@/utils/language'

export const useLanguageStore = defineStore('language', () => {
  const currentLanguage = ref(getUserLanguage())
  
  // 切换语言
  const setLanguage = (locale: string) => {
    currentLanguage.value = locale as SupportedLanguage
    saveUserLanguage(locale as any) // 使用共享工具函数保存
    
    // 注意：i18n locale 的更新需要在组件中进行
    // 这里只更新store的状态和localStorage
  }
  
  // 重置语言为默认值
  const resetLanguage = () => {
    const defaultLanguage = getUserLanguage()
    currentLanguage.value = defaultLanguage
    saveUserLanguage(defaultLanguage as any)
  }
  
  // 获取可用语言列表
  const availableLanguages = [
    { code: 'zh', name: '中文', nativeName: '中文' },
    { code: 'en', name: 'English', nativeName: 'English' }
  ]
  
  return {
    currentLanguage,
    availableLanguages,
    setLanguage,
    resetLanguage
  }
})