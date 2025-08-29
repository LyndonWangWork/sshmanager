<template>
  <div class="space-y-2">
    <label
      v-if="label"
      :for="inputId"
      class="block text-sm font-medium text-tech-700 mb-1 transition-colors"
    >
      {{ label }}
      <span v-if="required" class="text-error-500 ml-1">*</span>
    </label>
    
    <div class="relative">
      <!-- 前置图标 -->
      <div v-if="prefixIcon" class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <component :is="prefixIcon" class="h-5 w-5 text-tech-400" />
      </div>
      
      <input
        :id="inputId"
        :value="modelValue || ''"
        @input="updateValue"
        @focus="focused = true"
        @blur="focused = false"
        :type="type"
        :required="required"
        :disabled="disabled"
        :placeholder="placeholder"
        :class="inputClasses"
        :readonly="readonly"
      />
      
      <!-- 清除按钮 -->
      <button
        v-if="showClearButton && modelValue && !disabled && !readonly"
        @click="clearInput"
        type="button"
        class="absolute inset-y-0 right-0 pr-3 flex items-center"
        :class="{ 'pr-10': suffixIcon || loading }"
      >
        <svg class="h-5 w-5 text-tech-400 hover:text-tech-600 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
      
      <!-- 后置图标 -->
      <div v-if="suffixIcon && !showClearButton" class="absolute inset-y-0 right-0 pr-3 flex items-center">
        <component :is="suffixIcon" class="h-5 w-5 text-tech-400" />
      </div>
      
      <!-- 加载状态 -->
      <div v-if="loading" class="absolute inset-y-0 right-0 pr-3 flex items-center">
        <div class="loading-spinner w-4 h-4"></div>
      </div>
    </div>
    
    <!-- 错误信息 -->
    <div v-if="error" class="flex items-center mt-2 animate-slide-down">
      <svg class="w-4 h-4 text-error-500 mr-1" fill="currentColor" viewBox="0 0 20 20">
        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
      </svg>
      <p class="text-sm text-error-600">{{ error }}</p>
    </div>
    
    <!-- 提示信息 -->
    <p v-else-if="hint" class="mt-1 text-sm text-tech-500 animate-fade-in">
      {{ hint }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, useId } from 'vue'
import type { Component } from 'vue'

interface Props {
  modelValue: string | number | undefined
  label?: string
  type?: string
  required?: boolean
  disabled?: boolean
  readonly?: boolean
  loading?: boolean
  placeholder?: string
  error?: string
  hint?: string
  prefixIcon?: Component
  suffixIcon?: Component
  size?: 'sm' | 'md' | 'lg'
  showClearButton?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  required: false,
  disabled: false,
  readonly: false,
  loading: false,
  size: 'md',
  showClearButton: true
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number | undefined]
  focus: [event: FocusEvent]
  blur: [event: FocusEvent]
}>()

const inputId = useId()
const focused = ref(false)

const updateValue = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = target.value
  
  // 如果是数字类型输入框，尝试转换为数字
  if (props.type === 'number') {
    const numValue = parseFloat(value)
    emit('update:modelValue', isNaN(numValue) ? value : numValue)
  } else {
    emit('update:modelValue', value)
  }
}

const clearInput = () => {
  emit('update:modelValue', props.type === 'number' ? 0 : '')
}

const inputClasses = computed(() => {
  const baseClasses = 'input transition-all duration-300'
  const sizeClasses = {
    sm: 'px-3 py-2 text-sm',
    md: 'px-4 py-3 text-base',
    lg: 'px-5 py-4 text-lg'
  }
  
  let classes = [baseClasses, sizeClasses[props.size]]
  
  // 前置图标时增加左内边距
  if (props.prefixIcon) {
    classes.push('pl-10')
  }
  
  // 后置图标、加载状态或清除按钮时增加右内边距
  if (props.suffixIcon || props.loading || props.showClearButton) {
    classes.push('pr-10')
  }
  
  if (props.error) {
    classes.push('border-error-300 focus:border-error-500 focus:ring-error-500 bg-error-50/50')
  } else if (focused.value) {
    classes.push('ring-2 ring-primary-500 border-primary-500 shadow-glow')
  }
  
  if (props.disabled) {
    classes.push('bg-tech-100 cursor-not-allowed opacity-60')
  }
  
  if (props.readonly) {
    classes.push('bg-tech-50 cursor-default')
  }
  
  return classes.join(' ')
})
</script>
```