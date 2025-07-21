// AI分析結果モデル

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::models::Ticket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub analyzed_at: DateTime<Utc>,
    pub ticket_count: usize,
    pub categories: Vec<TaskCategory>,
    pub urgency_scores: Vec<UrgencyScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCategory {
    pub name: String,
    pub ticket_ids: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgencyScore {
    pub ticket_id: String,
    pub score: f32, // 0.0 - 1.0
    pub factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub ticket: Ticket,
    pub priority_score: f32, // 0.0 - 1.0
    pub reasoning: String,
    pub suggested_order: usize,
    pub time_estimate: Option<String>,
}