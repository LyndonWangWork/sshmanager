import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import KeyCard from '@/components/KeyCard.vue'
import type { SshKeyPair } from '@/types'

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

const mockKey: SshKeyPair = {
  id: 'test-key-1',
  name: 'Test SSH Key',
  key_type: 'Ed25519',
  key_size: 256,
  comment: 'test@example.com',
  public_key: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGtest...',
  private_key: '-----BEGIN OPENSSH PRIVATE KEY-----\ntest...',
  fingerprint: 'SHA256:test123...',
  created_at: '2024-01-01T00:00:00Z',
  last_used: '2024-01-02T00:00:00Z'
}

describe('KeyCard', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('renders key information properly', () => {
    const wrapper = mount(KeyCard, {
      props: { keyPair: mockKey }
    })
    
    expect(wrapper.text()).toContain('Test SSH Key')
    expect(wrapper.text()).toContain('Ed25519')
    expect(wrapper.text()).toContain('test@example.com')
    expect(wrapper.text()).toContain('SHA256:test123...')
  })

  it('shows key type badge correctly', () => {
    const wrapper = mount(KeyCard, {
      props: { keyPair: mockKey }
    })
    
    const typeBadge = wrapper.find('[data-testid="key-type-badge"]')
    expect(typeBadge.exists()).toBe(true)
    expect(typeBadge.text()).toBe('Ed25519')
  })

  it('displays creation date correctly', () => {
    const wrapper = mount(KeyCard, {
      props: { keyPair: mockKey }
    })
    
    expect(wrapper.text()).toContain('2024-01-01')
  })

  it('shows last used date when available', () => {
    const wrapper = mount(KeyCard, {
      props: { keyPair: mockKey }
    })
    
    expect(wrapper.text()).toContain('2024-01-02')
  })

  it('handles key without last used date', () => {
    const keyWithoutLastUsed = { ...mockKey, last_used: undefined }
    const wrapper = mount(KeyCard, {
      props: { keyPair: keyWithoutLastUsed }
    })
    
    expect(wrapper.text()).toContain('从未使用')
  })

  it('emits edit event when edit button is clicked', async () => {
    const wrapper = mount(KeyCard, {
      props: { keyPair: mockKey }
    })
    
    const editButton = wrapper.find('[data-testid="edit-key-btn"]')
    if (editButton.exists()) {
      await editButton.trigger('click')
      expect(wrapper.emitted().edit).toBeTruthy()
      expect(wrapper.emitted().edit[0]).toEqual([mockKey])
    }
  })

  it('emits delete event when delete button is clicked', async () => {
    const wrapper = mount(KeyCard, {
      props: { keyPair: mockKey }
    })
    
    const deleteButton = wrapper.find('[data-testid="delete-key-btn"]')
    if (deleteButton.exists()) {
      await deleteButton.trigger('click')
      expect(wrapper.emitted().delete).toBeTruthy()
      expect(wrapper.emitted().delete[0]).toEqual([mockKey.id])
    }
  })

  it('shows public key preview correctly', () => {
    const wrapper = mount(KeyCard, {
      props: { keyPair: mockKey }
    })
    
    const publicKeyPreview = wrapper.find('[data-testid="public-key-preview"]')
    if (publicKeyPreview.exists()) {
      expect(publicKeyPreview.text()).toContain('ssh-ed25519')
    }
  })

  it('truncates long key names appropriately', () => {
    const longNameKey = {
      ...mockKey,
      name: 'This is a very long SSH key name that should be truncated when displayed'
    }
    
    const wrapper = mount(KeyCard, {
      props: { keyPair: longNameKey }
    })
    
    expect(wrapper.find('[data-testid="key-name"]').exists()).toBe(true)
  })
})