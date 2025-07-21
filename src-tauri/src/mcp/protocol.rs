// MCP通信プロトコル定義

use serde::{Serialize, Deserialize};
// 必要なインポートは実装時に追加

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub action: String,
    pub workspace: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklogWorkspace {
    pub name: String,
    pub domain: String,
    pub api_key: String,
    pub enabled: bool,
}