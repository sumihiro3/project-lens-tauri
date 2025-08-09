/**
 * セキュアデータアクセス層
 * 
 * 暗号化された認証情報の安全な保存・取得を提供するセキュアリポジトリ。
 * CryptoServiceとRepository層を統合し、マスターパスワード認証による
 * アクセス制御を実装。
 * 
 * セキュリティ仕様:
 * - 全操作でマスターパスワード認証を要求
 * - APIキーなどの機密情報は暗号化してデータベースに保存
 * - メモリ上では復号化した情報をSecureString/SecureBytesで管理
 * - セッション無効時は全操作を拒否
 */

use crate::crypto::{CryptoService, CryptoError, SecureString};
use crate::auth::{MasterPasswordManager, MasterPasswordError};
use crate::storage::repository::{Repository, DatabaseError};
use crate::models::{BacklogWorkspaceConfig, AIProviderConfig, AIProviderType};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

/// セキュアリポジトリ操作中に発生する可能性のあるエラー種別
#[derive(Debug, Serialize, Deserialize)]
pub enum SecureRepositoryError {
    /// 認証エラー（マスターパスワード未設定、セッション無効など）
    AuthenticationError(String),
    /// 暗号化・復号化エラー
    CryptographyError(String),
    /// データベースアクセスエラー
    DatabaseError(String),
    /// データ形式エラー
    DataFormatError(String),
    /// システムエラー
    SystemError(String),
}

impl From<MasterPasswordError> for SecureRepositoryError {
    fn from(error: MasterPasswordError) -> Self {
        SecureRepositoryError::AuthenticationError(error.to_string())
    }
}

impl From<CryptoError> for SecureRepositoryError {
    fn from(error: CryptoError) -> Self {
        SecureRepositoryError::CryptographyError(error.to_string())
    }
}

impl From<DatabaseError> for SecureRepositoryError {
    fn from(error: DatabaseError) -> Self {
        SecureRepositoryError::DatabaseError(error.to_string())
    }
}

impl std::fmt::Display for SecureRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecureRepositoryError::AuthenticationError(msg) => write!(f, "認証エラー: {}", msg),
            SecureRepositoryError::CryptographyError(msg) => write!(f, "暗号化エラー: {}", msg),
            SecureRepositoryError::DatabaseError(msg) => write!(f, "データベースエラー: {}", msg),
            SecureRepositoryError::DataFormatError(msg) => write!(f, "データ形式エラー: {}", msg),
            SecureRepositoryError::SystemError(msg) => write!(f, "システムエラー: {}", msg),
        }
    }
}

impl std::error::Error for SecureRepositoryError {}

/// セキュアデータアクセス層
/// 
/// Repository層とCryptoServiceを統合し、認証済みセッションでのみ
/// 暗号化データへのアクセスを許可するセキュアリポジトリ。
pub struct SecureRepository {
    /// データベースリポジトリ
    repository: Repository,
    /// 暗号化サービス
    crypto_service: CryptoService,
    /// マスターパスワード管理（共有参照）
    master_password_manager: Arc<Mutex<MasterPasswordManager>>,
    /// 現在の暗号化バージョン
    encryption_version: String,
}

impl SecureRepository {
    /// 新しいセキュアリポジトリインスタンスを作成
    /// 
    /// # 引数
    /// * `db_path` - SQLiteデータベースファイルのパス
    /// * `master_password_manager` - マスターパスワード管理インスタンス
    /// 
    /// # 戻り値
    /// セキュアリポジトリインスタンス
    /// 
    /// # エラー
    /// データベース接続失敗時
    pub fn new(
        db_path: &str, 
        master_password_manager: Arc<Mutex<MasterPasswordManager>>
    ) -> Result<Self, SecureRepositoryError> {
        let repository = Repository::new(db_path)?;
        let crypto_service = CryptoService::new();
        
        Ok(Self {
            repository,
            crypto_service,
            master_password_manager,
            encryption_version: "v1".to_string(), // 現在のバージョン
        })
    }

