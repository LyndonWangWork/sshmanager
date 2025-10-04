import { createI18n } from 'vue-i18n'
import { getUserLanguage } from '@/utils/language'

// 中文翻译
const zh = {
  common: {
    confirm: '确认',
    select: '选择',
    cancel: '取消',
    save: '保存',
    delete: '删除',
    edit: '编辑',
    add: '添加',
    remove: '删除',
    close: '关闭',
    back: '返回',
    loading: '加载中...',
    success: '成功',
    error: '错误',
    warning: '警告',
    info: '信息',
    file: '文件',
    line: '行',
    copy: '复制'
  },
  nav: {
    dashboard: '仪表板',
    keyManager: '密钥管理',
    keyGenerator: '生成密钥',
    configEditor: '配置编辑',
    settings: '设置',
    logout: '退出登录'
  },
  dashboard: {
    title: 'SSH密钥管理器',
    subtitle: '管理您的SSH密钥和配置',
    stats: {
      totalKeys: '密钥总数',
      sshConfig: 'SSH配置',
      status: '状态',
      loaded: '已加载',
      normal: '正常'
    },
    quickActions: {
      title: '快速操作',
      generateKey: '生成新密钥',
      editConfig: '编辑SSH配置'
    }
  },
  keyGenerator: {
    title: '生成SSH密钥',
    keyInfo: {
      title: '密钥信息',
      name: '密钥名称',
      namePlaceholder: '例如：github-work',
      comment: '注释 (可选)',
      commentPlaceholder: '例如：user{\'@\'}hostname'
    },
    keyType: {
      title: '密钥类型',
      selectType: '选择类型',
      recommended: '推荐',
      keyLength: '密钥长度 (bits)',
      lengthHint: '更高的位数提供更强的安全性，但会增加计算开销'
    },
    advancedOptions: {
      title: '高级选项',
      usePassphrase: '使用密码保护私钥（推荐）',
      passphrase: '密钥密码',
      passphrasePlaceholder: '请输入一个强密码来保护私钥',
      confirmPassphrase: '确认密码',
      confirmPassphrasePlaceholder: '请再次输入密码',
      passphraseHint: '密码长度建议不少于8位，包含字母、数字和特殊字符',
      securityTip: {
        title: '安全提示',
        content: '密码保护可以防止私钥文件被盗用。即使文件被获取，没有密码也无法使用。'
      },
      realKeyGeneration: {
        title: '真实密钥生成',
        content: '本应用现在使用真实的密码学算法生成SSH密钥，生成的密钥可以直接用于SSH登录和认证。'
      }
    },
    generate: {
      button: '生成密钥',
      generating: '生成中...',
      progress: {
        title: '生成进度',
        init: '初始化随机数生成器...',
        generate: '生成密钥对...',
        fingerprint: '计算密钥指纹...',
        format: '格式化密钥...',
        complete: '密钥生成完成'
      }
    },
    result: {
      title: '生成成功',
      successMessage: '密钥生成成功',
      name: '名称',
      type: '类型',
      length: '长度',
      fingerprint: '密钥指纹',
      publicKey: '公钥',
      copyPublicKey: '复制公钥',
      copyFingerprint: '复制指纹',
      saveFile: '保存文件',
      viewAllKeys: '查看所有密钥',
      generateAnother: '再生成一个'
    },
    defaultState: {
      title: '准备生成SSH密钥',
      subtitle: '设置密钥参数后点击"生成密钥"按钮'
    },
    errors: {
      nameRequired: '请输入密钥名称',
      passphraseLength: '密码长度不能少于8位',
      passphraseConfirm: '两次输入的密码不一致'
    }
  },
  keyManager: {
    title: '密钥管理',
    generateNew: '生成新密钥',
    importKeys: '导入密钥',
    exportKeys: '导出密钥',
    stats: {
      totalKeys: '总密钥数',
      rsaKeys: 'RSA 密钥',
      ed25519Keys: 'Ed25519 密钥',
      ecdsaKeys: 'ECDSA 密钥'
    },
    search: {
      placeholder: '搜索密钥名称或注释...',
      allTypes: '所有类型',
      sortBy: {
        createdAt: '按创建时间',
        name: '按名称',
        lastUsed: '按使用时间'
      }
    },
    actions: {
      edit: '编辑',
      delete: '删除',
      export: '导出',
      copy: '复制',
      viewDetails: '查看详情'
    },
    empty: {
      noKeys: '没有找到密钥',
      noKeysCreated: '还没有创建任何SSH密钥',
      noMatching: '没有匹配搜索条件的密钥',
      generateFirst: '生成第一个密钥'
    }
  },
  keyCard: {
    actions: {
      copyPublicKey: '复制公钥',
      exportKey: '导出密钥',
      editInfo: '编辑信息',
      deleteKey: '删除密钥',
      copyFingerprint: '复制指纹'
    },
    labels: {
      fingerprint: '指纹',
      comment: '注释',
      noComment: '无注释',
      createdTime: '创建时间',
      lastUsed: '最后使用',
      publicKeyContent: '公钥内容',
      showPublicKey: '显示公钥',
      hidePublicKey: '收起公钥'
    },
    confirmDelete: '确定要删除密钥 "{name}" 吗？此操作无法撤销。'
  },
  configEditor: {
    title: 'SSH配置编辑',
    reload: '重新加载',
    saveConfig: '保存配置',
    hostConfig: {
      title: '主机配置',
      add: '添加',
      hostPattern: 'Host 模式',
      hostPatternPlaceholder: '例如：github.com 或 server-*',
      hostPatternHint: '支持通配符，如 * 和 ?',
      hostname: '主机名',
      hostnamePlaceholder: '例如：192.168.1.100 或 server.example.com',
      hostnameHint: '实际连接的主机地址',
      user: '用户名',
      userPlaceholder: '例如：root 或 ubuntu',
      port: '端口',
      portPlaceholder: '默认 22',
      identityFile: '身份文件 (私钥)',
      selectKey: '选择密钥',
      browse: '浏览',
      identityGroups: {
        softwareKeys: '软件内密钥',
        sshDirKeys: '配置目录密钥'
      },
      addOption: '添加选项',
      newHost: '新主机',
      deleteConfirm: '确定要删除这个主机配置吗？',
      noHostsMessage: '暂无主机配置',
      otherOptions: '其他选项',
      optionName: '选项名',
      optionValue: '选项值',
      deleteOption: '删除选项',
      selectHostMessage: '请选择一个主机进行配置',
      optionDescriptions: {
        HostName: '要连接的真实主机名或 IP 地址',
        User: '用于登录远程主机的用户名',
        Port: 'SSH 连接端口，默认 22',
        IdentityFile: '用于认证的私钥路径，可设置多个',
        ProxyJump: '通过中转主机进行跳转（等价于 -J）',
        ProxyCommand: '自定义代理命令，用于建立连接',
        ForwardAgent: '是否转发本地 SSH 代理，yes/no',
        ForwardX11: '是否启用 X11 转发，yes/no',
        StrictHostKeyChecking: '主机密钥校验策略（yes/no/accept-new）',
        UserKnownHostsFile: '已知主机文件路径（默认 ~/.ssh/known_hosts）',
        PreferredAuthentications: '首选认证方法，逗号分隔',
        ServerAliveInterval: '保持心跳间隔（秒）',
        ServerAliveCountMax: '心跳失败次数阈值',
        Compression: '是否启用压缩，yes/no',
        IdentitiesOnly: '只使用指定私钥进行认证，yes/no',
        AddKeysToAgent: '自动将密钥添加到 ssh-agent（yes/no/confirm）',
        ControlMaster: '复用已有连接（yes/no/auto）',
        ControlPath: '连接复用的控制套接字路径',
        ControlPersist: '连接保持时间（如 10m 或 yes）',
        LocalForward: '本地端口转发：本地端口 远端地址:端口',
        RemoteForward: '远程端口转发：远端端口 本地地址:端口',
        SendEnv: '发送到远端的环境变量（可多次指定）',
        SetEnv: '在会话中设置环境变量（KEY=VALUE）',
        LogLevel: '日志级别（QUIET/ERROR/INFO/VERBOSE/DEBUG）',
        TCPKeepAlive: '启用 TCP keepalive 机制，保持连接活跃（yes/no）'
      }
    },
    preview: {
      title: '配置预览',
      showEditor: '显示编辑器',
      hideEditor: '隐藏编辑器',
      rawPlaceholder: '# SSH 配置文件\n# 请直接编辑原始配置',
      applyChanges: '应用更改'
    },
    messages: {
      loadError: '加载配置失败:',
      saveError: '保存配置失败:',
      saveSuccess: '配置保存成功',
      copySuccess: '配置已复制到剪贴板',
      copyError: '复制失败:',
      featureNotImplemented: '功能待实现',
      fileExists: '文件已存在'
    },
    empty: '请选择一个主机配置或添加新的主机'
  },
  importExport: {
    dialog: {
      importTitle: '导入密钥',
      exportTitle: '导出密钥',
      cancel: '取消',
      importing: '导入中...',
      exporting: '导出中...',
      importAction: '导入密钥',
      exportAction: '导出密钥'
    },
    import: {
      method: {
        title: '导入方式',
        file: '从文件导入',
        text: '从文本导入'
      },
      file: {
        label: '选择密钥文件',
        placeholder: '请选择密钥文件'
      },
      text: {
        label: '粘贴密钥数据',
        placeholder: '请粘贴导出的密钥数据 (JSON格式)'
      },
      preview: {
        title: '即将导入的密钥',
        count: '个'
      }
    },
    export: {
      scope: {
        title: '导出范围',
        all: '导出所有密钥',
        selected: '导出选中的密钥'
      },
      format: {
        title: '导出格式',
        json: 'JSON 格式 (.json)',
        openssh: 'OpenSSH 格式',
        pem: 'PEM 格式',
        description: 'JSON格式：适合应用备份和恢复 | OpenSSH/PEM格式：适合系统使用'
      },
      security: {
        includePrivate: '包含私钥 (不推荐，仅在安全环境中使用)'
      },
      preview: {
        title: '导出预览',
        willExport: '将导出',
        keys: '个密钥',
        keyName: '密钥:',
        keyType: '类型:',
        publicKeyFile: '公钥文件',
        privateKeyFile: '私钥文件',
        moreKeys: '还有',
        fingerprint: '指纹:'
      }
    },
    messages: {
      noValidKeys: '文件中没有找到有效的密钥数据',
      parseError: '文件解析失败：',
      unknownError: '未知错误',
      fileReadError: '文件读取失败',
      invalidFormat: '无效的JSON文件格式。支持的格式：标准导出文件、密钥数组或单个密钥对象',
      importSuccess: '成功导入',
      exportSuccess: '密钥已成功导出到:',
      importError: '导入失败:',
      exportError: '导出失败:',
      noImportData: '没有可导入的数据',
      parseWarning: '解析成功，但有',
      invalidKeysIgnored: '个无效密钥被忽略'
    }
  },
  settings: {
    title: '设置',
    appSettings: '应用设置',
    developing: '功能开发中',
    language: {
      title: '语言设置',
      select: '选择语言'
    },
    reset: {
      title: '重置应用',
      description: '清除所有数据，包括主密码、所有密钥和当前语言设置。此操作无法撤销。',
      button: '重置应用',
      success: '数据已成功重置',
      confirm: '确定要重置应用吗？所有数据将被清除，包括主密码、所有密钥和当前语言设置。此操作无法撤销。'
    }
  },
  auth: {
    setup: {
      title: '初始化应用',
      subtitle: '设置主密码来保护您的SSH密钥',
      button: '初始化应用',
      initializing: '初始化中...'
    },
    login: {
      title: '解锁应用',
      subtitle: '输入主密码来访问您的密钥',
      button: '解锁',
      loggingIn: '登录中...'
    },
    masterPassword: '主密码',
    confirmPassword: '确认密码',
    setupPlaceholder: '请设置一个强密码',
    loginPlaceholder: '请输入主密码',
    confirmPlaceholder: '请再次输入密码',
    securityTips: {
      title: '重要提示',
      items: [
        '请妥善保管您的主密码，一旦丢失无法找回',
        '建议使用包含大小写字母、数字和特殊字符的强密码',
        '密码长度建议不少于12位'
      ]
    },
    errors: {
      passwordLength: '密码长度不能少于8位',
      passwordMismatch: '两次输入的密码不一致',
      initializationFailed: '初始化失败',
      wrongPassword: '密码错误',
      operationFailed: '操作失败，请稍后重试'
    }
  }
}

