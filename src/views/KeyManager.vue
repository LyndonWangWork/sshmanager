<template>
  <div class="min-h-screen bg-transparent">
    <!-- 导航栏 -->
    <nav class="bg-white shadow-sm border-b border-gray-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <router-link to="/" class="text-blue-600 hover:text-blue-800 mr-4">
              <ArrowLeftIcon class="h-5 w-5" />
            </router-link>
            <h1 class="text-xl font-semibold text-gray-900">密钥管理</h1>
          </div>
          <div class="flex items-center space-x-4">
            <BaseButton @click="$router.push({ name: 'KeyGenerator' })">
              <PlusIcon class="h-4 w-4 mr-2" />
              生成新密钥
            </BaseButton>
            <BaseButton variant="secondary" @click="showImportDialog = true">
              <ArrowUpTrayIcon class="h-4 w-4 mr-2" />
              导入密钥
            </BaseButton>
            <BaseButton variant="secondary" @click="showExportDialog = true" :disabled="keyStore.keys.length === 0">
              <ArrowDownTrayIcon class="h-4 w-4 mr-2" />
              导出密钥
            </BaseButton>
          </div>
        </div>
      </div>
    </nav>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <!-- 统计信息 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        <div class="bg-white p-6 rounded-lg shadow-sm">
          <div class="flex items-center">
            <div class="p-2 bg-blue-100 rounded-lg">
              <KeyIcon class="h-6 w-6 text-blue-600" />
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-gray-500">总密钥数</p>
              <p class="text-2xl font-semibold text-gray-900">{{ keyStore.keys.length }}</p>
            </div>
          </div>
        </div>
        
        <div class="bg-white p-6 rounded-lg shadow-sm">
          <div class="flex items-center">
            <div class="p-2 bg-green-100 rounded-lg">
              <ShieldCheckIcon class="h-6 w-6 text-green-600" />
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-gray-500">RSA 密钥</p>
              <p class="text-2xl font-semibold text-gray-900">{{ rsaKeyCount }}</p>
            </div>
          </div>
        </div>
        
        <div class="bg-white p-6 rounded-lg shadow-sm">
          <div class="flex items-center">
            <div class="p-2 bg-purple-100 rounded-lg">
              <CpuChipIcon class="h-6 w-6 text-purple-600" />
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-gray-500">Ed25519 密钥</p>
              <p class="text-2xl font-semibold text-gray-900">{{ ed25519KeyCount }}</p>
            </div>
          </div>
        </div>
        
        <div class="bg-white p-6 rounded-lg shadow-sm">
          <div class="flex items-center">
            <div class="p-2 bg-orange-100 rounded-lg">
              <CircleStackIcon class="h-6 w-6 text-orange-600" />
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-gray-500">ECDSA 密钥</p>
              <p class="text-2xl font-semibold text-gray-900">{{ ecdsaKeyCount }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 搜索和过滤 -->
      <div class="bg-white rounded-lg shadow-sm p-6 mb-8">
        <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
          <!-- 搜索框 -->
          <div class="relative flex-1 max-w-md">
            <MagnifyingGlassIcon class="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索密钥名称或注释..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          
          <!-- 过滤器 -->
          <div class="flex items-center space-x-4">
            <select
              v-model="selectedKeyType"
              class="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">所有类型</option>
              <option value="rsa">RSA</option>
              <option value="ed25519">Ed25519</option>
              <option value="ecdsa">ECDSA</option>
            </select>
            
            <select
              v-model="sortBy"
              class="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="created_at">按创建时间</option>
              <option value="name">按名称</option>
              <option value="last_used">按使用时间</option>
            </select>
          </div>
        </div>
      </div>

      <!-- 密钥列表 -->
      <div v-if="keyStore.isLoading" class="flex justify-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
      
      <div v-else-if="filteredKeys.length === 0" class="text-center py-12">
        <KeyIcon class="mx-auto h-12 w-12 text-gray-400" />
        <h3 class="mt-2 text-sm font-medium text-gray-900">没有找到密钥</h3>
        <p class="mt-1 text-sm text-gray-500">
          {{ keyStore.keys.length === 0 ? '还没有创建任何SSH密钥' : '没有匹配搜索条件的密钥' }}
        </p>
        <div class="mt-6">
          <BaseButton @click="$router.push({ name: 'KeyGenerator' })">
            <PlusIcon class="h-4 w-4 mr-2" />
            生成第一个密钥
          </BaseButton>
        </div>
      </div>
      
      <div v-else class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        <KeyCard
          v-for="key in filteredKeys"
          :key="key.id"
          :key-data="key"
          @delete="handleDeleteKey"
          @edit="handleEditKey"
          @export="handleExportKey"
        />
      </div>
    </div>

    <!-- 编辑密钥对话框 -->
    <div v-if="showEditDialog" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-full max-w-md mx-4">
        <h3 class="text-lg font-semibold mb-4">编辑密钥信息</h3>
        
        <div class="space-y-4">
          <BaseInput
            v-model="editForm.name"
            label="密钥名称"
            required
          />
          
          <BaseInput
            v-model="editForm.comment"
            label="注释"
          />
        </div>
        
        <div class="flex justify-end space-x-3 mt-6">
          <BaseButton variant="secondary" @click="closeEditDialog">
            取消
          </BaseButton>
          <BaseButton @click="saveKeyEdit">
            保存
          </BaseButton>
        </div>
      </div>
    </div>

    <!-- 导入导出对话框 -->
    <ImportExportDialog
      :show="showImportDialog"
      mode="import"
      @close="showImportDialog = false"
      @success="handleImportSuccess"
      @error="handleImportError"
    />
    
    <ImportExportDialog
      :show="showExportDialog"
      mode="export"
      :selected-keys="selectedKeys"
      @close="showExportDialog = false"
      @success="handleExportSuccess"
      @error="handleExportError"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useKeyStore } from '@/stores/key'