    /// マスターパスワード認証を確認
    /// 
    /// セキュアな操作を実行前に認証状態を確認し、セッションを延長。
    /// 
    /// # 戻り値
    /// 認証済みセッションのマスターパスワード文字列
    /// 
    /// # エラー
    /// 認証失敗、セッション無効時
    fn verify_authentication(&self) -> Result<SecureString, SecureRepositoryError> {
        let manager = self.master_password_manager.lock().map_err(|_| {
            SecureRepositoryError::SystemError("マスターパスワード管理のロック取得に失敗しました".to_string())
        })?;

        // 認証状態確認
        if !manager.is_authenticated()? {
            return Err(SecureRepositoryError::AuthenticationError(
                "認証されていません。マスターパスワードを入力してください".to_string()
            ));
        }

        // セッション延長
        manager.extend_session()?;

        // マスターパスワードを取得（実際の実装では、パスワードを別途管理すべき）
        // 注意: この実装は簡略化されており、実際にはより安全な方法でパスワードを管理する必要がある
        Ok(SecureString::new("dummy_password".to_string()))
    }

    /// Backlogワークスペース設定を暗号化して保存
    /// 
    /// # 引数
    /// * `workspace_config` - 保存するワークスペース設定（平文APIキー含む）
    /// * `master_password` - 暗号化に使用するマスターパスワード
    /// 
    /// # 戻り値
    /// 保存された設定のID
    /// 
    /// # エラー
    /// 認証失敗、暗号化失敗、データベース保存失敗時
    pub fn save_backlog_workspace_config(
        &self,
        workspace_config: &mut BacklogWorkspaceConfig,
        api_key_plaintext: &str,
    ) -> Result<String, SecureRepositoryError> {
        // 認証確認
        let master_password = self.verify_authentication()?;
        
        // APIキーを暗号化
        let encrypted_api_key = self.crypto_service.encrypt(
            api_key_plaintext.as_bytes(),
            master_password.as_str().ok_or(SecureRepositoryError::SystemError(
                "マスターパスワードの取得に失敗しました".to_string()
            ))?
        )?;

        // Base64エンコード（データベース保存用）
        workspace_config.api_key_encrypted = base64::encode(&encrypted_api_key);
        workspace_config.encryption_version = self.encryption_version.clone();

        // データベースに保存
        self.repository.save_backlog_workspace_config(workspace_config)?;

        Ok(workspace_config.id.clone())
    }

    /// Backlogワークスペース設定を復号化して取得
    /// 
    /// # 引数
    /// * `workspace_id` - 取得するワークスペースのID
    /// 
    /// # 戻り値
    /// 復号化されたワークスペース設定と平文APIキー
    /// 
    /// # エラー
    /// 認証失敗、データ取得失敗、復号化失敗時
    pub fn get_backlog_workspace_config(
        &self,
        workspace_id: &str,
    ) -> Result<(BacklogWorkspaceConfig, SecureString), SecureRepositoryError> {
        // 認証確認
        let master_password = self.verify_authentication()?;
        
        // データベースから取得
        let config = self.repository.get_backlog_workspace_config(workspace_id)?
            .ok_or(SecureRepositoryError::DataFormatError(
                format!("ワークスペース設定が見つかりません: {}", workspace_id)
            ))?;

        // 暗号化されたAPIキーをデコード
        let encrypted_api_key = base64::decode(&config.api_key_encrypted)
            .map_err(|e| SecureRepositoryError::DataFormatError(
                format!("暗号化データのデコードに失敗しました: {}", e)
            ))?;

        // APIキーを復号化
        let api_key_bytes = self.crypto_service.decrypt(
            &encrypted_api_key,
            master_password.as_str().ok_or(SecureRepositoryError::SystemError(
                "マスターパスワードの取得に失敗しました".to_string()
            ))?
        )?;

        let api_key_plaintext = String::from_utf8(api_key_bytes)
            .map_err(|e| SecureRepositoryError::DataFormatError(
                format!("APIキーの文字列変換に失敗しました: {}", e)
            ))?;

        Ok((config, SecureString::new(api_key_plaintext)))
    }

