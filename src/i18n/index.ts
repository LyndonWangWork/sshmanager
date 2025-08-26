import { createI18n } from 'vue-i18n'
import { getUserLanguage } from '@/utils/language'

// 中文翻译
const zh = {
  common: {
    confirm: '确认',
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
    info: '信息'
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
      commentPlaceholder: '例如：user@hostname'
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
    empty: {
      noKeys: '没有找到密钥',
      noKeysCreated: '还没有创建任何SSH密钥',
      noMatching: '没有匹配搜索条件的密钥',
      generateFirst: '生成第一个密钥'
    }
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
      addOption: '添加选项'
    },
    empty: '请选择一个主机配置或添加新的主机'
  },
  settings: {
    title: '设置',
    appSettings: '应用设置',
    language: {
      title: '语言设置',
      current: '当前语言',
      select: '选择语言',
      chinese: '中文',
      english: 'English'
    },
    developing: '设置功能正在开发中...'
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
    info: 'Info'
  },
  nav: {
    dashboard: 'Dashboard',
    keyManager: 'Key Manager',
    keyGenerator: 'Key Generator',
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
      commentPlaceholder: 'e.g., user@hostname'
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
    empty: {
      noKeys: 'No keys found',
      noKeysCreated: 'No SSH keys have been created yet',
      noMatching: 'No keys match the search criteria',
      generateFirst: 'Generate First Key'
    }
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
      addOption: 'Add Option'
    },
    empty: 'Please select a host configuration or add a new host'
  },
  settings: {
    title: 'Settings',
    appSettings: 'Application Settings',
    language: {
      title: 'Language Settings',
      current: 'Current Language',
      select: 'Select Language',
      chinese: '中文',
      english: 'English'
    },
    developing: 'Settings functionality is under development...'
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