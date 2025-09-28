<template>
    <div class="fixed bottom-4 right-4 z-50 space-y-2">
        <div v-for="toast in toasts" :key="toast.id" class="text-white px-4 py-3 rounded-lg shadow-lg text-sm" :class="toast.type === 'success' ? 'bg-green-600' :
            toast.type === 'error' ? 'bg-red-600' :
                'bg-gray-800'
            " role="status" aria-live="polite">
            {{ toast.text }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { useToastQueue } from '@/composables/useToast'

const { toasts } = useToastQueue()
</script>
