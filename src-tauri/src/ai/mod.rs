// AI統合モジュール
// Mastra AIを使用したチケット分析

pub mod service;
pub mod provider;
pub mod analysis;

pub use service::AIService;
pub use provider::{AIProvider, OpenAIProvider, ClaudeProvider, GeminiProvider};
pub use analysis::{AnalysisResult, Recommendation, TaskCategory};