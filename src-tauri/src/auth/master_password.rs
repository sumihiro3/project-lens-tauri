/**
 * マスターパスワード管理機能
 * 
 * アプリケーション全体の暗号化データアクセスを制御するマスターパスワード管理システム。
 * セッション管理、パスワード強度チェック、タイムアウト機能を実装。
 * 
 * セキュリティ仕様:
 * - パスワードハッシュ: PBKDF2-HMAC-SHA256（100,000回イテレーション）
 * - セッション管理: メモリ内での一時的な認証状態保持
 * - タイムアウト: 30分間の非活動でセッション無効化
 * - パスワード強度: 最低8文字、大小英数字と記号の組み合わせ推奨
 */

use crate::crypto::{CryptoService, CryptoError, SecureString};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

/// マスターパスワード管理機能に関するエラー種別
#[derive(Debug, Serialize, Deserialize)]
pub enum MasterPasswordError {
    /// パスワードが設定されていない
    PasswordNotSet,
    /// パスワードが不正
    InvalidPassword,
    /// セッションが無効（未認証またはタイムアウト）
    SessionInvalid,
    /// パスワード強度不足
    WeakPassword(String),
    /// 暗号化処理エラー
    CryptoError(String),
    /// システムエラー
    SystemError(String),
}

impl From<CryptoError> for MasterPasswordError {
    fn from(error: CryptoError) -> Self {
        MasterPasswordError::CryptoError(error.to_string())
    }
}

impl std::fmt::Display for MasterPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MasterPasswordError::PasswordNotSet => write!(f, "マスターパスワードが設定されていません"),
            MasterPasswordError::InvalidPassword => write!(f, "マスターパスワードが正しくありません"),
            MasterPasswordError::SessionInvalid => write!(f, "セッションが無効です。再度認証してください"),
            MasterPasswordError::WeakPassword(reason) => write!(f, "パスワード強度不足: {}", reason),
            MasterPasswordError::CryptoError(msg) => write!(f, "暗号化エラー: {}", msg),
            MasterPasswordError::SystemError(msg) => write!(f, "システムエラー: {}", msg),
        }
    }
}

impl std::error::Error for MasterPasswordError {}

/// セッション状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    /// 未認証
    NotAuthenticated,
    /// 認証済み（有効期限付き）
    Authenticated { expires_at: u64 },
    /// セッション期限切れ
    Expired,
}

/// パスワード強度レベル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordStrength {
    /// 弱い（要件を満たしていない）
    Weak,
    /// 普通（最低要件を満たしている）
    Fair,
    /// 強い（推奨要件を満たしている）
    Strong,
    /// 非常に強い（高セキュリティ要件を満たしている）
    VeryStrong,
}

/// セッション情報の内部管理構造
#[derive(Debug, Clone)]
struct SessionInfo {
    /// 認証済みかどうか
    is_authenticated: bool,
    /// セッション有効期限（UNIX timestamp）
    expires_at: u64,
    /// 最後のアクティビティ時刻
    last_activity: u64,
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            expires_at: 0,
            last_activity: 0,
        }
    }
}

/// マスターパスワード管理システム
/// 
/// アプリケーション全体の暗号化データアクセスを制御するマスターパスワード管理機能。
/// シングルトンパターンで実装され、セッション管理とパスワード認証を提供。
pub struct MasterPasswordManager {
    /// 暗号化サービス
    crypto_service: CryptoService,
    /// セッション情報（スレッドセーフ）
    session: Arc<Mutex<SessionInfo>>,
    /// セッションタイムアウト時間（秒）
    session_timeout_seconds: u64,
    /// マスターパスワードハッシュの保存先（実際にはより安全な場所に保存すべき）
    password_hash_storage: Arc<Mutex<Option<Vec<u8>>>>,
}

impl Default for MasterPasswordManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MasterPasswordManager {
    /// 新しいマスターパスワード管理インスタンスを作成
    /// 
    /// デフォルトで30分のセッションタイムアウトを設定。
    pub fn new() -> Self {
        Self {
            crypto_service: CryptoService::new(),
            session: Arc::new(Mutex::new(SessionInfo::default())),
            session_timeout_seconds: 30 * 60, // 30分
            password_hash_storage: Arc::new(Mutex::new(None)),
        }
    }

    /// カスタムセッションタイムアウトでインスタンスを作成
    /// 
    /// # 引数
    /// * `timeout_seconds` - セッションタイムアウト時間（秒）
    pub fn with_timeout(timeout_seconds: u64) -> Self {
        Self {
            crypto_service: CryptoService::new(),
            session: Arc::new(Mutex::new(SessionInfo::default())),
            session_timeout_seconds: timeout_seconds,
            password_hash_storage: Arc::new(Mutex::new(None)),
        }
    }

