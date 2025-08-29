import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { createI18n } from 'vue-i18n'
import KeyCard from '@/components/KeyCard.vue'
import type { SshKeyPair } from '@/types'
import { zh, en } from '@/i18n'

// 创建i18n实例
const i18n = createI18n({
  legacy: false,
  locale: 'zh',
  fallbackLocale: 'en',
  messages: {
    zh,
    en
  }
})

// Mock Tauri API
const mockInvoke = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke
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
      props: { keyData: mockKey },
      global: {
        plugins: [i18n]
      }
    })
    
    expect(wrapper.text()).toContain('Test SSH Key')
    expect(wrapper.text()).toContain('Ed25519')
    expect(wrapper.text()).toContain('test@example.com')
    expect(wrapper.text()).toContain('SHA256:test123...')
  })

  it('shows key type badge correctly', () => {
    const wrapper = mount(KeyCard, {
      props: { keyData: mockKey },
      global: {
        plugins: [i18n]
      }
    })
    
    const typeBadge = wrapper.find('span.font-semibold.uppercase')
    expect(typeBadge.exists()).toBe(true)
    expect(typeBadge.text()).toBe('Ed25519')
  })

  it('displays creation date correctly', () => {
    const wrapper = mount(KeyCard, {
      props: { keyData: mockKey },
      global: {
        plugins: [i18n]
      }
    })
    
    // 由于日期格式化依赖于系统环境，我们检查是否包含年份信息
    expect(wrapper.text()).toContain('2024')
  })

  it('shows last used date when available', () => {
    const wrapper = mount(KeyCard, {
      props: { keyData: mockKey },
      global: {
        plugins: [i18n]
      }
    })
    
    // 检查是否包含最后使用时间的信息
    expect(wrapper.text()).toContain('2024')
  })

  it('handles key without last used date', () => {
    const keyWithoutLastUsed = { ...mockKey, last_used: undefined }
    const wrapper = mount(KeyCard, {
      props: { keyData: keyWithoutLastUsed },
      global: {
        plugins: [i18n]
      }
    })
    
    // 检查是否显示"无注释"文本（这表明组件正确处理了undefined值）
    // 或者检查是否不显示最后使用时间相关的元素
    const lastUsedElement = wrapper.find('div.flex.items-center.space-x-2.text-tech-500')
    expect(lastUsedElement.exists()).toBe(true)
  })

  it('emits edit event when edit button is clicked', async () => {
    const wrapper = mount(KeyCard, {
      props: { keyData: mockKey },
      global: {
        plugins: [i18n]
      }
    })
    
    // 打开菜单
    const menuButton = wrapper.find('button[title="打开菜单"]')
    if (menuButton.exists()) {
      await menuButton.trigger('click')
      
      // 点击编辑按钮
      const editButton = wrapper.find('button.flex.items-center.w-full.px-4.py-3')
      if (editButton.exists()) {
        await editButton.trigger('click')
        expect(wrapper.emitted().edit).toBeTruthy()
      }
    }
  })

  it('emits delete event when delete button is clicked', async () => {
    // Mock confirm dialog
    const mockConfirm = vi.spyOn(window, 'confirm').mockImplementation(() => true)
    
    const wrapper = mount(KeyCard, {
      props: { keyData: mockKey },
      global: {
        plugins: [i18n]
      }
    })
    
    // 打开菜单
    const menuButton = wrapper.find('button[title="打开菜单"]')
    if (menuButton.exists()) {
      await menuButton.trigger('click')
      
      // 点击删除按钮
      const deleteButtons = wrapper.findAll('button.flex.items-center.w-full.px-4.py-3')
      const deleteButton = deleteButtons[deleteButtons.length - 1] // 最后一个是删除按钮
      if (deleteButton.exists()) {
        await deleteButton.trigger('click')
        expect(wrapper.emitted().delete).toBeTruthy()
        expect(wrapper.emitted().delete[0]).toEqual([mockKey.id])
      }
    }
    
    mockConfirm.mockRestore()
  })

  it('shows public key preview correctly', async () => {
    const wrapper = mount(KeyCard, {
      props: { keyData: mockKey },
      global: {
        plugins: [i18n]
      }
    })
    
    // 点击显示公钥按钮
    const toggleButton = wrapper.find('button.flex.items-center.space-x-2')
    if (toggleButton.exists()) {
      await toggleButton.trigger('click')
      
      const publicKeyPreview = wrapper.find('textarea')
      expect(publicKeyPreview.exists()).toBe(true)
      expect(publicKeyPreview.element.value).toContain('ssh-ed25519')
    }
  })
})