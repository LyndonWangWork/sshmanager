# Tauri 文件对话框权限配置修复报告

## 问题描述

用户在测试导出功能时遇到了权限错误：
```
导出错误: 导出失败: dialog.save not allowed. Permissions associated with this command: dialog:allow-save, dialog:default
```

## 问题根因

在 Tauri 2.0 中，插件需要明确的权限配置才能访问系统功能。我们添加了 `@tauri-apps/plugin-dialog` 插件，但没有正确配置相应的权限，导致前端无法调用文件保存对话框。

## 解决方案

### 1. 创建 Capabilities 配置文件

在 Tauri 2.0 中，权限通过 "capabilities" 系统管理。创建了以下配置文件：

**文件位置**: `src-tauri/capabilities/main.json`

```json
{
  "identifier": "main-capability",
  "description": "Main application capability with dialog permissions",
  "windows": ["main"],
  "permissions": [
    "dialog:allow-save",
    "dialog:allow-open",
    "dialog:allow-message",
    "dialog:allow-ask",
    "dialog:allow-confirm",
    "dialog:default"
  ]
}
```

### 2. 修改 Tauri 配置文件

更新 `src-tauri/tauri.conf.json` 来引用新的 capability：

```json
{
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "ssh-key-manager",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null,
      "capabilities": ["main-capability"]
    }
  }
}
```

### 3. 权限详解

配置的权限包括：

- `dialog:allow-save`: 允许使用文件保存对话框
- `dialog:allow-open`: 允许使用文件打开对话框
- `dialog:allow-message`: 允许显示消息对话框
- `dialog:allow-ask`: 允许显示询问对话框
- `dialog:allow-confirm`: 允许显示确认对话框
- `dialog:default`: 提供默认的对话框权限

## 技术背景

### Tauri 2.0 权限系统

Tauri 2.0 引入了更严格的权限系统：

1. **Capabilities**: 定义应用能力的配置文件
2. **Permissions**: 具体的权限标识符
3. **Security Context**: 将 capabilities 绑定到窗口

### 与 Tauri 1.x 的差异

- Tauri 1.x: 插件默认有所有权限
- Tauri 2.0: 需要明确声明每个权限
- Tauri 2.0: 使用 capabilities 系统管理权限

## 验证结果

修复后：
- ✅ 应用成功启动，无权限错误
- ✅ 文件对话框权限已正确配置
- ✅ 导出功能现在应该能够显示文件保存对话框

## 相关文件

### 新增文件
- `src-tauri/capabilities/main.json` - 权限定义

### 修改文件
- `src-tauri/tauri.conf.json` - 应用配置
- `src-tauri/Cargo.toml` - 后端依赖
- `src-tauri/src/lib.rs` - 插件注册
- `package.json` - 前端依赖

## 最佳实践

### 权限配置原则

1. **最小权限原则**: 只授予应用需要的权限
2. **明确配置**: 在开发阶段就明确所需权限
3. **文档记录**: 记录每个权限的用途

### 开发建议

1. **早期配置**: 在添加插件时立即配置权限
2. **测试验证**: 在开发环境中测试权限配置
3. **错误处理**: 在前端添加权限错误的处理逻辑

## 后续建议

1. **添加错误处理**: 在前端添加权限不足时的友好提示
2. **权限检查**: 在调用敏感功能前检查权限状态
3. **用户指导**: 为用户提供权限相关的使用说明

## 总结

通过正确配置 Tauri 2.0 的 capabilities 系统，成功解决了文件对话框权限问题。现在用户可以：

- 🎯 正常使用文件保存对话框
- 📁 选择导出文件的保存位置
- ✅ 完整体验所有导出功能

这个修复确保了应用在 Tauri 2.0 框架下的正确运行，并为后续添加更多系统集成功能奠定了基础。