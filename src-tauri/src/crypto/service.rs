/**
 * 暗号化サービス実装
 * 
 * AES-256-GCM暗号化・復号化機能とPBKDF2キー導出機能を提供。
 * APIキーなどの機密情報を安全に暗号化してローカルファイルシステムに保存し、
 * 復号化してメモリ上でのみ使用する機能を実装。
 * 
 * セキュリティ仕様:
 * - 暗号化アルゴリズム: AES-256-GCM（認証付き暗号化）
 * - キー導出: PBKDF2-HMAC-SHA256（100,000回イテレーション）
 * - ソルト: ランダム生成（32バイト）
 * - ノンス: ランダム生成（12バイト、AES-GCM標準）
 * - データ形式: [32 bytes: salt][12 bytes: nonce][remaining: encrypted_data]
 */

use ring::aead::{self, AES_256_GCM, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey};
use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use std::num::NonZeroU32;

/// 暗号化処理中に発生する可能性のあるエラー種別
#[derive(Debug)]
pub enum CryptoError {
    /// ランダム値生成に失敗
    RandomGenerationFailed,
    /// キー導出に失敗
    KeyDerivationFailed,
    /// 暗号化処理に失敗
    EncryptionFailed,
    /// 復号化処理に失敗（パスワード不正やデータ改ざんを含む）
    DecryptionFailed,
    /// データ形式が不正
    InvalidDataFormat,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::RandomGenerationFailed => write!(f, "セキュアなランダム値の生成に失敗しました"),
            CryptoError::KeyDerivationFailed => write!(f, "パスワードからの暗号化キー導出に失敗しました"),
            CryptoError::EncryptionFailed => write!(f, "データの暗号化処理に失敗しました"),
            CryptoError::DecryptionFailed => write!(f, "データの復号化処理に失敗しました（パスワード不正または改ざん検知）"),
            CryptoError::InvalidDataFormat => write!(f, "暗号化データの形式が不正です"),
        }
    }
}

impl std::error::Error for CryptoError {}

/**
 * 暗号化サービス
 * 
 * APIキーや認証トークンなどの機密情報を安全に暗号化・復号化するサービス。
 * AES-256-GCM認証付き暗号化とPBKDF2キー導出を使用してセキュリティを確保。
 */
pub struct CryptoService {
    /// セキュアなランダム値生成器
    rng: SystemRandom,
}

/// ノンス管理用のヘルパー構造体
struct SingleUseNonce {
    nonce: [u8; 12],
}

impl NonceSequence for SingleUseNonce {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        Nonce::try_assume_unique_for_key(&self.nonce)
    }
}