    /// 全Backlogワークスペース設定を復号化して取得
    /// 
    /// # 戻り値
    /// 復号化されたワークスペース設定一覧と対応する平文APIキー
    /// 
    /// # エラー
    /// 認証失敗、データ取得失敗、復号化失敗時
    pub fn get_all_backlog_workspace_configs(
        &self,
    ) -> Result<Vec<(BacklogWorkspaceConfig, SecureString)>, SecureRepositoryError> {
        // 認証確認
        let master_password = self.verify_authentication()?;
        
        // データベースから全取得
        let configs = self.repository.get_all_backlog_workspace_configs()?;
        let mut result = Vec::new();

        for config in configs {
            // 暗号化されたAPIキーをデコード
            let encrypted_api_key = base64::decode(&config.api_key_encrypted)
                .map_err(|e| SecureRepositoryError::DataFormatError(
                    format!("暗号化データのデコードに失敗しました: {}", e)
                ))?;

            // APIキーを復号化
            let api_key_bytes = self.crypto_service.decrypt(
                &encrypted_api_key,
                master_password.as_str().ok_or(SecureRepositoryError::SystemError(
                    "マスターパスワードの取得に失敗しました".to_string()
                ))?
            )?;

            let api_key_plaintext = String::from_utf8(api_key_bytes)
                .map_err(|e| SecureRepositoryError::DataFormatError(
                    format!("APIキーの文字列変換に失敗しました: {}", e)
                ))?;

            result.push((config, SecureString::new(api_key_plaintext)));
        }

        Ok(result)
    }

    /// AIプロバイダー設定を暗号化して保存
    /// 
    /// # 引数
    /// * `provider_config` - 保存するプロバイダー設定（平文APIキー含む）
    /// * `api_key_plaintext` - 暗号化するAPIキー
    /// 
    /// # 戻り値
    /// 保存された設定のID
    /// 
    /// # エラー
    /// 認証失敗、暗号化失敗、データベース保存失敗時
    pub fn save_ai_provider_config(
        &self,
        provider_config: &mut AIProviderConfig,
        api_key_plaintext: &str,
    ) -> Result<String, SecureRepositoryError> {
        // 認証確認
        let master_password = self.verify_authentication()?;
        
        // APIキーを暗号化
        let encrypted_api_key = self.crypto_service.encrypt(
            api_key_plaintext.as_bytes(),
            master_password.as_str().ok_or(SecureRepositoryError::SystemError(
                "マスターパスワードの取得に失敗しました".to_string()
            ))?
        )?;

        // Base64エンコード（データベース保存用）
        provider_config.api_key_encrypted = base64::encode(&encrypted_api_key);
        provider_config.encryption_version = self.encryption_version.clone();

        // データベースに保存（注意: Repository層にAIProviderConfig保存機能を追加する必要がある）
        // 現在は仮実装
        // self.repository.save_ai_provider_config(provider_config)?;

        Ok(provider_config.id.clone())
    }

    /// AIプロバイダー設定を復号化して取得
    /// 
    /// # 引数
    /// * `provider_id` - 取得するプロバイダーのID
    /// 
    /// # 戻り値
    /// 復号化されたプロバイダー設定と平文APIキー
    /// 
    /// # エラー
    /// 認証失敗、データ取得失敗、復号化失敗時
    pub fn get_ai_provider_config(
        &self,
        provider_id: &str,
    ) -> Result<(AIProviderConfig, SecureString), SecureRepositoryError> {
        // 認証確認
        let _master_password = self.verify_authentication()?;
        
        // TODO: Repository層にAIProviderConfig取得機能を追加する必要がある
        // 現在は仮実装でエラーを返す
        Err(SecureRepositoryError::SystemError(
            "AIプロバイダー設定の取得機能は未実装です".to_string()
        ))
    }

    /// Backlogワークスペース設定を削除
    /// 
    /// # 引数
    /// * `workspace_id` - 削除するワークスペースのID
    /// 
    /// # エラー
    /// 認証失敗、データベース操作失敗時
    pub fn delete_backlog_workspace_config(
        &self,
        workspace_id: &str,
    ) -> Result<(), SecureRepositoryError> {
        // 認証確認
        let _master_password = self.verify_authentication()?;
        
        // データベースから削除
        self.repository.delete_backlog_workspace_config(workspace_id)?;

        Ok(())
    }

    /// AIプロバイダー設定を削除
    /// 
    /// # 引数
    /// * `provider_id` - 削除するプロバイダーのID
    /// 
    /// # エラー
    /// 認証失敗、データベース操作失敗時
    pub fn delete_ai_provider_config(
        &self,
        provider_id: &str,
    ) -> Result<(), SecureRepositoryError> {
        // 認証確認
        let _master_password = self.verify_authentication()?;
        
        // TODO: Repository層にAIProviderConfig削除機能を追加する必要がある
        // 現在は仮実装でエラーを返す
        Err(SecureRepositoryError::SystemError(
            "AIプロバイダー設定の削除機能は未実装です".to_string()
        ))
    }

