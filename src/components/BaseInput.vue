<template>
  <div>
    <label
      v-if="label"
      :for="inputId"
      class="block text-sm font-medium text-gray-700 mb-1"
    >
      {{ label }}
      <span v-if="required" class="text-red-500">*</span>
    </label>
    <input
      :id="inputId"
      :value="modelValue"
      @input="updateValue"
      :type="type"
      :required="required"
      :disabled="disabled"
      :placeholder="placeholder"
      :class="inputClasses"
    />
    <p v-if="error" class="mt-1 text-sm text-red-600">
      {{ error }}
    </p>
    <p v-else-if="hint" class="mt-1 text-sm text-gray-500">
      {{ hint }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed, useId } from 'vue'

interface Props {
  modelValue: string | number
  label?: string
  type?: string
  required?: boolean
  disabled?: boolean
  placeholder?: string
  error?: string
  hint?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  required: false,
  disabled: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number]
}>()

const inputId = useId()

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

const inputClasses = computed(() => {
  let classes = 'input'
  
  if (props.error) {
    classes += ' border-red-300 focus:border-red-500 focus:ring-red-500'
  }
  
  if (props.disabled) {
    classes += ' bg-gray-100 cursor-not-allowed'
  }
  
  return classes
})
</script>