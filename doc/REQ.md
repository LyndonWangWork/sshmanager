在使用github时，可以给账号添加ssh key以便使用git协议进行clone。例如给账号A添加了key 1，那么key 1就不可以再给账号B添加了。
所以，需要再生成一个ssh key，此时就面临一个ssh key生成和管理的问题。
我希望有一个工具，可以生成ssh key，管理ssh key，并可以可视化的编辑~/.ssh/config
该工具所有数据本地加密保存，可以导入导出
技术栈：tauri, vue3, tailwindcss