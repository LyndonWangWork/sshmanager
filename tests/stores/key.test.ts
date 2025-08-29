import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useKeyStore } from '@/stores/key'
import type { SshKeyPair, KeyGenerationParams } from '@/types'

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

const mockKey: SshKeyPair = {
  id: 'test-key-1',
  name: 'Test Key',
  key_type: 'Ed25519',
  key_size: 256,
  comment: 'test@example.com',
  public_key: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGtest...',
  private_key: '-----BEGIN OPENSSH PRIVATE KEY-----\ntest...',
  fingerprint: 'SHA256:test123...',
  created_at: '2024-01-01T00:00:00Z'
}

const mockGenerationParams: KeyGenerationParams = {
  name: 'New Test Key',
  key_type: 'Ed25519',
  key_size: 256,
  comment: 'newkey@example.com',
  passphrase: ''
}

describe('Key Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('initializes with empty state', () => {
    const keyStore = useKeyStore()
    
    expect(keyStore.keys).toEqual([])
    expect(keyStore.isLoading).toBe(false)
  })

  it('loadKeys fetches all keys successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    invoke.mockResolvedValue([mockKey])
    
    const keyStore = useKeyStore()
    await keyStore.loadKeys()
    
    expect(keyStore.keys).toEqual([mockKey])
    expect(keyStore.isLoading).toBe(false)
    expect(invoke).toHaveBeenCalledWith('get_all_keys')
  })

  it('generateKey creates new key successfully', async () => {
    const newKey = { ...mockKey, id: 'new-key-id', name: 'New Test Key' }
    const { invoke } = await import('@tauri-apps/api/core')
    invoke.mockResolvedValue(newKey)
    
    const keyStore = useKeyStore()
    const result = await keyStore.generateKey(mockGenerationParams)
    
    expect(result).toEqual(newKey)
    expect(keyStore.keys).toContainEqual(newKey)
    expect(invoke).toHaveBeenCalledWith('generate_ssh_key', { 
      params: mockGenerationParams 
    })
  })

  it('deleteKey removes key successfully', async () => {
    const { invoke } = await import('@tauri-apps/api/core')
    invoke.mockResolvedValue(true)
    
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey] // Initialize with existing key
    
    const result = await keyStore.deleteKey(mockKey.id)
    
    expect(result).toBe(true)
    expect(keyStore.keys).toEqual([])
    expect(invoke).toHaveBeenCalledWith('delete_key', { keyId: mockKey.id })
  })
})