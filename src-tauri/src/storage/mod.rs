// ストレージモジュール
// ローカルデータ管理

pub mod service;
pub mod repository;
pub mod schema;

#[cfg(test)]
mod schema_test;

pub use service::StorageService;
pub use repository::{TicketRepository, ConfigRepository};