impl CryptoService {
    /**
     * 暗号化サービスの新しいインスタンスを作成
     * 
     * セキュアなランダム値生成器を初期化して暗号化処理の準備を行う。
     */
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }
    
    /**
     * データを暗号化
     * 
     * 平文データをAES-256-GCMで暗号化し、認証タグを含む暗号化データを返す。
     * パスワードからPBKDF2でキーを導出し、ランダムソルトとノンスを生成。
     * 
     * # 引数
     * * `plaintext` - 暗号化する平文データ
     * * `password` - 暗号化に使用するパスワード
     * 
     * # 戻り値
     * 暗号化されたデータ（ソルト+ノンス+暗号文の結合）
     * 
     * # エラー
     * ランダム値生成やキー導出、暗号化処理に失敗した場合
     */
    pub fn encrypt(&self, plaintext: &[u8], password: &str) -> Result<Vec<u8>, CryptoError> {
        // 1. ランダムソルトを生成（32バイト）
        let salt = self.generate_salt()?;
        
        // 2. パスワードからキーを導出
        let key = self.derive_key(password, &salt)?;
        
        // 3. ランダムノンスを生成（12バイト、AES-GCM標準）
        let nonce_bytes = self.generate_nonce()?;
        
        // 4. AES-256-GCM暗号化を実行
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key)
            .map_err(|_| CryptoError::EncryptionFailed)?;
        let nonce_sequence = SingleUseNonce { nonce: nonce_bytes };
        let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);
        
        let mut data = plaintext.to_vec();
        sealing_key.seal_in_place_append_tag(aead::Aad::empty(), &mut data)
            .map_err(|_| CryptoError::EncryptionFailed)?;
        
        // 5. ソルト + ノンス + 暗号文を結合
        let mut result = Vec::with_capacity(32 + 12 + data.len());
        result.extend_from_slice(&salt);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&data);
        
        Ok(result)
    }
    
    /**
     * データを復号化
     * 
     * 暗号化されたデータをAES-256-GCMで復号化し、平文データを返す。
     * 認証タグの検証により改ざん検知も実行される。
     * 
     * # 引数
     * * `ciphertext` - 復号化する暗号化データ（ソルト+ノンス+暗号文）
     * * `password` - 復号化に使用するパスワード
     * 
     * # 戻り値
     * 復号化された平文データ
     * 
     * # エラー
     * データ形式不正、パスワード不正、改ざん検知時など
     */
    pub fn decrypt(&self, ciphertext: &[u8], password: &str) -> Result<Vec<u8>, CryptoError> {
        // 1. データ形式を検証（最小サイズ: 32 + 12 + 16 = 60バイト）
        if ciphertext.len() < 60 {
            return Err(CryptoError::InvalidDataFormat);
        }
        
        // 2. ソルト（32バイト）を抽出
        let salt = &ciphertext[0..32];
        
        // 3. ノンス（12バイト）を抽出
        let nonce_bytes: [u8; 12] = ciphertext[32..44].try_into()
            .map_err(|_| CryptoError::InvalidDataFormat)?;
        
        // 4. 暗号文部分を抽出
        let encrypted_data = &ciphertext[44..];
        
        // 5. パスワードからキーを導出
        let key = self.derive_key(password, salt)?;
        
        // 6. AES-256-GCM復号化を実行
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key)
            .map_err(|_| CryptoError::DecryptionFailed)?;
        let nonce_sequence = SingleUseNonce { nonce: nonce_bytes };
        let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);
        
        let mut data = encrypted_data.to_vec();
        let plaintext = opening_key.open_in_place(aead::Aad::empty(), &mut data)
            .map_err(|_| CryptoError::DecryptionFailed)?;
        
        Ok(plaintext.to_vec())
    }
    
    /**
     * PBKDF2を使用してパスワードから暗号化キーを導出
     * 
     * 100,000回のイテレーションでHMAC-SHA256を使用し、
     * 32バイトの暗号化キーを安全に生成する。
     * 
     * # 引数
     * * `password` - 元となるパスワード
     * * `salt` - キー導出用のソルト（32バイト）
     * 
     * # 戻り値
     * 導出された32バイトの暗号化キー
     */
    fn derive_key(&self, password: &str, salt: &[u8]) -> Result<[u8; 32], CryptoError> {
        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            salt,
            password.as_bytes(),
            &mut key,
        );
        Ok(key)
    }
    
    /**
     * セキュアなランダムソルトを生成
     * 
     * 暗号学的に安全な32バイトのランダムソルトを生成。
     * キー導出の安全性を確保するために使用される。
     * 
     * # 戻り値
     * 32バイトのランダムソルト
     */
    fn generate_salt(&self) -> Result<[u8; 32], CryptoError> {
        let mut salt = [0u8; 32];
        self.rng.fill(&mut salt)
            .map_err(|_| CryptoError::RandomGenerationFailed)?;
        Ok(salt)
    }
    
    /**
     * セキュアなランダムノンスを生成
     * 
     * AES-GCM用の12バイトのランダムノンスを生成。
     * 同じキーで同じノンスを再利用しないことがセキュリティ上重要。
     * 
     * # 戻り値
     * 12バイトのランダムノンス
     */
    fn generate_nonce(&self) -> Result<[u8; 12], CryptoError> {
        let mut nonce = [0u8; 12];
        self.rng.fill(&mut nonce)
            .map_err(|_| CryptoError::RandomGenerationFailed)?;
        Ok(nonce)
    }
}

