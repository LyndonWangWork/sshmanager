import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useAuthStore } from '@/stores/auth'

// Mock Tauri API
const mockInvoke = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke
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
    mockInvoke.mockResolvedValue(true)
    
    const authStore = useAuthStore()
    const result = await authStore.checkInitialization()
    
    expect(result).toBe(true)
    expect(authStore.isInitialized).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('is_app_initialized')
  })

  it('checkInitialization handles uninitialized app', async () => {
    mockInvoke.mockResolvedValue(false)
    
    const authStore = useAuthStore()
    const result = await authStore.checkInitialization()
    
    expect(result).toBe(false)
    expect(authStore.isInitialized).toBe(false)
  })

  it('initializeApp sets master password successfully', async () => {
    mockInvoke.mockResolvedValue({ success: true })
    
    const authStore = useAuthStore()
    const result = await authStore.initializeApp('test-password')
    
    expect(result.success).toBe(true)
    expect(authStore.isInitialized).toBe(true)
    expect(authStore.isAuthenticated).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('initialize_app', { 
      master_password: 'test-password' 
    })
  })

  it('initializeApp handles initialization failure', async () => {
    mockInvoke.mockResolvedValue({ success: false, error: 'Initialization failed' })
    
    const authStore = useAuthStore()
    const result = await authStore.initializeApp('test-password')
    
    expect(result.success).toBe(false)
    expect(result.error).toBe('Initialization failed')
    expect(authStore.isInitialized).toBe(false)
    expect(authStore.isAuthenticated).toBe(false)
  })

  it('login authenticates user successfully', async () => {
    mockInvoke.mockResolvedValue({ success: true })
    
    const authStore = useAuthStore()
    const result = await authStore.login('correct-password')
    
    expect(result.success).toBe(true)
    expect(authStore.isAuthenticated).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('authenticate', { 
      master_password: 'correct-password' 
    })
  })

  it('login handles authentication failure', async () => {
    mockInvoke.mockResolvedValue({ success: false, error: 'Invalid password' })
    
    const authStore = useAuthStore()
    const result = await authStore.login('wrong-password')
    
    expect(result.success).toBe(false)
    expect(result.error).toBe('Invalid password')
    expect(authStore.isAuthenticated).toBe(false)
  })

  it('logout clears authentication state', () => {
    const authStore = useAuthStore()
    
    // Set authenticated state
    authStore.isAuthenticated = true
    
    authStore.logout()
    
    expect(authStore.isAuthenticated).toBe(false)
  })

  it('handles API errors gracefully', async () => {
    mockInvoke.mockRejectedValue(new Error('Network error'))
    
    const authStore = useAuthStore()
    const result = await authStore.login('test-password')
    
    expect(result.success).toBe(false)
    expect(result.error).toBe('Network error')
    expect(authStore.isAuthenticated).toBe(false)
  })

  it('changeMasterPassword updates password successfully', async () => {
    mockInvoke.mockResolvedValue({ success: true })
    
    const authStore = useAuthStore()
    authStore.isAuthenticated = true // Set authenticated first
    
    const result = await authStore.changeMasterPassword('old-pass', 'new-pass')
    
    expect(result.success).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('change_master_password', {
      old_password: 'old-pass',
      new_password: 'new-pass'
    })
  })

  it('changeMasterPassword handles failure', async () => {
    mockInvoke.mockResolvedValue({ success: false, error: 'Invalid old password' })
    
    const authStore = useAuthStore()
    authStore.isAuthenticated = true
    
    const result = await authStore.changeMasterPassword('wrong-old', 'new-pass')
    
    expect(result.success).toBe(false)
    expect(result.error).toBe('Invalid old password')
  })
})