/**
 * 暗号化モジュール
 * 
 * APIキーなどの機密情報の暗号化・復号化機能を提供。
 * AES-256-GCM認証付き暗号化とPBKDF2キー導出を使用。
 */

pub mod service;

pub use service::{CryptoService, CryptoError, SecureBytes, SecureString};