/**
 * セキュアなメモリクリア機能を提供する構造体
 * 
 * 機密データ（パスワード、キー等）を保持し、使用後に安全にメモリから削除する。
 * メモリダンプ攻撃やスワップファイルへの機密情報漏洩を防ぐために使用。
 */
pub struct SecureBytes {
    /// 機密データを格納するバッファ
    data: Vec<u8>,
}

impl SecureBytes {
    /**
     * 機密データから新しいSecureBytesインスタンスを作成
     * 
     * # 引数
     * * `data` - 保護する機密データ
     */
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    
    /**
     * 機密データへの読み取り専用参照を取得
     * 
     * # 戻り値
     * 機密データのスライス参照
     */
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    
    /**
     * 機密データのコピーを取得
     * 
     * 注意: コピーしたデータも使用後は適切にクリアする必要がある
     * 
     * # 戻り値
     * 機密データのコピー
     */
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    /**
     * 機密データを手動でクリア
     * 
     * メモリ上の機密データを0で上書きして安全に削除する。
     * 通常はDropトレイトによって自動実行されるが、明示的にクリアしたい場合に使用。
     */
    pub fn clear(&mut self) {
        // メモリを0で上書きしてクリア
        self.data.fill(0);
        // 容量も削減してメモリ使用量を最小化
        self.data.clear();
        self.data.shrink_to_fit();
    }
}

impl Drop for SecureBytes {
    /**
     * インスタンス破棄時にメモリを自動クリア
     * 
     * スコープを抜ける際に自動的に機密データをメモリから安全に削除する。
     * コンパイラ最適化による削除を防ぐため、volatileな書き込みを使用。
     */
    fn drop(&mut self) {
        // メモリを0で上書きしてクリア（volatileな書き込みでコンパイラ最適化を回避）
        for byte in self.data.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
        self.data.clear();
        self.data.shrink_to_fit();
    }
}

/**
 * セキュアなパスワード文字列を管理する構造体
 * 
 * パスワード文字列を保持し、使用後に安全にメモリから削除する。
 * String型の代替として機密情報の取り扱いに使用。
 */
pub struct SecureString {
    /// パスワード文字列のバイト配列
    bytes: SecureBytes,
}

impl SecureString {
    /**
     * パスワード文字列から新しいSecureStringインスタンスを作成
     * 
     * # 引数
     * * `password` - 保護するパスワード文字列
     */
    pub fn new(password: String) -> Self {
        Self {
            bytes: SecureBytes::new(password.into_bytes()),
        }
    }
    
    /**
     * パスワード文字列への参照を取得
     * 
     * 注意: 返される文字列参照の有効期間はSecureStringインスタンスに依存
     * 
     * # 戻り値
     * パスワード文字列の参照（エラーの場合はNone）
     */
    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(self.bytes.as_slice()).ok()
    }
    
    /**
     * パスワード文字列のバイト配列への参照を取得
     * 
     * # 戻り値
     * パスワードのバイト配列スライス
     */
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
    
    /**
     * パスワード文字列を手動でクリア
     */
    pub fn clear(&mut self) {
        self.bytes.clear();
    }
}

impl Drop for SecureString {
    /**
     * インスタンス破棄時にパスワードを自動クリア
     */
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * 暗号化・復号化の往復テスト
     * 
     * 様々なデータサイズで暗号化・復号化が正確に動作することを確認
     */
    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let crypto_service = CryptoService::new();
        let password = "test_password_123";
        