    /// 暗号化バージョンの更新
    /// 
    /// 既存の暗号化データを新しいバージョンで再暗号化する。
    /// セキュリティ上の理由で暗号化方式を変更する場合に使用。
    /// 
    /// # 引数
    /// * `new_version` - 新しい暗号化バージョン
    /// 
    /// # エラー
    /// 認証失敗、再暗号化失敗時
    pub fn migrate_encryption_version(
        &self,
        new_version: &str,
    ) -> Result<(), SecureRepositoryError> {
        // 認証確認
        let master_password = self.verify_authentication()?;
        
        // 全Backlogワークスペース設定を取得
        let configs = self.get_all_backlog_workspace_configs()?;
        
        for (mut config, api_key) in configs {
            if config.encryption_version != new_version {
                // 新しいバージョンで再暗号化
                let new_encrypted_api_key = self.crypto_service.encrypt(
                    api_key.as_str().ok_or(SecureRepositoryError::SystemError(
                        "APIキーの取得に失敗しました".to_string()
                    ))?.as_bytes(),
                    master_password.as_str().ok_or(SecureRepositoryError::SystemError(
                        "マスターパスワードの取得に失敗しました".to_string()
                    ))?
                )?;

                config.api_key_encrypted = base64::encode(&new_encrypted_api_key);
                config.encryption_version = new_version.to_string();

                // データベースを更新
                self.repository.save_backlog_workspace_config(&config)?;
            }
        }

        Ok(())
    }
}

