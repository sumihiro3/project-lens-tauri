// AIプロバイダー実装

use async_trait::async_trait;
use crate::models::Ticket;
use super::analysis::{AnalysisResult, Recommendation};

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn analyze_tickets(&self, tickets: Vec<Ticket>) -> Result<AnalysisResult, String>;
    async fn recommend_priorities(&self, analysis: AnalysisResult) -> Result<Vec<Recommendation>, String>;
}

pub struct OpenAIProvider {
    api_key: String,
    model: String,
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn analyze_tickets(&self, _tickets: Vec<Ticket>) -> Result<AnalysisResult, String> {
        // OpenAI実装
        todo!()
    }
    
    async fn recommend_priorities(&self, _analysis: AnalysisResult) -> Result<Vec<Recommendation>, String> {
        // OpenAI実装
        todo!()
    }
}

pub struct ClaudeProvider {
    api_key: String,
    model: String,
}

#[async_trait]
impl AIProvider for ClaudeProvider {
    async fn analyze_tickets(&self, _tickets: Vec<Ticket>) -> Result<AnalysisResult, String> {
        // Claude実装
        todo!()
    }
    
    async fn recommend_priorities(&self, _analysis: AnalysisResult) -> Result<Vec<Recommendation>, String> {
        // Claude実装
        todo!()
    }
}

pub struct GeminiProvider {
    api_key: String,
    model: String,
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn analyze_tickets(&self, _tickets: Vec<Ticket>) -> Result<AnalysisResult, String> {
        // Gemini実装
        todo!()
    }
    
    async fn recommend_priorities(&self, _analysis: AnalysisResult) -> Result<Vec<Recommendation>, String> {
        // Gemini実装
        todo!()
    }
}