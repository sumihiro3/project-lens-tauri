// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// モジュールのインポート
mod ai;
mod crypto;
mod docker;
mod mcp;
mod models;
mod storage;

use docker::service::DockerService;
use docker::container::ContainerStatus;

// Dockerサービス関連のTauriコマンド

/// Dockerが利用可能かどうかを確認するコマンド
#[tauri::command]
async fn check_docker_available() -> Result<bool, String> {
    let docker_service = DockerService::default();
    docker_service.is_docker_available().await
}

/// Dockerのバージョン情報を取得するコマンド
#[tauri::command]
async fn get_docker_version() -> Result<String, String> {
    let docker_service = DockerService::default();
    docker_service.get_docker_version().await
}

/// Docker Engineが実行中かどうかを確認するコマンド
#[tauri::command]
async fn is_docker_running() -> Result<bool, String> {
    let docker_service = DockerService::default();
    docker_service.is_docker_running().await
}

/// MCP Serverコンテナの状態を確認するコマンド
#[tauri::command]
async fn check_mcp_server_status() -> Result<ContainerStatus, String> {
    let docker_service = DockerService::default();
    docker_service.check_mcp_server_container().await
}

/// MCP Serverコンテナを起動するコマンド
#[tauri::command]
async fn start_mcp_server() -> Result<(), String> {
    let docker_service = DockerService::default();
    docker_service.start_mcp_server_container().await
}

/// MCP Serverコンテナを停止するコマンド
#[tauri::command]
async fn stop_mcp_server() -> Result<(), String> {
    let docker_service = DockerService::default();
    docker_service.stop_mcp_server_container().await
}

/// MCP Serverコンテナが存在するかどうかを確認するコマンド
#[tauri::command]
async fn check_mcp_server_exists() -> Result<bool, String> {
    let docker_service = DockerService::default();
    docker_service.check_mcp_server_container_exists().await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_docker_available,
            get_docker_version,
            is_docker_running,
            check_mcp_server_status,
            start_mcp_server,
            stop_mcp_server,
            check_mcp_server_exists,
        ])
        .run(tauri::generate_context!())
        .expect("Tauriアプリケーションの実行中にエラーが発生しました");
}