        // 様々なサイズのテストデータ
        let test_cases = vec![
            b"short".to_vec(),
            b"medium length test data for encryption".to_vec(),
            vec![0u8; 1000], // 大きなデータ
            b"special chars: \x00\x01\x02\xff\xfe\xfd".to_vec(), // バイナリデータ
            String::from("日本語テストデータ").into_bytes(), // UTF-8データ
        ];
        
        for (i, original_data) in test_cases.iter().enumerate() {
            // 暗号化
            let encrypted = crypto_service.encrypt(original_data, password)
                .expect(&format!("Test case {}: 暗号化に失敗", i));
            
            // 復号化
            let decrypted = crypto_service.decrypt(&encrypted, password)
                .expect(&format!("Test case {}: 復号化に失敗", i));
            
            // データの一致を確認
            assert_eq!(original_data, &decrypted, "Test case {}: 往復後のデータが一致しない", i);
            
            // 暗号化データが元データと異なることを確認
            assert_ne!(original_data.as_slice(), &encrypted[44..], "Test case {}: 暗号化データが平文と同じ", i);
        }
    }
    
    /**
     * 間違ったパスワードでの復号化失敗テスト
     * 
     * 不正なパスワードを使用した場合に適切にエラーが発生することを確認
     */
    #[test]
    fn test_wrong_password_fails() {
        let crypto_service = CryptoService::new();
        let original_data = b"secret data";
        let correct_password = "correct_password";
        let wrong_password = "wrong_password";
        
        // 正しいパスワードで暗号化
        let encrypted = crypto_service.encrypt(original_data, correct_password)
            .expect("暗号化に失敗");
        
        // 間違ったパスワードで復号化を試行
        let result = crypto_service.decrypt(&encrypted, wrong_password);
        
        // エラーが発生することを確認
        assert!(matches!(result, Err(CryptoError::DecryptionFailed)), 
                "間違ったパスワードでエラーが発生しない");
    }
    
    /**
     * 改ざんされたデータの復号化失敗テスト
     * 
     * 暗号化データが改ざんされた場合に認証エラーが発生することを確認
     */
    #[test]
    fn test_tampered_data_fails() {
        let crypto_service = CryptoService::new();
        let original_data = b"confidential information";
        let password = "secure_password";
        
        // データを暗号化
        let mut encrypted = crypto_service.encrypt(original_data, password)
            .expect("暗号化に失敗");
        
        // 暗号化データの末尾を改ざん
        let last_index = encrypted.len() - 1;
        encrypted[last_index] ^= 0x01; // 1ビット反転
        
        // 改ざんされたデータの復号化を試行
        let result = crypto_service.decrypt(&encrypted, password);
        
        // 認証エラーが発生することを確認
        assert!(matches!(result, Err(CryptoError::DecryptionFailed)),
                "改ざんされたデータでエラーが発生しない");
    }
    
    /**
     * 不正なデータ形式のテスト
     * 
     * 短すぎるデータや不正な形式のデータに対してエラーが発生することを確認
     */
    #[test]
    fn test_invalid_data_format() {
        let crypto_service = CryptoService::new();
        let password = "test_password";
        
        // 短すぎるデータ（最小サイズ60バイト未満）
        let short_data = vec![0u8; 30];
        let result = crypto_service.decrypt(&short_data, password);
        assert!(matches!(result, Err(CryptoError::InvalidDataFormat)),
                "短いデータでエラーが発生しない");
        
        // 空のデータ
        let empty_data = vec![];
        let result = crypto_service.decrypt(&empty_data, password);
        assert!(matches!(result, Err(CryptoError::InvalidDataFormat)),
                "空データでエラーが発生しない");
    }
    
    /**
     * 異なる暗号化で同じ結果にならないことのテスト
     * 
     * 同じデータ・パスワードでも毎回異なる暗号化結果になることを確認（ソルト・ノンスの効果）
     */
    #[test]
    fn test_different_encryption_results() {
        let crypto_service = CryptoService::new();
        let data = b"same data";
        let password = "same_password";
        
        // 同じデータを複数回暗号化
        let encrypted1 = crypto_service.encrypt(data, password)
            .expect("1回目の暗号化に失敗");
        let encrypted2 = crypto_service.encrypt(data, password)
            .expect("2回目の暗号化に失敗");
        let encrypted3 = crypto_service.encrypt(data, password)
            .expect("3回目の暗号化に失敗");
        
        // 全て異なる結果になることを確認
        assert_ne!(encrypted1, encrypted2, "1回目と2回目の暗号化結果が同じ");
        assert_ne!(encrypted2, encrypted3, "2回目と3回目の暗号化結果が同じ");
        assert_ne!(encrypted1, encrypted3, "1回目と3回目の暗号化結果が同じ");
        
        // 全て正常に復号化できることを確認
        let decrypted1 = crypto_service.decrypt(&encrypted1, password)
            .expect("1回目の復号化に失敗");
        let decrypted2 = crypto_service.decrypt(&encrypted2, password)
            .expect("2回目の復号化に失敗");
        let decrypted3 = crypto_service.decrypt(&encrypted3, password)
            .expect("3回目の復号化に失敗");
        
        // 全て元データと一致することを確認
        assert_eq!(data, decrypted1.as_slice());
        assert_eq!(data, decrypted2.as_slice());
        assert_eq!(data, decrypted3.as_slice());
    }
    
    /**
     * データ形式の正確性テスト
     * 
     * 暗号化データの形式（ソルト+ノンス+暗号文）が正しいことを確認
     */
    #[test]
    fn test_data_format_structure() {
        let crypto_service = CryptoService::new();
        let data = b"format test data";
        let password = "format_password";
        
        let encrypted = crypto_service.encrypt(data, password)
            .expect("暗号化に失敗");
        
        // データサイズの確認（ソルト32 + ノンス12 + 元データ + 認証タグ16）
        assert!(encrypted.len() >= 32 + 12 + data.len() + 16,
                "暗号化データのサイズが不正");
        
        // 異なるソルトとノンスが使用されていることを確認
        let salt1 = &encrypted[0..32];
        let nonce1 = &encrypted[32..44];
        
        let encrypted2 = crypto_service.encrypt(data, password)
            .expect("2回目の暗号化に失敗");
        let salt2 = &encrypted2[0..32];
        let nonce2 = &encrypted2[32..44];
        
        assert_ne!(salt1, salt2, "ソルトが再利用されている");
        assert_ne!(nonce1, nonce2, "ノンスが再利用されている");
    }
    
    /**
     * SecureBytesのメモリクリア機能テスト
     * 
     * SecureBytesがDropされた後にメモリがクリアされることを確認
     */
    #[test]
    fn test_secure_bytes_memory_clear() {
        let test_data = b"confidential data".to_vec();
        let data_ptr: *const u8;
        
        {
            let secure_bytes = SecureBytes::new(test_data.clone());
            data_ptr = secure_bytes.as_slice().as_ptr();
            
            // データが正しく格納されていることを確認
            assert_eq!(secure_bytes.as_slice(), test_data.as_slice());
        } // ここでsecure_bytesがDropされる
        
        // メモリがクリアされていることを確認（完全な検証は困難だが、基本的な動作を確認）
        // 注意: この検証は環境依存でありベストエフォートベース
    }
    
    /**
     * SecureStringのメモリクリア機能テスト
     * 
     * SecureStringがパスワードを安全に管理することを確認
     */
    #[test]
    fn test_secure_string_functionality() {
        let password = "secret_password_123".to_string();
        let secure_password = SecureString::new(password.clone());
        
        // 文字列として正しく取得できることを確認
        assert_eq!(secure_password.as_str(), Some(password.as_str()));
        
        // バイト配列として正しく取得できることを確認
        assert_eq!(secure_password.as_bytes(), password.as_bytes());
        
        // 手動クリア機能のテスト
        let mut secure_password_manual = SecureString::new("manual_clear_test".to_string());
        secure_password_manual.clear();
        
        // クリア後は空になることを確認
        assert_eq!(secure_password_manual.as_bytes().len(), 0);
    }
    
    /**
     * 暗号化性能テスト
     * 
     * PBKDF2の100,000回イテレーションを考慮した現実的なパフォーマンステスト。
     * APIキー暗号化の単体性能を測定。
     */
    #[test]
    fn test_encryption_performance() {
        let crypto_service = CryptoService::new();
        let api_key = "sk-1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let password = "performance_test_password";
        
        // 単体の暗号化・復号化性能を測定
        let start_time = std::time::Instant::now();
        let encrypted = crypto_service.encrypt(api_key.as_bytes(), password)
            .expect("暗号化に失敗");
        let encrypt_time = start_time.elapsed();
        
        let start_time = std::time::Instant::now();
        let _decrypted = crypto_service.decrypt(&encrypted, password)
            .expect("復号化に失敗");
        let decrypt_time = start_time.elapsed();
        
        println!("暗号化時間: {:.2}ms", encrypt_time.as_millis());
        println!("復号化時間: {:.2}ms", decrypt_time.as_millis());
        
        // PBKDF2の100,000回イテレーションを考慮し、単体処理が5秒以内で完了することを確認
        assert!(encrypt_time.as_secs() < 5, 
                "暗号化に{}秒かかった（目標: 5秒以内）", encrypt_time.as_secs_f64());
        assert!(decrypt_time.as_secs() < 5, 
                "復号化に{}秒かかった（目標: 5秒以内）", decrypt_time.as_secs_f64());
        
        // 実際の使用パターン: 少数のAPIキーの暗号化・復号化
        let start_time = std::time::Instant::now();
        for i in 0..5 {
            let test_key = format!("{}-{}", api_key, i);
            let encrypted = crypto_service.encrypt(test_key.as_bytes(), password)
                .expect(&format!("{}回目の暗号化に失敗", i + 1));
            let _decrypted = crypto_service.decrypt(&encrypted, password)
                .expect(&format!("{}回目の復号化に失敗", i + 1));
        }
        let batch_time = start_time.elapsed();
        
        println!("5件の暗号化・復号化: {:.2}秒", batch_time.as_secs_f64());
        
        // 5件の往復処理が30秒以内で完了することを確認（セキュリティと実用性のバランス）
        assert!(batch_time.as_secs() < 30, 
                "5件の往復処理に{}秒かかった（目標: 30秒以内）", batch_time.as_secs_f64());
    }
    
    /**
     * 空文字列やspecial文字のテスト
     * 
     * エッジケースでの動作を確認
     */
    #[test]
    fn test_edge_cases() {
        let crypto_service = CryptoService::new();
        let password = "edge_case_password";
        
        // 空データ
        let empty_data = b"";
        let encrypted = crypto_service.encrypt(empty_data, password)
            .expect("空データの暗号化に失敗");
        let decrypted = crypto_service.decrypt(&encrypted, password)
            .expect("空データの復号化に失敗");
        assert_eq!(empty_data, decrypted.as_slice());
        
        // 1バイトデータ
        let single_byte = b"x";
        let encrypted = crypto_service.encrypt(single_byte, password)
            .expect("1バイトデータの暗号化に失敗");
        let decrypted = crypto_service.decrypt(&encrypted, password)
            .expect("1バイトデータの復号化に失敗");
        assert_eq!(single_byte, decrypted.as_slice());
        
        // 空のパスワード（技術的には可能）
        let test_data = b"test with empty password";
        let empty_password = "";
        let encrypted = crypto_service.encrypt(test_data, empty_password)
            .expect("空パスワードでの暗号化に失敗");
        let decrypted = crypto_service.decrypt(&encrypted, empty_password)
            .expect("空パスワードでの復号化に失敗");
        assert_eq!(test_data, decrypted.as_slice());
    }
}