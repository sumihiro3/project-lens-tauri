// AIサービス実装

use crate::models::Ticket;
use super::{OpenAIProvider, ClaudeProvider, GeminiProvider, AnalysisResult, Recommendation};

pub enum AIProviderType {
    OpenAI(OpenAIProvider),
    Claude(ClaudeProvider),
    Gemini(GeminiProvider),
}

pub struct AIService {
    provider: AIProviderType,
    config: AIConfig,
}

pub struct AIConfig {
    pub provider_type: String,
    pub model: String,
    pub analysis_interval: u32,
}

impl AIService {
    pub fn new(provider: AIProviderType, config: AIConfig) -> Self {
        Self { provider, config }
    }
    
    pub async fn analyze_tickets(&self, tickets: Vec<Ticket>) -> Result<AnalysisResult, String> {
        match &self.provider {
            AIProviderType::OpenAI(provider) => provider.analyze_tickets(tickets).await,
            AIProviderType::Claude(provider) => provider.analyze_tickets(tickets).await,
            AIProviderType::Gemini(provider) => provider.analyze_tickets(tickets).await,
        }
    }
    
    pub async fn recommend_priorities(&self, analysis: AnalysisResult) -> Result<Vec<Recommendation>, String> {
        match &self.provider {
            AIProviderType::OpenAI(provider) => provider.recommend_priorities(analysis).await,
            AIProviderType::Claude(provider) => provider.recommend_priorities(analysis).await,
            AIProviderType::Gemini(provider) => provider.recommend_priorities(analysis).await,
        }
    }
}