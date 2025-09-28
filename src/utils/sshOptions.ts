// 常见的 OpenSSH Host 级别配置选项（大小写按官方文档首字母大写惯例）
// 仅列出常用与较为通用的键，用户仍可在原始编辑器中进行高级自定义
export const COMMON_SSH_HOST_OPTIONS: string[] = [
    'HostName',
    'User',
    'Port',
    'IdentityFile',
    'ProxyJump',
    'ProxyCommand',
    'ForwardAgent',
    'ForwardX11',
    'StrictHostKeyChecking',
    'UserKnownHostsFile',
    'PreferredAuthentications',
    'ServerAliveInterval',
    'ServerAliveCountMax',
    'Compression',
    'IdentitiesOnly',
    'AddKeysToAgent',
    'ControlMaster',
    'ControlPath',
    'ControlPersist',
    'LocalForward',
    'RemoteForward',
    'SendEnv',
    'SetEnv',
    'LogLevel',
]

// 选项元数据：用于渲染控件类型
export type SshOptionType = 'text' | 'boolean' | 'enum'
export interface SshOptionSpec {
    type: SshOptionType
    values?: string[]
}

// 常见布尔/枚举型选项的规范（值一律使用小写，便于匹配与生成）
export const SSH_OPTION_SPECS: Record<string, SshOptionSpec> = {
    ForwardAgent: { type: 'boolean' },
    ForwardX11: { type: 'boolean' },
    Compression: { type: 'boolean' },
    IdentitiesOnly: { type: 'boolean' },

    StrictHostKeyChecking: { type: 'enum', values: ['yes', 'no', 'accept-new'] },
    AddKeysToAgent: { type: 'enum', values: ['yes', 'no', 'confirm'] },
    ControlMaster: { type: 'enum', values: ['yes', 'no', 'auto'] },
    LogLevel: { type: 'enum', values: ['quiet', 'error', 'info', 'verbose', 'debug'] },
}

// 工具：根据现有 other_options 生成完整键集合（不存在的填空字符串）
export function buildFullOptionsFrom(
    existing: Record<string, string> | undefined,
    extraKeys: string[] = []
): Array<{ key: string; value: string }> {
    const entries: Array<{ key: string; value: string }> = []
    const seen = new Set<string>()

    // 预定义的常见选项
    for (const k of COMMON_SSH_HOST_OPTIONS) {
        seen.add(k)
        entries.push({ key: k, value: (existing && existing[k]) || '' })
    }

    // 任何额外需要展示的 key（去重）
    for (const k of extraKeys) {
        if (!seen.has(k)) {
            seen.add(k)
            entries.push({ key: k, value: (existing && existing[k]) || '' })
        }
    }

    // 将 existing 中不在常量列表的键追加到末尾，避免丢失
    if (existing) {
        for (const [k, v] of Object.entries(existing)) {
            if (!seen.has(k)) {
                entries.push({ key: k, value: v })
            }
        }
    }

    return entries
}

// 工具：将键值数组还原为对象，并移除空键或空值
export function compactOptions(
    arr: Array<{ key: string; value: string }>
): Record<string, string> {
    const out: Record<string, string> = {}
    for (const { key, value } of arr) {
        const k = (key || '').trim()
        const v = (value || '').trim()
        if (k && v) {
            out[k] = v
        }
    }
    return out
}


