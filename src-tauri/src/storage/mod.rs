// ストレージモジュール
// ローカルデータ管理

pub mod service;
pub mod repository;
pub mod schema;
pub mod secure_repository;

#[cfg(test)]
mod schema_test;


pub use service::StorageService;
pub use repository::{TicketRepository, ConfigRepository, Repository, DatabaseError};
pub use secure_repository::{SecureRepository, SecureRepositoryError};