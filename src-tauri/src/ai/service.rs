// AIサービス実装

use crate::models::Ticket;
use super::{AIProvider, AnalysisResult, Recommendation};
use std::sync::Arc;

pub struct AIService {
    provider: Arc<dyn AIProvider>,
    config: AIConfig,
}

pub struct AIConfig {
    pub provider_type: String,
    pub model: String,
    pub analysis_interval: u32,
}

impl AIService {
    pub fn new(provider: Arc<dyn AIProvider>, config: AIConfig) -> Self {
        Self { provider, config }
    }
    
    pub async fn analyze_tickets(&self, tickets: Vec<Ticket>) -> Result<AnalysisResult, String> {
        self.provider.analyze_tickets(tickets).await
    }
    
    pub async fn recommend_priorities(&self, analysis: AnalysisResult) -> Result<Vec<Recommendation>, String> {
        self.provider.recommend_priorities(analysis).await
    }
}