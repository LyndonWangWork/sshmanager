/**
 * 语言检测和管理工具函数
 */

// 支持的语言代码
export const SUPPORTED_LANGUAGES = ['zh', 'en'] as const

export type SupportedLanguage = typeof SUPPORTED_LANGUAGES[number]

/**
 * 检测系统/浏览器语言
 * @returns 检测到的语言代码，如果没有匹配的语言则返回英文
 */
export const detectSystemLanguage = (): SupportedLanguage => {
  // 尝试从不同来源获取语言信息
  const sources = [
    // 浏览器语言（主要）
    navigator.language,
    // 浏览器语言列表中的首选语言
    navigator.languages?.[0],
    // 系统语言（备用）
    ...(navigator.languages || [])
  ]
  
  for (const lang of sources) {
    if (!lang) continue
    
    // 提取语言代码（去掉地区代码，如 zh-CN -> zh）
    const langCode = lang.toLowerCase().split('-')[0] as SupportedLanguage
    
    // 检查是否为支持的语言
    if (SUPPORTED_LANGUAGES.includes(langCode)) {
      return langCode
    }
  }
  
  // 如果没有匹配的语言，默认返回英文
  return 'en'
}

/**
 * 获取用户语言偏好
 * 优先使用保存的设置，其次使用系统语言，最后使用英文
 * @returns 用户的语言偏好
 */
export const getUserLanguage = (): SupportedLanguage => {
  const savedLanguage = localStorage.getItem('ssh-manager-language') as SupportedLanguage
  
  // 验证保存的语言是否有效
  if (savedLanguage && SUPPORTED_LANGUAGES.includes(savedLanguage)) {
    return savedLanguage
  }
  
  return detectSystemLanguage()
}

/**
 * 保存用户语言偏好到localStorage
 * @param language 要保存的语言代码
 */
export const saveUserLanguage = (language: SupportedLanguage): void => {
  localStorage.setItem('ssh-manager-language', language)
}