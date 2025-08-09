// ProjectLens モジュール定義
pub mod ai;
pub mod auth;
pub mod crypto;
pub mod storage;
pub mod mcp;
pub mod docker;
pub mod models;

use docker::service::DockerService;
use docker::container::ContainerStatus;
use auth::master_password::{MasterPasswordManager, MasterPasswordError, SessionStatus, PasswordStrength};
use std::sync::{Arc, Mutex};

// グローバルなマスターパスワード管理インスタンス（実際の実装では依存注入を使用すべき）
lazy_static::lazy_static! {
    static ref MASTER_PASSWORD_MANAGER: Arc<Mutex<MasterPasswordManager>> = 
        Arc::new(Mutex::new(MasterPasswordManager::new()));
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Docker関連のTauriコマンド
#[tauri::command]
async fn check_docker_available() -> Result<bool, String> {
    let docker_service = DockerService::default();
    docker_service.is_docker_available().await
}

#[tauri::command]
async fn is_docker_running() -> Result<bool, String> {
    let docker_service = DockerService::default();
    docker_service.is_docker_running().await
}

#[tauri::command]
async fn get_docker_version() -> Result<String, String> {
    let docker_service = DockerService::default();
    docker_service.get_docker_version().await
}

#[tauri::command]
async fn check_mcp_server_status() -> Result<ContainerStatus, String> {
    let docker_service = DockerService::default();
    docker_service.check_mcp_server_container().await
}

#[tauri::command]
async fn start_mcp_server() -> Result<(), String> {
    let docker_service = DockerService::default();
    docker_service.start_mcp_server_container().await
}

#[tauri::command]
async fn stop_mcp_server() -> Result<(), String> {
    let docker_service = DockerService::default();
    docker_service.stop_mcp_server_container().await
}

#[tauri::command]
async fn check_mcp_server_exists() -> Result<bool, String> {
    let docker_service = DockerService::default();
    docker_service.check_mcp_server_container_exists().await
}

// 認証関連のTauriコマンド

/// マスターパスワードを設定
#[tauri::command]
async fn set_master_password(password: String) -> Result<PasswordStrength, String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    manager.set_password(&password).map_err(|e| e.to_string())
}

/// マスターパスワードを検証してセッションを開始
#[tauri::command]
async fn verify_master_password(password: String) -> Result<u64, String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    manager.verify_password(&password).map_err(|e| e.to_string())
}

/// 現在のセッション状態を確認
#[tauri::command]
async fn get_session_status() -> Result<SessionStatus, String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    manager.get_session_status().map_err(|e| e.to_string())
}

/// セッションを延長
#[tauri::command]
async fn extend_session() -> Result<u64, String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    manager.extend_session().map_err(|e| e.to_string())
}

/// セッションをクリア（ログアウト）
#[tauri::command]
async fn clear_session() -> Result<(), String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    manager.clear_session().map_err(|e| e.to_string())
}

/// マスターパスワードが設定済みかどうかを確認
#[tauri::command]
async fn is_master_password_set() -> Result<bool, String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    manager.is_password_set().map_err(|e| e.to_string())
}

/// 現在認証済みかどうかを確認
#[tauri::command]
async fn is_authenticated() -> Result<bool, String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    manager.is_authenticated().map_err(|e| e.to_string())
}

/// パスワード強度をチェック
#[tauri::command]
async fn check_password_strength(password: String) -> Result<PasswordStrength, String> {
    let manager = MASTER_PASSWORD_MANAGER.lock().map_err(|e| {
        format!("マスターパスワード管理の取得に失敗しました: {}", e)
    })?;
    
    Ok(manager.check_password_strength(&password))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            check_docker_available,
            is_docker_running,
            get_docker_version,
            check_mcp_server_status,
            start_mcp_server,
            stop_mcp_server,
            check_mcp_server_exists,
            set_master_password,
            verify_master_password,
            get_session_status,
            extend_session,
            clear_session,
            is_master_password_set,
            is_authenticated,
            check_password_strength
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
