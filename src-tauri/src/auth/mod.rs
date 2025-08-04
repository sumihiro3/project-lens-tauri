/**
 * 認証モジュール
 * 
 * セキュアな認証情報管理と暗号化データアクセス制御を提供。
 * マスターパスワードによる認証システムとセッション管理機能を実装。
 */

pub mod master_password;

pub use master_password::{
    MasterPasswordManager, 
    MasterPasswordError, 
    SessionStatus,
    PasswordStrength
};