import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import KeyGenerator from '@/views/KeyGenerator.vue'

// Mock Tauri API
const mockInvoke = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke
}))

// Mock router
const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: { template: '<div>Home</div>' } },
    { path: '/keys', component: { template: '<div>Keys</div>' } }
  ]
})

describe('KeyGenerator', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('renders key generation form correctly', async () => {
    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    expect(wrapper.find('h1').text()).toContain('生成SSH密钥')
    expect(wrapper.find('input[name="name"]').exists()).toBe(true)
    expect(wrapper.find('select[name="keyType"]').exists()).toBe(true)
    expect(wrapper.find('input[name="comment"]').exists()).toBe(true)
  })

  it('displays all key type options', async () => {
    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    const keyTypeSelect = wrapper.find('select[name="keyType"]')
    const options = keyTypeSelect.findAll('option')
    
    expect(options.some(option => option.text().includes('RSA'))).toBe(true)
    expect(options.some(option => option.text().includes('Ed25519'))).toBe(true)
    expect(options.some(option => option.text().includes('ECDSA'))).toBe(true)
  })

  it('shows key size options for RSA keys', async () => {
    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    // Change to RSA key type
    const keyTypeSelect = wrapper.find('select[name="keyType"]')
    await keyTypeSelect.setValue('RSA')

    const keySizeSelect = wrapper.find('select[name="keySize"]')
    expect(keySizeSelect.exists()).toBe(true)
    
    const options = keySizeSelect.findAll('option')
    expect(options.some(option => option.text().includes('2048'))).toBe(true)
    expect(options.some(option => option.text().includes('4096'))).toBe(true)
  })

  it('hides key size options for Ed25519 keys', async () => {
    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    // Change to Ed25519 key type
    const keyTypeSelect = wrapper.find('select[name="keyType"]')
    await keyTypeSelect.setValue('Ed25519')

    const keySizeSelect = wrapper.find('select[name="keySize"]')
    expect(keySizeSelect.exists()).toBe(false)
  })

  it('validates required fields before generation', async () => {
    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    const generateButton = wrapper.find('[data-testid="generate-key-btn"]')
    await generateButton.trigger('click')

    // Should show validation errors for empty required fields
    expect(wrapper.text()).toContain('请输入密钥名称')
  })

  it('generates key with valid form data', async () => {
    const mockGeneratedKey = {
      id: 'new-key-id',
      name: 'Test Key',
      key_type: 'Ed25519',
      key_size: 256,
      comment: 'test@example.com',
      public_key: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGtest...',
      private_key: '-----BEGIN OPENSSH PRIVATE KEY-----\ntest...',
      fingerprint: 'SHA256:test123...',
      created_at: '2024-01-01T00:00:00Z'
    }

    mockInvoke.mockResolvedValue(mockGeneratedKey)

    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    // Fill form
    await wrapper.find('input[name="name"]').setValue('Test Key')
    await wrapper.find('input[name="comment"]').setValue('test@example.com')
    await wrapper.find('select[name="keyType"]').setValue('Ed25519')

    // Submit form
    const generateButton = wrapper.find('[data-testid="generate-key-btn"]')
    await generateButton.trigger('click')

    expect(mockInvoke).toHaveBeenCalledWith('generate_ssh_key', {
      params: {
        name: 'Test Key',
        key_type: 'Ed25519',
        key_size: 256,
        comment: 'test@example.com',
        passphrase: ''
      }
    })
  })

  it('shows progress during key generation', async () => {
    // Mock slow key generation
    mockInvoke.mockImplementation(() => new Promise(resolve => setTimeout(resolve, 100)))

    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    // Fill and submit form
    await wrapper.find('input[name="name"]').setValue('Test Key')
    await wrapper.find('input[name="comment"]').setValue('test@example.com')
    
    const generateButton = wrapper.find('[data-testid="generate-key-btn"]')
    await generateButton.trigger('click')

    // Should show loading state
    expect(wrapper.text()).toContain('生成中')
    expect(generateButton.attributes('disabled')).toBeDefined()
  })

  it('handles generation errors gracefully', async () => {
    mockInvoke.mockRejectedValue(new Error('Generation failed'))

    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    // Fill and submit form
    await wrapper.find('input[name="name"]').setValue('Test Key')
    await wrapper.find('input[name="comment"]').setValue('test@example.com')
    
    await wrapper.find('[data-testid="generate-key-btn"]').trigger('click')

    // Should show error message
    expect(wrapper.text()).toContain('生成失败')
  })

  it('resets form after successful generation', async () => {
    const mockGeneratedKey = {
      id: 'new-key-id',
      name: 'Test Key',
      key_type: 'Ed25519',
      key_size: 256,
      comment: 'test@example.com',
      public_key: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGtest...',
      private_key: '-----BEGIN OPENSSH PRIVATE KEY-----\ntest...',
      fingerprint: 'SHA256:test123...',
      created_at: '2024-01-01T00:00:00Z'
    }

    mockInvoke.mockResolvedValue(mockGeneratedKey)

    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    // Fill form
    await wrapper.find('input[name="name"]').setValue('Test Key')
    await wrapper.find('input[name="comment"]').setValue('test@example.com')

    // Submit form
    await wrapper.find('[data-testid="generate-key-btn"]').trigger('click')

    // Wait for generation to complete
    await wrapper.vm.$nextTick()

    // Form should be reset
    expect((wrapper.find('input[name="name"]').element as HTMLInputElement).value).toBe('')
    expect((wrapper.find('input[name="comment"]').element as HTMLInputElement).value).toBe('')
  })

  it('shows advanced options when toggled', async () => {
    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    const advancedToggle = wrapper.find('[data-testid="advanced-options-toggle"]')
    if (advancedToggle.exists()) {
      await advancedToggle.trigger('click')
      
      expect(wrapper.find('input[name="passphrase"]').exists()).toBe(true)
    }
  })

  it('includes passphrase when provided', async () => {
    const wrapper = mount(KeyGenerator, {
      global: {
        plugins: [router]
      }
    })

    // Fill form with passphrase
    await wrapper.find('input[name="name"]').setValue('Test Key')
    await wrapper.find('input[name="comment"]').setValue('test@example.com')
    
    // Enable advanced options if needed
    const advancedToggle = wrapper.find('[data-testid="advanced-options-toggle"]')
    if (advancedToggle.exists()) {
      await advancedToggle.trigger('click')
      await wrapper.find('input[name="passphrase"]').setValue('secret-passphrase')
    }

    await wrapper.find('[data-testid="generate-key-btn"]').trigger('click')

    expect(mockInvoke).toHaveBeenCalledWith('generate_ssh_key', {
      params: expect.objectContaining({
        passphrase: 'secret-passphrase'
      })
    })
  })
})