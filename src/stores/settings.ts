import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { appDataDir, join } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'

// Persistent keys
const ENABLED_KEY = 'autoExportEnabled'
const DIR_KEY = 'autoExportDir'

export const useSettingsStore = defineStore('settings', () => {
    const autoExportEnabled = ref<boolean>((() => {
        const raw = localStorage.getItem(ENABLED_KEY)
        return raw === 'true'
    })())

    const autoExportDir = ref<string | null>((() => {
        return localStorage.getItem(DIR_KEY)
    })())

    const isAutoExportEnabled = computed(() => autoExportEnabled.value)
    const exportDirectory = computed(() => autoExportDir.value)

    const setAutoExportEnabled = (enabled: boolean) => {
        autoExportEnabled.value = enabled
        localStorage.setItem(ENABLED_KEY, String(enabled))
    }

    const setAutoExportDir = (dir: string | null) => {
        autoExportDir.value = dir
        if (dir) {
            localStorage.setItem(DIR_KEY, dir)
        } else {
            localStorage.removeItem(DIR_KEY)
        }
    }

    // Resolve effective export directory; default to app data dir
    const getEffectiveExportDir = async (): Promise<string> => {
        const base = exportDirectory.value || await appDataDir()
        // Ensure the directory exists on disk
        await invoke<boolean>('ensure_dir_exists', { dirPath: base })
        return base
    }

    // Resolve a default file path inside the export directory
    const getDefaultExportFilePath = async (): Promise<string> => {
        const dir = await getEffectiveExportDir()
        return await join(dir, 'ssh_keys.json')
    }

    return {
        // state
        isAutoExportEnabled,
        exportDirectory,
        // actions
        setAutoExportEnabled,
        setAutoExportDir,
        getEffectiveExportDir,
        getDefaultExportFilePath,
    }
})


