import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import ImportExportDialog from '@/components/ImportExportDialog.vue'
import { useKeyStore } from '@/stores/key'

// Mock Tauri API
const mockInvoke = vi.fn()
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke
}))

describe('ImportExportDialog 导出功能', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
    
    // Mock URL.createObjectURL and related methods
    global.URL.createObjectURL = vi.fn(() => 'mock-url')
    global.URL.revokeObjectURL = vi.fn()
    
    // Mock document methods
    const mockClick = vi.fn()
    const mockAppendChild = vi.fn()
    const mockRemoveChild = vi.fn()
    
    global.document.createElement = vi.fn(() => ({
      click: mockClick,
      href: '',
      download: ''
    }))
    global.document.body.appendChild = mockAppendChild
    global.document.body.removeChild = mockRemoveChild
  })

  it('应该正确显示导出对话框', () => {
    const wrapper = mount(ImportExportDialog, {
      props: {
        show: true,
        mode: 'export',
        selectedKeys: []
      }
    })

    expect(wrapper.find('h3').text()).toBe('导出密钥')
    expect(wrapper.find('select').exists()).toBe(true)
  })

  it('应该支持选择不同的导出格式', async () => {
    const wrapper = mount(ImportExportDialog, {
      props: {
        show: true,
        mode: 'export',
        selectedKeys: []
      }
    })

    const select = wrapper.find('select')
    const options = select.findAll('option')
    
    expect(options).toHaveLength(3)
    expect(options[0].text()).toBe('JSON 格式 (.json)')
    expect(options[1].text()).toBe('OpenSSH 格式')
    expect(options[2].text()).toBe('PEM 格式')
  })

  it('应该在有密钥时启用导出按钮', () => {
    const keyStore = useKeyStore()
    keyStore.keys = [{
      id: 'test-key-1',
      name: 'Test Key',
      key_type: 'Ed25519',
      key_size: 256,
      comment: '',
      public_key: 'ssh-ed25519 AAAAC3...',
      private_key: '-----BEGIN OPENSSH PRIVATE KEY-----\n...',
      fingerprint: 'SHA256:...',
      created_at: '2025-01-26T10:00:00.000Z',
      last_used: null
    }]

    const wrapper = mount(ImportExportDialog, {
      props: {
        show: true,
        mode: 'export',
        selectedKeys: []
      }
    })

    const exportButton = wrapper.find('button:not([variant="secondary"])')
    expect(exportButton.attributes('disabled')).toBeFalsy()
  })

  it('应该显示导出预览', () => {
    const keyStore = useKeyStore()
    keyStore.keys = [{
      id: 'test-key-1',
      name: 'Test Key',
      key_type: 'Ed25519',
      key_size: 256,
      comment: '',
      public_key: 'ssh-ed25519 AAAAC3...',
      private_key: '-----BEGIN OPENSSH PRIVATE KEY-----\n...',
      fingerprint: 'SHA256:...',
      created_at: '2025-01-26T10:00:00.000Z',
      last_used: null
    }]

    const wrapper = mount(ImportExportDialog, {
      props: {
        show: true,
        mode: 'export',
        selectedKeys: []
      }
    })

    expect(wrapper.find('[class*="bg-gray-50"]').exists()).toBe(true)
    expect(wrapper.text()).toContain('导出预览')
  })

  it('应该支持私钥包含选项', async () => {
    const wrapper = mount(ImportExportDialog, {
      props: {
        show: true,
        mode: 'export',
        selectedKeys: []
      }
    })

    const checkbox = wrapper.find('input[type="checkbox"]')
    expect(checkbox.exists()).toBe(true)
    
    await checkbox.setChecked(true)
    expect(checkbox.element.checked).toBe(true)
  })
})