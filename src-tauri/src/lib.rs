pub mod commands;
pub mod error;
pub mod services;
pub mod storage;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

use commands::*;
use services::CryptoService;
use std::sync::Mutex;
use storage::StorageService;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .manage(Mutex::new(CryptoService::new()))
        .manage(Mutex::new(
            StorageService::new().expect("存储服务初始化失败"),
        ))
        .invoke_handler(tauri::generate_handler![
            is_initialized,
            initialize_app,
            authenticate,
            generate_ssh_key,
            get_all_keys,
            delete_key,
            export_key,
            export_keys_to_file,
            write_file_content,
            ensure_dir_exists,
            save_ssh_config,
            read_ssh_config,
            list_identity_files,
            check_file_exists,
            import_keys,
            export_all_keys,
            reset_all_data,
            open_config_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
