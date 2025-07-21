// ProjectLens モジュール定義
pub mod ai;
pub mod crypto;
pub mod storage;
pub mod mcp;
pub mod docker;
pub mod models;

use docker::service::DockerService;
use docker::container::ContainerStatus;

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
            check_mcp_server_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