    /// マスターパスワードを設定
    /// 
    /// 新しいマスターパスワードを設定し、セキュアにハッシュ化して保存。
    /// パスワード強度チェックを実行し、弱いパスワードの場合は警告。
    /// 
    /// # 引数
    /// * `password` - 設定するマスターパスワード
    /// 
    /// # 戻り値
    /// パスワード強度レベル
    /// 
    /// # エラー
    /// パスワード強度不足またはハッシュ化失敗時
    pub fn set_password(&self, password: &str) -> Result<PasswordStrength, MasterPasswordError> {
        // パスワード強度チェック
        let strength = self.check_password_strength(password);
        
        // 弱いパスワードの場合は設定を拒否
        if matches!(strength, PasswordStrength::Weak) {
            return Err(MasterPasswordError::WeakPassword(
                "パスワードは最低8文字で、大文字・小文字・数字を含む必要があります".to_string()
            ));
        }

        // パスワードをセキュアにハッシュ化
        let secure_password = SecureString::new(password.to_string());
        let password_data = b"master_password_validation_data"; // 固定データでハッシュ化
        
        let password_hash = self.crypto_service.encrypt(
            password_data,
            secure_password.as_str().ok_or(MasterPasswordError::SystemError(
                "パスワード文字列の処理に失敗しました".to_string()
            ))?
        )?;

        // ハッシュをメモリに保存（実際の実装では永続化が必要）
        {
            let mut storage = self.password_hash_storage.lock().map_err(|_| {
                MasterPasswordError::SystemError("ロック取得に失敗しました".to_string())
            })?;
            *storage = Some(password_hash);
        }

        // セッションをクリア（新しいパスワードで再認証が必要）
        self.clear_session()?;

        Ok(strength)
    }

    /// マスターパスワードを検証してセッションを開始
    /// 
    /// 入力されたパスワードを検証し、正しい場合はセッションを開始。
    /// セッション有効期限を設定し、認証状態を管理。
    /// 
    /// # 引数
    /// * `password` - 検証するマスターパスワード
    /// 
    /// # 戻り値
    /// セッション有効期限（UNIX timestamp）
    /// 
    /// # エラー
    /// パスワード未設定、パスワード不正、システムエラー時
    pub fn verify_password(&self, password: &str) -> Result<u64, MasterPasswordError> {
        // パスワードハッシュを取得
        let password_hash = {
            let storage = self.password_hash_storage.lock().map_err(|_| {
                MasterPasswordError::SystemError("ロック取得に失敗しました".to_string())
            })?;
            storage.as_ref().ok_or(MasterPasswordError::PasswordNotSet)?.clone()
        };

        // パスワード検証
        let secure_password = SecureString::new(password.to_string());
        let validation_data = b"master_password_validation_data";
        
        let decrypted = self.crypto_service.decrypt(
            &password_hash,
            secure_password.as_str().ok_or(MasterPasswordError::SystemError(
                "パスワード文字列の処理に失敗しました".to_string()
            ))?
        ).map_err(|_| MasterPasswordError::InvalidPassword)?;

        // データが一致するか確認
        if decrypted != validation_data {
            return Err(MasterPasswordError::InvalidPassword);
        }

        // セッション開始
        let now = self.current_timestamp()?;
        let expires_at = now + self.session_timeout_seconds;
        
        {
            let mut session = self.session.lock().map_err(|_| {
                MasterPasswordError::SystemError("セッションロック取得に失敗しました".to_string())
            })?;
            session.is_authenticated = true;
            session.expires_at = expires_at;
            session.last_activity = now;
        }

        Ok(expires_at)
    }

    /// 現在のセッション状態を確認
    /// 
    /// セッションの認証状態と有効期限を確認し、タイムアウトの場合は自動的にクリア。
    /// 
    /// # 戻り値
    /// 現在のセッション状態
    pub fn get_session_status(&self) -> Result<SessionStatus, MasterPasswordError> {
        let now = self.current_timestamp()?;
        
        let mut session = self.session.lock().map_err(|_| {
            MasterPasswordError::SystemError("セッションロック取得に失敗しました".to_string())
        })?;

        if !session.is_authenticated {
            return Ok(SessionStatus::NotAuthenticated);
        }

        if now > session.expires_at {
            // セッション期限切れ - クリア
            session.is_authenticated = false;
            session.expires_at = 0;
            session.last_activity = 0;
            return Ok(SessionStatus::Expired);
        }

        Ok(SessionStatus::Authenticated { expires_at: session.expires_at })
    }

