//! MCP（Model Context Protocol）サービス
//! Backlog MCP Serverとの通信を管理するサービス層

use crate::mcp::client::MCPClient;
use crate::mcp::protocol::*;
use crate::models::*;
use std::sync::Arc;

/// MCP サービス
/// 
/// Backlog MCP Serverとの通信を抽象化し、
/// アプリケーション層に対してBacklogデータへの統一的なアクセス方法を提供する
pub struct MCPService {
    /// MCPクライアントのArc参照
    client: Arc<MCPClient>,
}

impl MCPService {
    /// 新しいMCPサービスインスタンスを作成
    /// 
    /// # 引数
    /// * `client` - MCPクライアントのArc参照
    /// 
    /// # 戻り値
    /// 初期化されたMCPServiceインスタンス
    pub fn new(client: Arc<MCPClient>) -> Self {
        Self { client }
    }

    /// 利用可能なBacklogワークスペースの一覧を取得
    /// 
    /// # 戻り値
    /// * `Ok(Vec<BacklogWorkspace>)` - ワークスペース一覧
    /// * `Err(String)` - エラーメッセージ
    pub async fn get_workspaces(&self) -> Result<Vec<BacklogWorkspace>, String> {
        self.client.get_workspaces().await
    }

    /// 指定されたユーザーが関係するチケット一覧を取得
    /// 
    /// # 引数
    /// * `workspace` - 対象のBacklogワークスペース
    /// * `user_id` - 対象ユーザーのID
    /// 
    /// # 戻り値
    /// * `Ok(Vec<Ticket>)` - チケット一覧
    /// * `Err(String)` - エラーメッセージ
    pub async fn get_user_tickets(&self, workspace: &BacklogWorkspace, user_id: &str) -> Result<Vec<Ticket>, String> {
        self.client.get_user_tickets(workspace, user_id).await
    }

    /// 指定されたワークスペース内のプロジェクト一覧を取得
    /// 
    /// # 引数
    /// * `workspace` - 対象のBacklogワークスペース
    /// 
    /// # 戻り値
    /// * `Ok(Vec<Project>)` - プロジェクト一覧
    /// * `Err(String)` - エラーメッセージ
    pub async fn get_projects(&self, workspace: &BacklogWorkspace) -> Result<Vec<Project>, String> {
        self.client.get_projects(workspace).await
    }

    /// MCP ServerのDockerコンテナ実行状態を確認
    /// 
    /// # 戻り値
    /// * `Ok(true)` - コンテナが正常に実行されている
    /// * `Ok(false)` - コンテナが停止している
    /// * `Err(String)` - エラーメッセージ
    pub async fn check_container_status(&self) -> Result<bool, String> {
        // 実装は今後追加予定
        Ok(false)
    }
}