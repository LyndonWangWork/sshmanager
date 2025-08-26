import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useKeyStore } from '@/stores/key'
import type { SshKeyPair, KeyGenerationParams } from '@/types'

// Mock Tauri API
const mockInvoke = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke
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
    mockInvoke.mockResolvedValue([mockKey])
    
    const keyStore = useKeyStore()
    await keyStore.loadKeys()
    
    expect(keyStore.keys).toEqual([mockKey])
    expect(keyStore.isLoading).toBe(false)
    expect(mockInvoke).toHaveBeenCalledWith('get_all_keys')
  })

  it('loadKeys handles errors gracefully', async () => {
    mockInvoke.mockRejectedValue(new Error('Load failed'))
    
    const keyStore = useKeyStore()
    await keyStore.loadKeys()
    
    expect(keyStore.keys).toEqual([])
    expect(keyStore.isLoading).toBe(false)
  })

  it('generateKey creates new key successfully', async () => {
    const newKey = { ...mockKey, id: 'new-key-id', name: 'New Test Key' }
    mockInvoke.mockResolvedValue(newKey)
    
    const keyStore = useKeyStore()
    const result = await keyStore.generateKey(mockGenerationParams)
    
    expect(result).toEqual(newKey)
    expect(keyStore.keys).toContain(newKey)
    expect(mockInvoke).toHaveBeenCalledWith('generate_ssh_key', { 
      params: mockGenerationParams 
    })
  })

  it('generateKey handles generation failure', async () => {
    mockInvoke.mockRejectedValue(new Error('Generation failed'))
    
    const keyStore = useKeyStore()
    const result = await keyStore.generateKey(mockGenerationParams)
    
    expect(result).toBeNull()
    expect(keyStore.keys).toEqual([])
  })

  it('updateKey modifies existing key', async () => {
    const updatedKey = { ...mockKey, name: 'Updated Name' }
    mockInvoke.mockResolvedValue(updatedKey)
    
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey] // Initialize with existing key
    
    const result = await keyStore.updateKey(mockKey.id, { name: 'Updated Name' })
    
    expect(result).toEqual(updatedKey)
    expect(keyStore.keys[0].name).toBe('Updated Name')
    expect(mockInvoke).toHaveBeenCalledWith('update_ssh_key', {
      key_id: mockKey.id,
      updates: { name: 'Updated Name' }
    })
  })

  it('updateKey handles update failure', async () => {
    mockInvoke.mockRejectedValue(new Error('Update failed'))
    
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey]
    
    const result = await keyStore.updateKey(mockKey.id, { name: 'Updated Name' })
    
    expect(result).toBeNull()
    expect(keyStore.keys[0].name).toBe(mockKey.name) // Unchanged
  })

  it('deleteKey removes key successfully', async () => {
    mockInvoke.mockResolvedValue({ success: true })
    
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey] // Initialize with existing key
    
    const result = await keyStore.deleteKey(mockKey.id)
    
    expect(result).toBe(true)
    expect(keyStore.keys).toEqual([])
    expect(mockInvoke).toHaveBeenCalledWith('delete_ssh_key', { key_id: mockKey.id })
  })

  it('deleteKey handles deletion failure', async () => {
    mockInvoke.mockResolvedValue({ success: false })
    
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey]
    
    const result = await keyStore.deleteKey(mockKey.id)
    
    expect(result).toBe(false)
    expect(keyStore.keys).toContain(mockKey) // Still present
  })

  it('importKeys handles import successfully', async () => {
    const importedKeys = [mockKey]
    mockInvoke.mockResolvedValue({ success: true, imported_keys: importedKeys })
    
    const keyStore = useKeyStore()
    const result = await keyStore.importKeys('file-path')
    
    expect(result.success).toBe(true)
    expect(result.imported_keys).toEqual(importedKeys)
    expect(keyStore.keys).toEqual(importedKeys)
    expect(mockInvoke).toHaveBeenCalledWith('import_keys', { file_path: 'file-path' })
  })

  it('importKeys handles import failure', async () => {
    mockInvoke.mockResolvedValue({ success: false, error: 'Invalid file format' })
    
    const keyStore = useKeyStore()
    const result = await keyStore.importKeys('invalid-file')
    
    expect(result.success).toBe(false)
    expect(result.error).toBe('Invalid file format')
  })

  it('exportKeys handles export successfully', async () => {
    mockInvoke.mockResolvedValue({ success: true })
    
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey]
    
    const result = await keyStore.exportKeys(['test-key-1'], 'export-path')
    
    expect(result.success).toBe(true)
    expect(mockInvoke).toHaveBeenCalledWith('export_keys', {
      key_ids: ['test-key-1'],
      file_path: 'export-path'
    })
  })

  it('exportKeys handles export failure', async () => {
    mockInvoke.mockResolvedValue({ success: false, error: 'Permission denied' })
    
    const keyStore = useKeyStore()
    const result = await keyStore.exportKeys(['test-key-1'], 'invalid-path')
    
    expect(result.success).toBe(false)
    expect(result.error).toBe('Permission denied')
  })

  it('getKeyById returns correct key', () => {
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey]
    
    const result = keyStore.getKeyById(mockKey.id)
    
    expect(result).toEqual(mockKey)
  })

  it('getKeyById returns undefined for non-existent key', () => {
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey]
    
    const result = keyStore.getKeyById('non-existent')
    
    expect(result).toBeUndefined()
  })

  it('keyCount computed property works correctly', () => {
    const keyStore = useKeyStore()
    
    expect(keyStore.keyCount).toBe(0)
    
    keyStore.keys = [mockKey]
    expect(keyStore.keyCount).toBe(1)
    
    keyStore.keys = [mockKey, { ...mockKey, id: 'key-2' }]
    expect(keyStore.keyCount).toBe(2)
  })

  it('keysByType computed property groups keys correctly', () => {
    const rsaKey = { ...mockKey, id: 'rsa-key', key_type: 'RSA' as any }
    const ecdsaKey = { ...mockKey, id: 'ecdsa-key', key_type: 'ECDSA' as any }
    
    const keyStore = useKeyStore()
    keyStore.keys = [mockKey, rsaKey, ecdsaKey]
    
    const result = keyStore.keysByType
    
    expect(result.Ed25519).toEqual([mockKey])
    expect(result.RSA).toEqual([rsaKey])
    expect(result.ECDSA).toEqual([ecdsaKey])
  })
})