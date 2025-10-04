# ConfirmDialog 组件使用指南

## 概述

`ConfirmDialog` 是一个通用的确认对话框组件，用于替换项目中重复的确认对话框代码。支持简单的确认操作和需要密码确认的操作。

## 特性

- 📱 响应式设计，支持移动端和桌面端
- 🔐 支持可选密码输入验证
- 🌐 多语言支持（通过i18n）
- 🎨 一致的设计风格
- 🚀 易于使用和配置

## 组件接口

### Props

| 属性名                | 类型      | 默认值         | 描述             |
| --------------------- | --------- | -------------- | ---------------- |
| `visible`             | `boolean` | -              | 是否显示对话框   |
| `title`               | `string`  | -              | 对话框标题       |
| `message`             | Sstring`  | -              | 确认消息内容     |
| `confirmButtonText`   | `string`  | `'删除'`       | 确认按钮文本     |
| `cancelButtonText`    | `string`  | `'取消'`       | 取消按钮文本     |
| `requirePassword`     | `boolean` | `false`        | 是否需要密码输入 |
| `passwordLabel`       | `string`  | `'主密码'`     | 密码输入框标签   |
| `passwordPlaceholder` | `string`  | `'请输入密码'` | 密码输入框占位符 |

### Events

| 事件名    | 参数                | 描述                               |
| --------- | ------------------- | ---------------------------------- |
| `confirm` | `password?: string` | 用户点击确认时触发，带可选密码参数 |
| `cancel`  | -                   | 用户点击取消时触发                 |

### Exposed Methods

| 方法名             | 参数            | 描述                   |
| ------------------ | --------------- | ---------------------- |
| `setPasswordError` | `error: string` | 设置密码输入错误信息   |
| `clearPassword`    | -               | 清空密码输入和错误信息 |

## 使用示例

### 1. 基础删除确认

```vue
<template>
  <div>
    <button @click="showConfirm = true">删除项目</button>
    
    <ConfirmDialog 
      :visible="showConfirm"
      title="删除项目"
      message="确定要删除此项目吗？此操作无法撤销。"
      @confirm="handleConfirm"
      @cancel=" handleCancel"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'

const showConfirm = ref(false)

const handleConfirm = () => {
  // 执行删除操作
  console.log('项目已删除')
  showConfirm.value = false
}

const handleCancel = () => {
  showConfirm.value = false
}
</script>
```

### 2. 需要密码确认的重置操作

```vue
<template>
  <div>
    <button @click="showConfirm = true">重置所有数据</button>
    
    <ConfirmDialog 
      :visible="showConfirm"
      title="重置应用"
      message="确定要重置应用吗？所有数据将被清除，此操作无法撤销。"
      confirm-button-text="重置"
      :require-password="true"
      password-label="主密码"
      password-placeholder="请输入您的主密码"
      @confirm="handleResetConfirm"
      @cancel="handleCancel"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'

const showConfirm = ref(false)

const handleResetConfirm = async (password) => {
  try {
    // 验证密码并执行重置
    await resetData(password)
    showConfirm.value = false
  } catch (error) {
    alert('重置失败: ' + error.message)
  }
}

const handleCancel = () => {
  showConfirm.value = false
}
</script>
```

## 项目中已使用的位置

### ConfigEditor.vue - SSH主机删除确认

```vue
<ConfirmDialog 
  :visible="showDeleteConfirm"
  :title="$t('configEditor.hostConfig.title')"
  :message="$t('configEditor.hostConfig.deleteConfirm')"
  @confirm="confirmDeleteHost"
  @cancel="cancelDeleteHost"
/>
```

### Settings.vue - 应用重置确认

```vue
<ConfirmDialog 
  :visible="showResetConfirm"
  :title="$t('settings.reset.title')"
  :message="$t('settings.reset.confirm')"
  :confirm-button-text="$t('settings.reset.button')"
  :require-password="true"
  @confirm="handleResetConfirm"
  @cancel="handleResetCancel"
/>
```

### KeyCard.vue - 密钥删除确认

```vue
<ConfirmDialog 
  :visible="showDeleteConfirm"
  :title="$t('keyCard.actions.deleteKey')"
  :message="$t('keyCard.confirmDelete', { name: keyData.name })"
  @confirm="confirmDelete"
  @cancel="cancelDelete"
/>
```

## 优化效果

### 代码统一性
- ✅ 所有删除确认对话框保持一致的外观和交互
- ✅ 统一的错误处理方式
- ✅ 一致的按钮样式和布局

### 用户体验提升
- ✅ 美观的自定义模态对话框替代浏览器原生确认框
- ✅ 更好的响应式设计
- ✅ 支持键盘导航和快捷键

### 维护性提升
- ✅ 集中管理确认对话框的样式和行为
- ✅ 更容易添加新功能和修改现有功能
- ✅ 统一的国际化支持

## 注意事项

1. **密码验证**: 密码验证需要在父组件中实现，组件只提供基本的长度检查
2. **错误处理**: 可以通过组件引用调用`setPasswordError`方法显示错误信息
3. **状态清理**: 组件会自动处理密码输入的状态清理
4. **性能优化**: 建议在需要时再渲染ConfirmDialog，避免不必要的DOM操作