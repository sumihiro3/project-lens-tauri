// MCP（Model Context Protocol）サービス
// Backlog MCP Serverとの通信を管理

use crate::mcp::client::MCPClient;
use crate::mcp::protocol::*;
use crate::models::*;
use std::sync::Arc;

/// MCP サービス
/// Backlog MCP Serverとの通信を抽象化
pub struct MCPService {
    client: Arc<MCPClient>,
}

impl MCPService {
    /// 新しいMCPサービスを作成
    pub fn new(client: Arc<MCPClient>) -> Self {
        Self { client }
    }

    /// ワークスペース一覧を取得
    pub async fn get_workspaces(&self) -> Result<Vec<BacklogWorkspace>, String> {
        self.client.get_workspaces().await
    }

    /// ユーザーのチケット一覧を取得
    pub async fn get_user_tickets(&self, workspace: &BacklogWorkspace, user_id: &str) -> Result<Vec<Ticket>, String> {
        self.client.get_user_tickets(workspace, user_id).await
    }

    /// プロジェクト一覧を取得
    pub async fn get_projects(&self, workspace: &BacklogWorkspace) -> Result<Vec<Project>, String> {
        self.client.get_projects(workspace).await
    }

    /// Docker コンテナの状態を確認
    pub async fn check_container_status(&self) -> Result<bool, String> {
        // 実装は今後追加予定
        Ok(false)
    }
}