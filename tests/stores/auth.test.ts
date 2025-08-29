import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useAuthStore } from '@/stores/auth'

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

describe('Auth Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('initializes with correct default state', () => {
    const authStore = useAuthStore()
    
    expect(authStore.isAuthenticated).toBe(false)
    expect(authStore.isInitialized).toBe(false)
  })

  it('checkInitialization sets initialized state correctly', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    invoke.mockResolvedValue(true)
    
    const authStore = useAuthStore()
    const result = await authStore.checkInitialization()
    
    expect(result).toBe(true)
    expect(authStore.isInitialized).toBe(true)
    expect(invoke).toHaveBeenCalledWith('is_initialized')
  })

  it('login authenticates user successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    invoke.mockResolvedValue(true)
    
    const authStore = useAuthStore()
    const result = await authStore.login('correct-password')
    
    expect(result).toBe(true)
    expect(authStore.isAuthenticated).toBe(true)
    expect(invoke).toHaveBeenCalledWith('authenticate', { 
      masterKey: 'correct-password' 
    })
  })

  it('logout clears authentication state', () => {
    const authStore = useAuthStore()
    
    // Set authenticated state
    authStore.isAuthenticated = true
    
    authStore.logout()
    
    expect(authStore.isAuthenticated).toBe(false)
  })
})