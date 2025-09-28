<template>
    <div v-if="show" class="fixed bottom-4 right-4 z-50">
        <div class="text-white px-4 py-3 rounded-lg shadow-lg text-sm" :class="typeClass" role="status"
            aria-live="polite">
            <slot>{{ text }}</slot>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onUnmounted, watch } from 'vue'

interface Props {
    show: boolean
    type?: 'success' | 'error' | 'info'
    text?: string
    duration?: number
}

const props = withDefaults(defineProps<Props>(), {
    type: 'info',
    text: '',
    duration: 2500
})

const emit = defineEmits<{
    (e: 'update:show', value: boolean): void
}>()

let timer: number | null = null

const clearTimer = () => {
    if (timer) {
        clearTimeout(timer)
        timer = null
    }
}

watch(
    () => props.show,
    (visible) => {
        clearTimer()
        if (visible && props.duration > 0) {
            timer = window.setTimeout(() => emit('update:show', false), props.duration)
        }
    },
    { immediate: true }
)

onUnmounted(clearTimer)

const typeClass = computed(() => {
    switch (props.type) {
        case 'success':
            return 'bg-green-600'
        case 'error':
            return 'bg-red-600'
        case 'info':
        default:
            return 'bg-gray-800'
    }
})
</script>