    /// セッションを延長
    /// 
    /// 認証済みの場合にセッション有効期限を延長。
    /// アクティビティ時刻を更新し、セッションタイムアウトをリセット。
    /// 
    /// # 戻り値
    /// 新しいセッション有効期限（UNIX timestamp）
    /// 
    /// # エラー
    /// セッション無効時
    pub fn extend_session(&self) -> Result<u64, MasterPasswordError> {
        let now = self.current_timestamp()?;
        
        let mut session = self.session.lock().map_err(|_| {
            MasterPasswordError::SystemError("セッションロック取得に失敗しました".to_string())
        })?;

        if !session.is_authenticated || now > session.expires_at {
            return Err(MasterPasswordError::SessionInvalid);
        }

        let new_expires_at = now + self.session_timeout_seconds;
        session.expires_at = new_expires_at;
        session.last_activity = now;

        Ok(new_expires_at)
    }

    /// セッションをクリア
    /// 
    /// 認証状態をリセットし、セッション情報をクリア。
    /// ログアウト時やセキュリティ上の理由でセッションを無効化する場合に使用。
    pub fn clear_session(&self) -> Result<(), MasterPasswordError> {
        let mut session = self.session.lock().map_err(|_| {
            MasterPasswordError::SystemError("セッションロック取得に失敗しました".to_string())
        })?;

        session.is_authenticated = false;
        session.expires_at = 0;
        session.last_activity = 0;

        Ok(())
    }

    /// マスターパスワードが設定済みかどうかを確認
    /// 
    /// # 戻り値
    /// パスワード設定状態
    pub fn is_password_set(&self) -> Result<bool, MasterPasswordError> {
        let storage = self.password_hash_storage.lock().map_err(|_| {
            MasterPasswordError::SystemError("ロック取得に失敗しました".to_string())
        })?;
        Ok(storage.is_some())
    }

    /// 認証済みかどうかを確認（セッション有効性チェック付き）
    /// 
    /// # 戻り値
    /// 認証状態
    pub fn is_authenticated(&self) -> Result<bool, MasterPasswordError> {
        match self.get_session_status()? {
            SessionStatus::Authenticated { .. } => Ok(true),
            _ => Ok(false),
        }
    }

    /// パスワード強度をチェック
    /// 
    /// パスワードの複雑性と安全性を評価し、強度レベルを返す。
    /// 
    /// # 引数
    /// * `password` - チェックするパスワード
    /// 
    /// # 戻り値
    /// パスワード強度レベル
    pub fn check_password_strength(&self, password: &str) -> PasswordStrength {
        let length = password.len();
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());
        
        // 基本要件チェック
        if length < 8 || !has_lowercase || !has_uppercase || !has_digit {
            return PasswordStrength::Weak;
        }

        // 強度判定
        let criteria_met = [has_lowercase, has_uppercase, has_digit, has_special]
            .iter()
            .filter(|&&x| x)
            .count();

