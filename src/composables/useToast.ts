import { reactive, readonly } from 'vue'

export type ToastLevel = 'success' | 'error' | 'info'

export interface ToastItem {
    id: number
    type: ToastLevel
    text: string
}

const toastQueue = reactive<ToastItem[]>([])
let idCounter = 1

const removeById = (id: number) => {
    const index = toastQueue.findIndex(t => t.id === id)
    if (index !== -1) toastQueue.splice(index, 1)
}

const showInternal = (type: ToastLevel, text: string, durationMs = 2500) => {
    const id = idCounter++
    toastQueue.push({ id, type, text })
    if (durationMs > 0) {
        window.setTimeout(() => removeById(id), durationMs)
    }
    return id
}

export function useToast() {
    const show = (type: ToastLevel, text: string, durationMs?: number) => showInternal(type, text, durationMs)
    const success = (text: string, durationMs?: number) => showInternal('success', text, durationMs)
    const error = (text: string, durationMs?: number) => showInternal('error', text, durationMs)
    const info = (text: string, durationMs?: number) => showInternal('info', text, durationMs)

    return { show, success, error, info }
}

export function useToastQueue() {
    return { toasts: readonly(toastQueue) }
}

export function removeToast(id: number) {
    removeById(id)
}


