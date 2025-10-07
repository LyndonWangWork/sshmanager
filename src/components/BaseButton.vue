<template>
  <button :class="buttonClasses" :disabled="disabled || loading" @click="$emit('click', $event)" :type="type">
    <div v-if="loading" class="loading-spinner w-4 h-4 mr-2"></div>
    <div v-else-if="quickAction" class="btn-content">
      <component v-if="icon" :is="icon" class="btn-icon" />
      <span class="btn-text">
        <slot></slot>
      </span>
    </div>
    <template v-else>
      <component v-if="icon" :is="icon" class="w-5 h-5 mr-2" />
      <slot></slot>
    </template>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Component } from 'vue'

interface Props {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost' | 'outline'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  loading?: boolean
  icon?: Component
  type?: 'button' | 'submit' | 'reset'
  fullWidth?: boolean
  quickAction?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false,
  type: 'button',
  fullWidth: false,
  quickAction: false,
})

defineEmits<{
  click: [event: MouseEvent]
}>()

const buttonClasses = computed(() => {
  const base = 'btn'
  const variant = `btn-${props.variant}`
  const size = props.quickAction ? 'btn-quick-action' : `btn-${props.size}`
  const fullWidth = props.fullWidth ? 'w-full' : ''
  const disabled = props.disabled || props.loading ? 'opacity-50 cursor-not-allowed' : ''

  return [base, variant, size, fullWidth, disabled].filter(Boolean).join(' ')
})
</script>