// AI分析結果の定義

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub ticket_analyses: Vec<TicketAnalysis>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketAnalysis {
    pub ticket_id: String,
    pub urgency_score: f32,
    pub complexity_score: f32,
    pub user_relevance_score: f32,
    pub project_weight_factor: f32,
    pub final_priority_score: f32,
    pub category: TaskCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    pub ticket_id: String,
    pub priority_score: f32,
    pub recommendation_reason: String,
    pub category: TaskCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskCategory {
    Urgent,
    Recommended,
    Related,
    Normal,
}