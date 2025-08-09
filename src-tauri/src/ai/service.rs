//! AIサービス実装
//! チケット分析とAI推奨機能を提供するサービス層

use crate::models::Ticket;
use super::{OpenAIProvider, ClaudeProvider, GeminiProvider, AnalysisResult, Recommendation};
use super::provider::AIProvider;

/// AIプロバイダーの種類を表す列挙型
/// 
/// 各プロバイダーは独自の実装を持ち、
/// 統一されたインターフェースを通じてアクセスされる
pub enum AIProviderType {
    /// OpenAI GPTプロバイダー
    OpenAI(OpenAIProvider),
    /// Anthropic Claudeプロバイダー
    Claude(ClaudeProvider),
    /// Google Geminiプロバイダー
    Gemini(GeminiProvider),
}

/// AIサービスのメインクラス
/// 
/// 複数のAIプロバイダーを統一的に管理し、
/// チケット分析と優先度推奨機能を提供する
pub struct AIService {
    /// 使用するAIプロバイダー
    provider: AIProviderType,
    /// AI分析の設定情報
    config: AIConfig,
}

/// AI分析の設定情報
/// 
/// プロバイダーの選択、モデル設定、分析間隔等を管理
pub struct AIConfig {
    /// プロバイダーのタイプ名
    pub provider_type: String,
    /// 使用するモデル名
    pub model: String,
    /// 自動分析の実行間隔（分単位）
    pub analysis_interval: u32,
}

impl AIService {
    /// 新しいAIServiceインスタンスを作成
    /// 
    /// # 引数
    /// * `provider` - 使用するAIプロバイダー
    /// * `config` - AI分析設定
    /// 
    /// # 戻り値
    /// 初期化されたAIServiceインスタンス
    pub fn new(provider: AIProviderType, config: AIConfig) -> Self {
        Self { provider, config }
    }
    
    /// チケット群の分析を実行
    /// 
    /// 指定されたチケット群をAIで分析し、
    /// 緊急度、複雑度、関連性などのスコアを算出する
    /// 
    /// # 引数
    /// * `tickets` - 分析対象のチケット一覧
    /// 
    /// # 戻り値
    /// * `Ok(AnalysisResult)` - 分析結果
    /// * `Err(String)` - エラーメッセージ
    pub async fn analyze_tickets(&self, tickets: Vec<Ticket>) -> Result<AnalysisResult, String> {
        match &self.provider {
            AIProviderType::OpenAI(provider) => provider.analyze_tickets(tickets).await,
            AIProviderType::Claude(provider) => provider.analyze_tickets(tickets).await,
            AIProviderType::Gemini(provider) => provider.analyze_tickets(tickets).await,
        }
    }
    
    /// 分析結果に基づく優先度推奨を生成
    /// 
    /// AIによる分析結果を基に、ユーザーが取り組むべき
    /// タスクの優先度と推奨理由を生成する
    /// 
    /// # 引数
    /// * `analysis` - チケット分析結果
    /// 
    /// # 戻り値
    /// * `Ok(Vec<Recommendation>)` - 推奨結果一覧
    /// * `Err(String)` - エラーメッセージ
    pub async fn recommend_priorities(&self, analysis: AnalysisResult) -> Result<Vec<Recommendation>, String> {
        match &self.provider {
            AIProviderType::OpenAI(provider) => provider.recommend_priorities(analysis).await,
            AIProviderType::Claude(provider) => provider.recommend_priorities(analysis).await,
            AIProviderType::Gemini(provider) => provider.recommend_priorities(analysis).await,
        }
    }
}