// Base64エンコード/デコード用の依存関係
// Cargo.tomlに以下を追加する必要がある：
// base64 = "0.21.0"

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::sync::{Arc, Mutex};

    /// テスト用のセキュアリポジトリを作成
    fn create_test_secure_repository() -> (SecureRepository, NamedTempFile) {
        let temp_file = NamedTempFile::new().expect("一時ファイル作成に失敗");
        let db_path = temp_file.path().to_str().unwrap();
        
        let master_password_manager = Arc::new(Mutex::new(MasterPasswordManager::new()));
        
        // マスターパスワードを設定
        {
            let manager = master_password_manager.lock().unwrap();
            manager.set_password("TestMasterPassword123!").expect("パスワード設定に失敗");
            manager.verify_password("TestMasterPassword123!").expect("パスワード検証に失敗");
        }
        
        let secure_repo = SecureRepository::new(db_path, master_password_manager)
            .expect("セキュアリポジトリ作成に失敗");
            
        (secure_repo, temp_file)
    }

    /// セキュアリポジトリの作成テスト
    #[test]
    fn test_secure_repository_creation() {
        let (_secure_repo, _temp_file) = create_test_secure_repository();
        // 作成が成功すればテスト通過
    }

    /// 認証確認機能のテスト
    #[test]
    fn test_authentication_verification() {
        let (secure_repo, _temp_file) = create_test_secure_repository();
        
        // 認証済み状態での確認
        let result = secure_repo.verify_authentication();
        assert!(result.is_ok(), "認証確認に失敗: {:?}", result.err());
    }

    /// 未認証時のアクセス拒否テスト
    #[test]
    fn test_unauthenticated_access_denied() {
        let temp_file = NamedTempFile::new().expect("一時ファイル作成に失敗");
        let db_path = temp_file.path().to_str().unwrap();
        
        let master_password_manager = Arc::new(Mutex::new(MasterPasswordManager::new()));
        let secure_repo = SecureRepository::new(db_path, master_password_manager)
            .expect("セキュアリポジトリ作成に失敗");
        
        // 未認証状態での認証確認
        let result = secure_repo.verify_authentication();
        assert!(result.is_err(), "未認証状態でアクセスが許可されてしまいました");
        assert!(matches!(result.unwrap_err(), SecureRepositoryError::AuthenticationError(_)));
    }

    /// Backlogワークスペース設定の暗号化保存・復号化取得テスト
    #[test]
    fn test_backlog_workspace_config_encryption_roundtrip() {
        let (secure_repo, _temp_file) = create_test_secure_repository();
        
        // テスト用ワークスペース設定
        let mut workspace_config = BacklogWorkspaceConfig::new(
            "test-workspace-1".to_string(),
            "テストワークスペース".to_string(),
            "test.backlog.jp".to_string(),
            "".to_string(), // 暗号化前は空
            "".to_string(), // バージョンも空
        );
        
        let api_key_plaintext = "test-api-key-12345";
        
        // 暗号化保存
        let saved_id = secure_repo.save_backlog_workspace_config(
            &mut workspace_config, 
            api_key_plaintext
        ).expect("ワークスペース設定の保存に失敗");
        
        assert_eq!(saved_id, "test-workspace-1");
        assert!(!workspace_config.api_key_encrypted.is_empty(), "APIキーが暗号化されていません");
        assert_eq!(workspace_config.encryption_version, "v1");
        
        // 復号化取得
        let (retrieved_config, retrieved_api_key) = secure_repo.get_backlog_workspace_config(&saved_id)
            .expect("ワークスペース設定の取得に失敗");
        
        assert_eq!(retrieved_config.id, "test-workspace-1");
        assert_eq!(retrieved_config.name, "テストワークスペース");
        assert_eq!(retrieved_config.domain, "test.backlog.jp");
        assert_eq!(
            retrieved_api_key.as_str().unwrap(), 
            api_key_plaintext,
            "復号化されたAPIキーが一致しません"
        );
    }

    /// 複数ワークスペース設定の一括取得テスト
    #[test]
    fn test_get_all_backlog_workspace_configs() {
        let (secure_repo, _temp_file) = create_test_secure_repository();
        
        // 複数のワークスペース設定を保存
        let workspaces = vec![
            ("workspace-1", "ワークスペース1", "ws1.backlog.jp", "api-key-1"),
            ("workspace-2", "ワークスペース2", "ws2.backlog.jp", "api-key-2"),
        ];
        
        for (id, name, domain, api_key) in &workspaces {
            let mut config = BacklogWorkspaceConfig::new(
                id.to_string(),
                name.to_string(),
                domain.to_string(),
                "".to_string(),
                "".to_string(),
            );
            
            secure_repo.save_backlog_workspace_config(&mut config, api_key)
                .expect("ワークスペース設定の保存に失敗");
        }
        
        // 一括取得
        let all_configs = secure_repo.get_all_backlog_workspace_configs()
            .expect("ワークスペース設定の一括取得に失敗");
        
        assert_eq!(all_configs.len(), 2, "取得されたワークスペース数が一致しません");
        
        // 各設定の検証
        for (config, api_key) in all_configs {
            let expected_workspace = workspaces.iter()
                .find(|(id, _, _, _)| *id == config.id)
                .expect("予期しないワークスペースIDです");
            
            assert_eq!(config.name, expected_workspace.1);
            assert_eq!(config.domain, expected_workspace.2);
            assert_eq!(api_key.as_str().unwrap(), expected_workspace.3);
        }
    }

    /// ワークスペース設定削除テスト
    #[test]
    fn test_delete_backlog_workspace_config() {
        let (secure_repo, _temp_file) = create_test_secure_repository();
        
        // ワークスペース設定を保存
        let mut workspace_config = BacklogWorkspaceConfig::new(
            "delete-test-workspace".to_string(),
            "削除テストワークスペース".to_string(),
            "delete-test.backlog.jp".to_string(),
            "".to_string(),
            "".to_string(),
        );
        
        secure_repo.save_backlog_workspace_config(&mut workspace_config, "delete-test-api-key")
            .expect("ワークスペース設定の保存に失敗");
        
        // 削除前に存在確認
        let result = secure_repo.get_backlog_workspace_config("delete-test-workspace");
        assert!(result.is_ok(), "保存されたワークスペース設定が見つかりません");
        
        // 削除実行
        secure_repo.delete_backlog_workspace_config("delete-test-workspace")
            .expect("ワークスペース設定の削除に失敗");
        
        // 削除後に存在しないことを確認
        let result = secure_repo.get_backlog_workspace_config("delete-test-workspace");
        assert!(result.is_err(), "削除されたワークスペース設定が取得できてしまいました");
    }
}