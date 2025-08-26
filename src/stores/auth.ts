import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useAuthStore = defineStore('auth', () => {
  const isAuthenticated = ref(false)
  const isInitialized = ref(false)
  const isLoading = ref(false)
  
  // 检查是否已初始化
  const checkInitialization = async (): Promise<boolean> => {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('is_initialized')
      isInitialized.value = result
      return result
    } catch (error) {
      console.error('检查初始化状态失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }
  
  // 初始化应用
  const initializeApp = async (masterKey: string): Promise<boolean> => {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('initialize_app', { masterKey })
      if (result) {
        isInitialized.value = true
        isAuthenticated.value = true
      }
      return result
    } catch (error) {
      console.error('初始化应用失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }
  
  // 用户登录
  const login = async (masterKey: string): Promise<boolean> => {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('authenticate', { masterKey })
      isAuthenticated.value = result
      return result
    } catch (error) {
      console.error('登录失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }
  
  // 用户登出
  const logout = () => {
    isAuthenticated.value = false
  }
  
  return {
    // State
    isAuthenticated: computed(() => isAuthenticated.value),
    isInitialized: computed(() => isInitialized.value),
    isLoading: computed(() => isLoading.value),
    
    // Actions
    checkInitialization,
    initializeApp,
    login,
    logout,
  }
})