// 英文翻译
const en = {
  common: {
    confirm: 'Confirm',
    select: 'Select',
    cancel: 'Cancel',
    save: 'Save',
    delete: 'Delete',
    edit: 'Edit',
    add: 'Add',
    remove: 'Remove',
    close: 'Close',
    back: 'Back',
    loading: 'Loading...',
    success: 'Success',
    error: 'Error',
    warning: 'Warning',
    info: 'Info',
    file: 'File',
    line: 'Line',
    copy: 'Copy'
  },
  nav: {
    dashboard: 'Dashboard',
    keyManager: 'Key Manager',
    keyGenerator: 'Generate Key',
    configEditor: 'Config Editor',
    settings: 'Settings',
    logout: 'Logout'
  },
  dashboard: {
    title: 'SSH Key Manager',
    subtitle: 'Manage your SSH keys and configurations',
    stats: {
      totalKeys: 'Total Keys',
      sshConfig: 'SSH Config',
      status: 'Status',
      loaded: 'Loaded',
      normal: 'Normal'
    },
    quickActions: {
      title: 'Quick Actions',
      generateKey: 'Generate New Key',
      editConfig: 'Edit SSH Config'
    }
  },
  keyGenerator: {
    title: 'Generate SSH Key',
    keyInfo: {
      title: 'Key Information',
      name: 'Key Name',
      namePlaceholder: 'e.g., github-work',
      comment: 'Comment (Optional)',
      commentPlaceholder: 'e.g., user{\'@\'}hostname'
    },
    keyType: {
      title: 'Key Type',
      selectType: 'Select Type',
      recommended: 'Recommended',
      keyLength: 'Key Length (bits)',
      lengthHint: 'Higher bits provide stronger security but increase computational overhead'
    },
    advancedOptions: {
      title: 'Advanced Options',
      usePassphrase: 'Protect private key with passphrase (Recommended)',
      passphrase: 'Key Passphrase',
      passphrasePlaceholder: 'Enter a strong passphrase to protect the private key',
      confirmPassphrase: 'Confirm Passphrase',
      confirmPassphrasePlaceholder: 'Enter the passphrase again',
      passphraseHint: 'Passphrase should be at least 8 characters long and include letters, numbers, and special characters',
      securityTip: {
        title: 'Security Tip',
        content: 'Passphrase protection prevents private key misuse. Even if the file is stolen, it cannot be used without the passphrase.'
      },
      realKeyGeneration: {
        title: 'Real Key Generation',
        content: 'This application now uses real cryptographic algorithms to generate SSH keys that can be directly used for SSH login and authentication.'
      }
    },
    generate: {
      button: 'Generate Key',
      generating: 'Generating...',
      progress: {
        title: 'Generation Progress',
        init: 'Initializing random number generator...',
        generate: 'Generating key pair...',
        fingerprint: 'Calculating key fingerprint...',
        format: 'Formatting key...',
        complete: 'Key generation complete'
      }
    },
    result: {
      title: 'Generation Successful',
      successMessage: 'Key generated successfully',
      name: 'Name',
      type: 'Type',
      length: 'Length',
      fingerprint: 'Key Fingerprint',
      publicKey: 'Public Key',
      copyPublicKey: 'Copy Public Key',
      copyFingerprint: 'Copy Fingerprint',
      saveFile: 'Save File',
      viewAllKeys: 'View All Keys',
      generateAnother: 'Generate Another'
    },
    defaultState: {
      title: 'Ready to Generate SSH Key',
      subtitle: 'Set key parameters and click "Generate Key" button'
    },
    errors: {
      nameRequired: 'Please enter key name',
      passphraseLength: 'Passphrase must be at least 8 characters long',
      passphraseConfirm: 'Passphrases do not match'
    }
  },
  keyManager: {
    title: 'Key Management',
    generateNew: 'Generate New Key',
    importKeys: 'Import Keys',
    exportKeys: 'Export Keys',
    stats: {
      totalKeys: 'Total Keys',
      rsaKeys: 'RSA Keys',
      ed25519Keys: 'Ed25519 Keys',
      ecdsaKeys: 'ECDSA Keys'
    },
    search: {
      placeholder: 'Search key name or comment...',
      allTypes: 'All Types',
      sortBy: {
        createdAt: 'By Creation Time',
        name: 'By Name',
        lastUsed: 'By Last Used'
      }
    },
    actions: {
      edit: 'Edit',
      delete: 'Delete',
      export: 'Export',
      copy: 'Copy',
      viewDetails: 'View Details'
    },
    empty: {
      noKeys: 'No keys found',
      noKeysCreated: 'No SSH keys have been created yet',
      noMatching: 'No keys match the search criteria',
      generateFirst: 'Generate First Key'
    }
  },
  keyCard: {
    actions: {
      copyPublicKey: 'Copy Public Key',
      exportKey: 'Export Key',
      editInfo: 'Edit Info',
      deleteKey: 'Delete Key',
      copyFingerprint: 'Copy Fingerprint'
    },
    labels: {
      fingerprint: 'Fingerprint',
      comment: 'Comment',
      noComment: 'No Comment',
      createdTime: 'Created',
      lastUsed: 'Last Used',
      publicKeyContent: 'Public Key Content',
      showPublicKey: 'Show Public Key',
      hidePublicKey: 'Hide Public Key'
    },
    confirmDelete: 'Are you sure you want to delete key "{name}"? This action cannot be undone.'
  },
  configEditor: {
    title: 'SSH Config Editor',
    reload: 'Reload',
    saveConfig: 'Save Config',
    hostConfig: {
      title: 'Host Configuration',
      add: 'Add',
      hostPattern: 'Host Pattern',
      hostPatternPlaceholder: 'e.g., github.com or server-*',
      hostPatternHint: 'Supports wildcards like * and ?',
      hostname: 'Hostname',
      hostnamePlaceholder: 'e.g., 192.168.1.100 or server.example.com',
      hostnameHint: 'Actual host address to connect to',
      user: 'Username',
      userPlaceholder: 'e.g., root or ubuntu',
      port: 'Port',
      portPlaceholder: 'Default 22',
      identityFile: 'Identity File (Private Key)',
      selectKey: 'Select Key',
      browse: 'Browse',
      identityGroups: {
        softwareKeys: 'Managed Keys',
        sshDirKeys: 'Keys in SSH directory'
      },
      addOption: 'Add Option',
      newHost: 'New Host',
      deleteConfirm: 'Are you sure you want to delete this host configuration?',
      noHostsMessage: 'No host configurations',
      otherOptions: 'Other Options',
      optionName: 'Option Name',
      optionValue: 'Option Value',
      deleteOption: 'Delete Option',
      selectHostMessage: 'Please select a host to configure',
      optionDescriptions: {
        HostName: 'Actual hostname or IP address to connect to',
        User: 'Username for remote login',
        Port: 'SSH port, default 22',
        IdentityFile: 'Private key path(s) for authentication',
        ProxyJump: 'Jump through a proxy host (equivalent to -J)',
        ProxyCommand: 'Custom proxy command used to establish the connection',
        ForwardAgent: 'Forward local SSH agent, yes/no',
        ForwardX11: 'Enable X11 forwarding, yes/no',
        StrictHostKeyChecking: 'Host key checking policy (yes/no/accept-new)',
        UserKnownHostsFile: 'Known hosts file path (default ~/.ssh/known_hosts)',
        PreferredAuthentications: 'Preferred authentication methods, comma-separated',
        ServerAliveInterval: 'Keepalive interval in seconds',
        ServerAliveCountMax: 'Max keepalive failures before disconnect',
        Compression: 'Enable compression, yes/no',
        IdentitiesOnly: 'Use only specified identities for authentication, yes/no',
        AddKeysToAgent: 'Automatically add keys to ssh-agent (yes/no/confirm)',
        ControlMaster: 'Reuse existing connections (yes/no/auto)',
        ControlPath: 'Control socket path for connection multiplexing',
        ControlPersist: 'Keep master connection alive (e.g., 10m or yes)',
        LocalForward: 'Local port forward: local_port remote_host:remote_port',
        RemoteForward: 'Remote port forward: remote_port local_host:local_port',
        SendEnv: 'Environment variables to send (can be specified multiple times)',
        SetEnv: 'Set environment variables in session (KEY=VALUE)',
        LogLevel: 'Logging level (QUIET/ERROR/INFO/VERBOSE/DEBUG)',
        TCPKeepAlive: 'Enable TCP keepalive mechanism to keep connection alive (yes/no)'
      }
    },
    preview: {
      title: 'Configuration Preview',
      showEditor: 'Show Editor',
      hideEditor: 'Hide Editor',
      rawPlaceholder: '# SSH Configuration File\n# Please edit the raw configuration directly',
      applyChanges: 'Apply Changes'
    },
    messages: {
      loadError: 'Failed to load configuration:',
      saveError: 'Failed to save configuration:',
      saveSuccess: 'Configuration saved successfully',
      copySuccess: 'Configuration copied to clipboard',
      copyError: 'Copy failed:',
      featureNotImplemented: 'Feature not implemented yet',
      fileExists: 'File already exists'
    },
    empty: 'Please select a host configuration or add a new host'
  },
  importExport: {
    dialog: {
      importTitle: 'Import Keys',
      exportTitle: 'Export Keys',
      cancel: 'Cancel',
      importing: 'Importing...',
      exporting: 'Exporting...',
      importAction: 'Import Keys',
      exportAction: 'Export Keys'
    },
    import: {
      method: {
        title: 'Import Method',
        file: 'Import from File',
        text: 'Import from Text'
      },
      file: {
        label: 'Select Key File',
        placeholder: 'Please select a key file'
      },
      text: {
        label: 'Paste Key Data',
        placeholder: 'Please paste exported key data (JSON format)'
      },
      preview: {
        title: 'Keys to Import',
        count: 'keys'
      }
    },
    export: {
      scope: {
        title: 'Export Scope',
        all: 'Export All Keys',
        selected: 'Export Selected Keys'
      },
      format: {
        title: 'Export Format',
        json: 'JSON Format (.json)',
        openssh: 'OpenSSH Format',
        pem: 'PEM Format',
        description: 'JSON Format: Suitable for app backup and restore | OpenSSH/PEM Format: Suitable for system use'
      },
      security: {
        includePrivate: 'Include Private Keys (Not recommended, use only in secure environments)'
      },
      preview: {
        title: 'Export Preview',
        willExport: 'Will export',
        keys: 'keys',
        keyName: 'Key:',
        keyType: 'Type:',
        publicKeyFile: 'Public Key File',
        privateKeyFile: 'Private Key File',
        moreKeys: 'more',
        fingerprint: 'Fingerprint:'
      }
    },
    messages: {
      noValidKeys: 'No valid key data found in file',
      parseError: 'File parsing failed:',
      unknownError: 'Unknown error',
      fileReadError: 'File reading failed',
      invalidFormat: 'Invalid JSON file format. Supported formats: standard export file, key array, or single key object',
      importSuccess: 'Successfully imported',
      exportSuccess: 'Keys successfully exported to:',
      importError: 'Import failed:',
      exportError: 'Export failed:',
      noImportData: 'No data to import',
      parseWarning: 'Parsing successful, but',
      invalidKeysIgnored: 'invalid keys were ignored'
    }
  },
  settings: {
    title: 'Settings',
    appSettings: 'Application Settings',
    developing: 'Feature under development',
    language: {
      title: 'Language Settings',
      select: 'Select Language'
    },
    reset: {
      title: 'Reset Application',
      description: 'Clear all data, including the master password, all keys, and current language settings. This action cannot be undone.',
      button: 'Reset Application',
      success: 'Data has been successfully reset',
      confirm: 'Are you sure you want to reset the application? All data will be cleared, including the master password, all keys, and current language settings. This action cannot be undone.'
    }
  },
  auth: {
    setup: {
      title: 'Initialize Application',
      subtitle: 'Set a master password to protect your SSH keys',
      button: 'Initialize Application',
      initializing: 'Initializing...'
    },
    login: {
      title: 'Unlock Application',
      subtitle: 'Enter your master password to access your keys',
      button: 'Unlock',
      loggingIn: 'Logging in...'
    },
    masterPassword: 'Master Password',
    confirmPassword: 'Confirm Password',
    setupPlaceholder: 'Please set a strong password',
    loginPlaceholder: 'Please enter your master password',
    confirmPlaceholder: 'Please enter the password again',
    securityTips: {
      title: 'Important Notice',
      items: [
        'Please keep your master password safe. It cannot be recovered if lost',
        'Use a strong password with uppercase, lowercase, numbers, and special characters',
        'Password should be at least 12 characters long'
      ]
    },
    errors: {
      passwordLength: 'Password must be at least 8 characters long',
      passwordMismatch: 'Passwords do not match',
      initializationFailed: 'Initialization failed',
      wrongPassword: 'Wrong password',
      operationFailed: 'Operation failed, please try again later'
    }
  }
}

const userLanguage = getUserLanguage()

const i18n = createI18n({
  legacy: false, // 启用 Composition API 模式
  locale: userLanguage,
  fallbackLocale: 'en', // 如果没有对应的语言，使用英文
  messages: {
    zh,
    en
  },
  globalInjection: true // 启用全局注入，支持 $t
})

export default i18n
export { zh, en }