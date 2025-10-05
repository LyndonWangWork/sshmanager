import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { SshKeyPair, KeyGenerationParams } from '@/types'
import { useSettingsStore } from '@/stores/settings'
import { join } from '@tauri-apps/api/path'

export const useKeyStore = defineStore('key', () => {
  const keys = ref<SshKeyPair[]>([])
  const selectedKeyId = ref<string | null>(null)
  const isLoading = ref(false)
  const settingsStore = useSettingsStore()

  const selectedKey = computed(() =>
    keys.value.find(key => key.id === selectedKeyId.value)
  )

  // 加载所有密钥
  const loadKeys = async (): Promise<void> => {
    try {
      isLoading.value = true
      const result = await invoke<SshKeyPair[]>('get_all_keys')
      keys.value = result
    } catch (error) {
      console.error('加载密钥失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 生成新密钥
  const generateKey = async (params: KeyGenerationParams): Promise<SshKeyPair | null> => {
    try {
      isLoading.value = true
      const newKey = await invoke<SshKeyPair>('generate_ssh_key', { params })
      keys.value.push(newKey)
      await maybeAutoExport()
      return newKey
    } catch (error) {
      console.error('生成密钥失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 删除密钥
  const deleteKey = async (keyId: string): Promise<boolean> => {
    try {
      const result = await invoke<boolean>('delete_key', { keyId })
      if (result) {
        keys.value = keys.value.filter(key => key.id !== keyId)
        if (selectedKeyId.value === keyId) {
          selectedKeyId.value = null
        }
        await maybeAutoExport()
      }
      return result
    } catch (error) {
      console.error('删除密钥失败:', error)
      return false
    }
  }

  // 导出密钥
  const exportKey = async (keyId: string, exportPath?: string): Promise<boolean> => {
    try {
      if (exportPath) {
        return await invoke<boolean>('export_key', { keyId, exportPath })
      } else {
        // 使用文件选择器
        // TODO: 实现文件选择器集成
        console.log('导出密钥:', keyId)
        return true
      }
    } catch (error) {
      console.error('导出密钥失败:', error)
      return false
    }
  }

  // 导入密钥
  const importKeys = async (keysData: string): Promise<SshKeyPair[]> => {
    try {
      const importedKeys = await invoke<SshKeyPair[]>('import_keys', { keysData })
      // 更新本地密钥列表
      keys.value = [...keys.value, ...importedKeys]
      await maybeAutoExport()
      return importedKeys
    } catch (error) {
      console.error('导入密钥失败:', error)
      throw error
    }
  }

  // 导出所有密钥
  const exportAllKeys = async (): Promise<string> => {
    try {
      return await invoke<string>('export_all_keys')
    } catch (error) {
      console.error('导出所有密钥失败:', error)
      throw error
    }
  }

  // 设置选中的密钥
  const setSelectedKey = (id: string | null) => {
    selectedKeyId.value = id
  }

  // 清除所有密钥
  const clearKeys = () => {
    keys.value = []
    selectedKeyId.value = null
  }

  // 自动导出所有密钥到配置目录
  const maybeAutoExport = async () => {
    try {
      if (!settingsStore.isAutoExportEnabled) return
      const exportJson = await invoke<string>('export_all_keys')
      const dir = await settingsStore.getEffectiveExportDir()
      const filePath = await join(dir, 'ssh_keys.json')
      await invoke<boolean>('write_file_content', { filePath, content: exportJson })
    } catch (e) {
      console.error('自动导出失败:', e)
    }
  }

  return {
    // State
    keys: computed(() => keys.value),
    selectedKey,
    isLoading: computed(() => isLoading.value),

    // Actions
    loadKeys,
    generateKey,
    deleteKey,
    exportKey,
    importKeys,
    exportAllKeys,
    setSelectedKey,
    clearKeys,
  }
})