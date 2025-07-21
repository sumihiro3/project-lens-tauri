// MCP Server通信モジュール
// Backlog MCP Serverとの連携

pub mod service;
pub mod client;
pub mod protocol;

pub use service::MCPService;
pub use client::{MCPClient, ConnectionPool};
pub use protocol::{MCPRequest, MCPResponse, BacklogWorkspace};