// モデルモジュール
// データモデル定義

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub project_id: String,
    pub workspace_id: String,  // 技術仕様書準拠: ワークスペース識別子
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: Priority,
    pub assignee_id: Option<String>,  // User型からStringに変更
    pub reporter_id: String,          // User型からStringに変更
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub raw_data: String,  // 技術仕様書準拠: JSON形式でオリジナルデータを保存
    // 以下は別途管理（正規化）
    // pub comments: Vec<Comment>,
    // pub mentions: Vec<User>,
    // pub watchers: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low = 1,      // 技術仕様書準拠: INTEGER値との対応
    Normal = 2,
    High = 3,
    Critical = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub workspace_id: String,  // 技術仕様書準拠: workspace_nameからworkspace_idに変更
    pub weight_score: u8,      // 1-10の範囲チェック
    pub updated_at: DateTime<Utc>,
}

impl ProjectWeight {
    /// 重みスコアの検証（1-10の範囲チェック）
    pub fn validate_weight_score(score: u8) -> Result<u8, String> {
        if score >= 1 && score <= 10 {
            Ok(score)
        } else {
            Err(format!("重みスコアは1-10の範囲で指定してください: {}", score))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub key: String,
    pub description: Option<String>,
    pub workspace_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BacklogWorkspaceConfig {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub api_key_encrypted: String,
    pub encryption_version: String,  // 技術仕様書準拠: 暗号化バージョン管理
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BacklogWorkspaceConfig {
    /// 新しいワークスペース設定を作成
    pub fn new(
        id: String,
        name: String,
        domain: String,
        api_key_encrypted: String,
        encryption_version: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            domain,
            api_key_encrypted,
            encryption_version,
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }
}

/// AI分析結果データモデル（技術仕様書準拠）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysis {
    pub ticket_id: String,
    pub urgency_score: f32,
    pub complexity_score: f32,
    pub user_relevance_score: f32,
    pub project_weight_factor: f32,
    pub final_priority_score: f32,
    pub recommendation_reason: String,
    pub category: String,
    pub analyzed_at: DateTime<Utc>,
}

impl AIAnalysis {
    /// 新しいAI分析結果を作成
    pub fn new(
        ticket_id: String,
        urgency_score: f32,
        complexity_score: f32,
        user_relevance_score: f32,
        project_weight_factor: f32,
        recommendation_reason: String,
        category: String,
    ) -> Self {
        let final_priority_score = Self::calculate_final_score(
            urgency_score,
            complexity_score,
            user_relevance_score,
            project_weight_factor,
        );

        Self {
            ticket_id,
            urgency_score,
            complexity_score,
            user_relevance_score,
            project_weight_factor,
            final_priority_score,
            recommendation_reason,
            category,
            analyzed_at: Utc::now(),
        }
    }

    /// 最終優先度スコアの計算（技術仕様書のアルゴリズム準拠）
    fn calculate_final_score(
        urgency: f32,
        complexity: f32,
        user_relevance: f32,
        project_weight: f32,
    ) -> f32 {
        // 基本スコア（緊急度40%、複雑度30%、ユーザー関連度30%）
        let base_score = (urgency * 0.4) + (complexity * 0.3) + (user_relevance * 0.3);
        
        // プロジェクト重みを適用（1-10スケールを0.2-2.0に正規化）
        let weight_multiplier = project_weight / 5.0;
        
        // 0-100の範囲にクランプ
        (base_score * weight_multiplier).max(0.0).min(100.0)
    }
}

/// 緊急度判定要因データモデル（技術仕様書準拠）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgencyFactors {
    pub due_date: Option<DateTime<Utc>>,
    pub recent_comments: i32,
    pub mentions_count: i32,
    pub last_update_days: i32,
    pub is_assigned_to_user: bool,
    pub is_blocking_other_tickets: bool,
}

impl UrgencyFactors {
    /// 緊急度乗数の計算（技術仕様書アルゴリズム準拠）
    pub fn calculate_urgency_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        
        // 期限による緊急度
        if let Some(due_date) = self.due_date {
            let days_until_due = (due_date - Utc::now()).num_days();
            multiplier *= match days_until_due {
                ..=0 => 2.0,      // 期限切れ
                1 => 1.8,         // 1日以内
                2..=3 => 1.5,     // 2-3日以内
                4..=7 => 1.2,     // 1週間以内
                _ => 1.0,         // それ以上
            };
        }
        
        // コメント活動による緊急度
        if self.recent_comments > 3 {
            multiplier *= 1.3;
        }
        
        // メンション数による緊急度
        if self.mentions_count > 1 {
            multiplier *= 1.2;
        }
        
        // 担当者チケットは優先度アップ
        if self.is_assigned_to_user {
            multiplier *= 1.1;
        }
        
        // ブロッカーチケットは最優先
        if self.is_blocking_other_tickets {
            multiplier *= 1.5;
        }
        
        multiplier
    }
}

#[cfg(test)]
mod ai_analysis_test;