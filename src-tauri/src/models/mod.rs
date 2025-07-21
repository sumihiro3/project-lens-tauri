// モデルモジュール
// データモデル定義

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: Priority,
    pub assignee: Option<User>,
    pub reporter: User,
    pub comments: Vec<Comment>,
    pub mentions: Vec<User>,
    pub watchers: Vec<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub author: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectWeight {
    pub project_id: String,
    pub project_name: String,
    pub workspace_name: String,
    pub weight_score: u8, // 1-10
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BacklogWorkspaceConfig {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub api_key_encrypted: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}