        match (length, criteria_met) {
            (12.., 4) => PasswordStrength::VeryStrong,
            (10.., 4) | (12.., 3) => PasswordStrength::Strong,
            (8.., 4) | (10.., 3) => PasswordStrength::Strong,  // 修正: 4つの条件を満たす8文字以上も強い
            (8.., 3) => PasswordStrength::Fair,
            _ => PasswordStrength::Weak,
        }
    }

    /// 現在のUNIXタイムスタンプを取得
    /// 
    /// # 戻り値
    /// 現在のUNIXタイムスタンプ（秒）
    /// 
    /// # エラー
    /// システム時刻取得失敗時
    fn current_timestamp(&self) -> Result<u64, MasterPasswordError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .map_err(|_| MasterPasswordError::SystemError(
                "システム時刻の取得に失敗しました".to_string()
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    /// マスターパスワード設定と検証の基本テスト
    #[test]
    fn test_password_set_and_verify() {
        let manager = MasterPasswordManager::new();
        let password = "TestPassword123!";

        // パスワード設定
        let strength = manager.set_password(password).expect("パスワード設定に失敗");
        assert!(matches!(strength, PasswordStrength::Strong | PasswordStrength::VeryStrong));

        // パスワード設定状態確認
        assert!(manager.is_password_set().expect("設定状態確認に失敗"));

        // パスワード検証
        let expires_at = manager.verify_password(password).expect("パスワード検証に失敗");
        assert!(expires_at > 0);

        // 認証状態確認
        assert!(manager.is_authenticated().expect("認証状態確認に失敗"));
    }

    /// 間違ったパスワードでの検証失敗テスト
    #[test]
    fn test_wrong_password_verification() {
        let manager = MasterPasswordManager::new();
        let correct_password = "CorrectPassword123!";
        let wrong_password = "WrongPassword456!";

        // パスワード設定
        manager.set_password(correct_password).expect("パスワード設定に失敗");

        // 間違ったパスワードで検証
        let result = manager.verify_password(wrong_password);
        assert!(matches!(result, Err(MasterPasswordError::InvalidPassword)));

        // 認証されていないことを確認
        assert!(!manager.is_authenticated().expect("認証状態確認に失敗"));
    }

    /// パスワード強度チェックテスト
    #[test]
    fn test_password_strength_check() {
        let manager = MasterPasswordManager::new();

        // 弱いパスワード
        assert!(matches!(manager.check_password_strength("weak"), PasswordStrength::Weak));
        assert!(matches!(manager.check_password_strength("12345678"), PasswordStrength::Weak));

        // 普通のパスワード
        assert!(matches!(manager.check_password_strength("Password1"), PasswordStrength::Fair));

        // 強いパスワード（10文字・4種類の文字）→ VeryStrong判定される
        assert!(matches!(manager.check_password_strength("Strong123!"), PasswordStrength::Strong));

        // 非常に強いパスワード（12文字以上・4種類の文字）
        assert!(matches!(manager.check_password_strength("VeryStrongPassword1!"), PasswordStrength::VeryStrong));
    }

    /// 弱いパスワード設定の拒否テスト
    #[test]
    fn test_weak_password_rejection() {
        let manager = MasterPasswordManager::new();
        
        let weak_passwords = vec![
            "weak",
            "12345678",
            "password",
            "PASSWORD",
            "Pass123", // 短すぎる
        ];

        for weak_password in weak_passwords {
            let result = manager.set_password(weak_password);
            assert!(matches!(result, Err(MasterPasswordError::WeakPassword(_))));
        }
    }

    /// セッション管理テスト
    #[test]
    fn test_session_management() {
        let manager = MasterPasswordManager::new();
        let password = "SessionTest123!";

        // パスワード設定
        manager.set_password(password).expect("パスワード設定に失敗");

        // 初期状態：未認証
        let status = manager.get_session_status().expect("セッション状態取得に失敗");
        assert!(matches!(status, SessionStatus::NotAuthenticated));

        // 認証
        let expires_at = manager.verify_password(password).expect("パスワード検証に失敗");
        let status = manager.get_session_status().expect("セッション状態取得に失敗");
        assert!(matches!(status, SessionStatus::Authenticated { expires_at: e } if e == expires_at));

        // セッション延長（時間を空けて実行）
        std::thread::sleep(std::time::Duration::from_secs(1));
        let new_expires_at = manager.extend_session().expect("セッション延長に失敗");
        assert!(new_expires_at > expires_at);

        // セッションクリア
        manager.clear_session().expect("セッションクリアに失敗");
        let status = manager.get_session_status().expect("セッション状態取得に失敗");
        assert!(matches!(status, SessionStatus::NotAuthenticated));
    }

    /// セッションタイムアウトテスト
    #[test]
    fn test_session_timeout() {
        // 短いタイムアウト（1秒）で管理インスタンスを作成
        let manager = MasterPasswordManager::with_timeout(1);
        let password = "TimeoutTest123!";

        // パスワード設定と認証
        manager.set_password(password).expect("パスワード設定に失敗");
        manager.verify_password(password).expect("パスワード検証に失敗");

        // 認証状態確認
        assert!(manager.is_authenticated().expect("認証状態確認に失敗"));

        // 2秒待機（タイムアウト時間を超過）
        thread::sleep(Duration::from_secs(2));

        // セッション期限切れの確認
        let status = manager.get_session_status().expect("セッション状態取得に失敗");
        assert!(matches!(status, SessionStatus::Expired));
        assert!(!manager.is_authenticated().expect("認証状態確認に失敗"));
    }

    /// パスワード未設定時の動作テスト
    #[test]
    fn test_no_password_set() {
        let manager = MasterPasswordManager::new();

        // パスワード未設定状態確認
        assert!(!manager.is_password_set().expect("設定状態確認に失敗"));

        // パスワード未設定時の検証
        let result = manager.verify_password("AnyPassword123!");
        assert!(matches!(result, Err(MasterPasswordError::PasswordNotSet)));
    }

    /// セッション無効時の延長失敗テスト
    #[test]
    fn test_extend_invalid_session() {
        let manager = MasterPasswordManager::new();

        // セッション未開始での延長試行
        let result = manager.extend_session();
        assert!(matches!(result, Err(MasterPasswordError::SessionInvalid)));
    }
}