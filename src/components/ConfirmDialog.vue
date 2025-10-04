<template>
    <!-- 确认对话框 -->
    <div v-if="visible" class="fixed inset-0 z-50 overflow-y-auto">
        <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
            <div class="fixed inset-0 transition-opacity">
                <div class="absolute inset-0 bg-gray-500 opacity-75"></div>
            </div>

            <span class="hidden sm:inline-block sm:align-middle sm:h-screen"></span>&#8203;
            <div
                class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
                <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                    <div class="sm:flex sm:items-start">
                        <!-- 图标 -->
                        <div
                            class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 bg-red-100 rounded-full sm:mx-0 sm:h-10 sm:w-10">
                            <svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                                xmlns="www.w3.org/2000/svg">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z">
                                </path>
                            </svg>
                        </div>
                        <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                            <h3 class="text-lg leading-6 font-medium text-gray-900">
                                {{ title }}
                            </h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500">
                                    {{ message }}
                                </p>
                            </div>

                            <!-- 密码输入框 (可选) -->
                            <div v-if="requirePassword" class="mt-4">
                                <label class="block text-sm font-medium text-gray-700 mb-2">
                                    {{ computedPasswordLabel }}
                                </label>
                                <input v-model="localPassword" type="password"
                                    :placeholder="computedPasswordPlaceholder"
                                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                                    :class="{ 'border-red-500': passwordError }" />
                                <p v-if="passwordError" class="mt-1 text-sm text-red-600">
                                    {{ passwordError }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
                    <button type="button"
                        class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
                        @click="handleConfirm">
                        {{ computedConfirmText }}
                    </button>
                    <button type="button"
                        class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
                        @click="handleCancel">
                        {{ computedCancelText }}
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface Props {
    visible: boolean
    title: string
    message: string
    confirmButtonText?: string
    cancelButtonText?: string
    requirePassword?: boolean
    passwordLabel?: string
    passwordPlaceholder?: string
}

interface Emits {
    (e: 'confirm', password?: string): void
    (e: 'cancel'): void
    (e: 'update:visible', value: boolean): void
}

const props = withDefaults(defineProps<Props>(), {
    confirmButtonText: '删除',
    cancelButtonText: '取消',
    requirePassword: false,
    passwordLabel: '主密码',
    passwordPlaceholder: '请输入密码'
})

const emit = defineEmits<Emits>()

// 本地密码状态
const localPassword = ref('')
const passwordError = ref('')

// 国际化文本计算属性
const computedConfirmText = computed(() =>
    props.confirmButtonText === '删除' ? t('common.delete') : props.confirmButtonText
)

const computedCancelText = computed(() =>
    props.cancelButtonText === '取消' ? t('common.cancel') : props.cancelButtonText
)

const computedPasswordLabel = computed(() =>
    props.passwordLabel === '主密码' ? t('auth.masterPassword') : props.passwordLabel
)

const computedPasswordPlaceholder = computed(() =>
    props.passwordPlaceholder === '请输入密码' ? t('auth.loginPlaceholder') : props.passwordPlaceholder
)

// 监听密码变化，清除错误信息
watch(localPassword, () => {
    passwordError.value = ''
})

// 处理确认
const handleConfirm = () => {
    if (props.requirePassword) {
        // 验证密码长度
        if (localPassword.value.length < 8) {
            passwordError.value = t('auth.errors.passwordLength')
            return
        }

        // 发出带密码的确认事件
        emit('confirm', localPassword.value)
        // 清空密码
        localPassword.value = ''
    } else {
        // 不需要密码的确认
        emit('confirm')
    }
}

// 处理取消
const handleCancel = () => {
    emit('cancel')
    // 清空密码和错误信息
    localPassword.value = ''
    passwordError.value = ''
}

// 密码验证错误的外部设置方法
const setPasswordError = (error: string) => {
    passwordError.value = error
}

// 暴露方法给父组件使用
defineExpose({
    setPasswordError,
    clearPassword: () => {
        localPassword.value = ''
        passwordError.value = ''
    }
})
</script>
