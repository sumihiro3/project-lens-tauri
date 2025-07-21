// Docker環境チェックサービス実装

use super::container::{ContainerStatus, ContainerConfig, ContainerManager};
use std::process::Command;
use std::time::Duration;
use tokio::time;

/// Docker環境チェックとMCP Serverコンテナ管理を担当するサービス
pub struct DockerService {
    /// MCP Serverコンテナ名
    mcp_container_name: String,
}

impl DockerService {
    /// 新しいDockerServiceインスタンスを作成
    pub fn new(mcp_container_name: &str) -> Self {
        Self {
            mcp_container_name: mcp_container_name.to_string(),
        }
    }
    
    /// デフォルト設定でDockerServiceインスタンスを作成
    pub fn default() -> Self {
        Self {
            mcp_container_name: "backlog-mcp-server".to_string(),
        }
    }
    
    /// Dockerが利用可能かどうかを確認
    /// 
    /// # 戻り値
    /// - `Ok(true)` - Dockerが利用可能
    /// - `Ok(false)` - Dockerが利用不可能
    /// - `Err(String)` - エラーメッセージ
    pub async fn is_docker_available(&self) -> Result<bool, String> {
        // タイムアウト付きでDockerコマンド実行
        let result = time::timeout(Duration::from_secs(10), async {
            Command::new("docker")
                .arg("--version")
                .output()
                .map_err(|e| format!("Dockerコマンド実行エラー: {}", e))
        }).await;
        
        match result {
            Ok(Ok(output)) => Ok(output.status.success()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Dockerコマンドがタイムアウトしました".to_string()),
        }
    }
    
    /// Dockerのバージョン情報を取得
    /// 
    /// # 戻り値
    /// - `Ok(String)` - Dockerのバージョン情報
    /// - `Err(String)` - エラーメッセージ
    pub async fn get_docker_version(&self) -> Result<String, String> {
        // タイムアウト付きでDockerバージョン取得
        let result = time::timeout(Duration::from_secs(10), async {
            Command::new("docker")
                .arg("--version")
                .output()
                .map_err(|e| format!("Dockerコマンド実行エラー: {}", e))
        }).await;
        
        match result {
            Ok(Ok(output)) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    Ok(version)
                } else {
                    Err(format!("Dockerコマンド失敗: {}", String::from_utf8_lossy(&output.stderr)))
                }
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Dockerバージョン取得がタイムアウトしました".to_string()),
        }
    }
    
    /// Docker Engineが実行中かどうかを確認
    /// 
    /// # 戻り値
    /// - `Ok(true)` - Docker Engineが実行中
    /// - `Ok(false)` - Docker Engineが停止中
    /// - `Err(String)` - エラーメッセージ
    pub async fn is_docker_running(&self) -> Result<bool, String> {
        // タイムアウト付きでDocker実行状態確認
        let result = time::timeout(Duration::from_secs(10), async {
            Command::new("docker")
                .arg("info")
                .output()
                .map_err(|e| format!("Dockerコマンド実行エラー: {}", e))
        }).await;
        
        match result {
            Ok(Ok(output)) => Ok(output.status.success()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Docker実行状態確認がタイムアウトしました".to_string()),
        }
    }
    
    /// MCP Serverコンテナの状態を確認
    /// 
    /// # 戻り値
    /// - `Ok(ContainerStatus)` - コンテナの状態情報
    /// - `Err(String)` - エラーメッセージ
    pub async fn check_mcp_server_container(&self) -> Result<ContainerStatus, String> {
        // ContainerManagerを使用してコンテナ状態を確認
        let container_manager = ContainerManager::new(&self.mcp_container_name)
            .await
            .map_err(|e| format!("Docker接続エラー: {}", e))?;
        
        let is_running = container_manager.check_container_status()
            .await
            .map_err(|e| format!("コンテナ状態確認エラー: {}", e))?;
        
        Ok(ContainerStatus {
            name: self.mcp_container_name.clone(),
            state: if is_running { "running".to_string() } else { "stopped".to_string() },
            is_running,
        })
    }
    
    /// MCP Serverコンテナを起動
    /// 
    /// # 戻り値
    /// - `Ok(())` - コンテナ起動成功
    /// - `Err(String)` - エラーメッセージ
    pub async fn start_mcp_server_container(&self) -> Result<(), String> {
        // コンテナの状態を確認
        let status = self.check_mcp_server_container().await?;
        
        // 既に実行中の場合は何もしない
        if status.is_running {
            return Ok(());
        }
        
        // コンテナを起動
        let container_manager = ContainerManager::new(&self.mcp_container_name)
            .await
            .map_err(|e| format!("Docker接続エラー: {}", e))?;
        
        container_manager.start_container()
            .await
            .map_err(|e| format!("コンテナ起動エラー: {}", e))?;
        
        // コンテナが起動するまで待機（最大30秒）
        let mut attempts = 0;
        const MAX_ATTEMPTS: u8 = 15;
        
        while attempts < MAX_ATTEMPTS {
            time::sleep(Duration::from_secs(2)).await;
            
            let status = self.check_mcp_server_container().await?;
            if status.is_running {
                return Ok(());
            }
            
            attempts += 1;
        }
        
        Err("MCP Serverコンテナの起動がタイムアウトしました".to_string())
    }
    
    /// MCP Serverコンテナを停止
    /// 
    /// # 戻り値
    /// - `Ok(())` - コンテナ停止成功
    /// - `Err(String)` - エラーメッセージ
    pub async fn stop_mcp_server_container(&self) -> Result<(), String> {
        // コンテナの状態を確認
        let status = self.check_mcp_server_container().await?;
        
        // 既に停止している場合は何もしない
        if !status.is_running {
            return Ok(());
        }
        
        // コンテナを停止
        let container_manager = ContainerManager::new(&self.mcp_container_name)
            .await
            .map_err(|e| format!("Docker接続エラー: {}", e))?;
        
        container_manager.stop_container()
            .await
            .map_err(|e| format!("コンテナ停止エラー: {}", e))?;
        
        Ok(())
    }
    
    /// MCP Serverコンテナが存在するかどうかを確認
    /// 
    /// # 戻り値
    /// - `Ok(true)` - コンテナが存在する
    /// - `Ok(false)` - コンテナが存在しない
    /// - `Err(String)` - エラーメッセージ
    pub async fn check_mcp_server_container_exists(&self) -> Result<bool, String> {
        let output = Command::new("docker")
            .args(["ps", "-a", "--filter", &format!("name={}", self.mcp_container_name), "--format", "{{.Names}}"])
            .output()
            .map_err(|e| format!("Dockerコマンド実行エラー: {}", e))?;
            
        if !output.status.success() {
            return Err(format!("Dockerコマンド失敗: {}", String::from_utf8_lossy(&output.stderr)));
        }
        
        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(!output_str.is_empty())
    }
}