import type { SshKeyPair } from '@/types'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import KeyCard from '@/components/KeyCard.vue'
import ImportExportDialog from '@/components/ImportExportDialog.vue'
import {
  KeyIcon,
  PlusIcon,
  ArrowUpTrayIcon,
  ArrowDownTrayIcon,
  ArrowLeftIcon,
  MagnifyingGlassIcon,
  ShieldCheckIcon,
  CpuChipIcon, 
  CircleStackIcon
} from '@heroicons/vue/24/outline'

const keyStore = useKeyStore()

// 搜索和过滤状态
const searchQuery = ref('')
const selectedKeyType = ref('')
const sortBy = ref('created_at')

// 编辑对话框状态
const showEditDialog = ref(false)
const editingKey = ref<SshKeyPair | null>(null)
const editForm = ref({
  name: '',
  comment: ''
})

// 导入导出对话框状态
const showImportDialog = ref(false)
const showExportDialog = ref(false)
const selectedKeys = ref<SshKeyPair[]>([])

// 统计计算
const rsaKeyCount = computed(() => 
  keyStore.keys.filter(key => key.key_type === 'Rsa').length
)

const ed25519KeyCount = computed(() => 
  keyStore.keys.filter(key => key.key_type === 'Ed25519').length
)

const ecdsaKeyCount = computed(() => 
  keyStore.keys.filter(key => key.key_type === 'Ecdsa').length
)

// 过滤和排序密钥
const filteredKeys = computed(() => {
  let filtered = keyStore.keys
  
  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(key => 
      key.name.toLowerCase().includes(query) ||
      key.comment.toLowerCase().includes(query)
    )
  }
  
  // 类型过滤
  if (selectedKeyType.value) {
    filtered = filtered.filter(key => key.key_type === selectedKeyType.value)
  }
  
  // 排序
  return filtered.sort((a, b) => {
    switch (sortBy.value) {
      case 'name':
        return a.name.localeCompare(b.name)
      case 'last_used':
        const aTime = a.last_used ? new Date(a.last_used).getTime() : 0
        const bTime = b.last_used ? new Date(b.last_used).getTime() : 0
        return bTime - aTime
      case 'created_at':
      default:
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    }
  })
})

// 处理密钥删除
const handleDeleteKey = async (keyId: string) => {
  try {
    await keyStore.deleteKey(keyId)
    // TODO: 显示成功提示
  } catch (error) {
    console.error('删除密钥失败:', error)
    // TODO: 显示错误提示
  }
}

// 处理密钥编辑
const handleEditKey = (keyData: SshKeyPair) => {
  editingKey.value = keyData
  editForm.value = {
    name: keyData.name,
    comment: keyData.comment
  }
  showEditDialog.value = true
}

// 保存密钥编辑
const saveKeyEdit = async () => {
  // TODO: 实现密钥信息更新功能
  console.log('更新密钥信息:', editForm.value)
  closeEditDialog()
}

// 关闭编辑对话框
const closeEditDialog = () => {
  showEditDialog.value = false
  editingKey.value = null
  editForm.value = { name: '', comment: '' }
}

// 处理密钥导出
const handleExportKey = async (keyId: string) => {
  try {
    // 找到要导出的密钥
    const keyToExport = keyStore.keys.find(key => key.id === keyId)
    if (!keyToExport) {
      console.error('未找到要导出的密钥')
      return
    }
    
    // 设置选中的密钥并打开导出对话框
    selectedKeys.value = [keyToExport]
    showExportDialog.value = true
  } catch (error) {
    console.error('导出密钥失败:', error)
    // TODO: 显示错误提示
  }
}

// 导入密钥
const importKeys = () => {
  // TODO: 实现密钥导入功能
  console.log('导入密钥功能待实现')
}

// 处理导入成功
const handleImportSuccess = (message: string) => {
  console.log('导入成功:', message)
  // TODO: 显示成功提示
}

// 处理导入错误
const handleImportError = (error: string) => {
  console.error('导入错误:', error)
  // TODO: 显示错误提示
}

// 处理导出成功
const handleExportSuccess = (message: string) => {
  console.log('导出成功:', message)
  // TODO: 显示成功提示
}

// 处理导出错误
const handleExportError = (error: string) => {
  console.error('导出错误:', error)
  // TODO: 显示错误提示
}

// 页面初始化
onMounted(async () => {
  try {
    await keyStore.loadKeys()
  } catch (error) {
    console.error('加载密钥失败:', error)
  }
})
</script>