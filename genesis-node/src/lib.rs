// File: genesis-node/src/lib.rs

// genesis-node/src/lib.rs
pub mod genesis;
pub mod rpc;
pub mod websocket;
pub mod utils;

// Re-export functions if needed
pub use genesis::initialize_genesis_block;
pub use rpc::initialize_rpc_server;
pub use websocket::start_websocket_server;

// Common Constants
pub const RPC_SERVER_ADDRESS: &str = "127.0.0.1:9933";
pub const WEBSOCKET_SERVER_ADDRESS: &str = "127.0.0.1:9944";
pub const API_VERSION: &str = "1.0.0";

// Error Handling
use jsonrpc_core::Error as JsonRpcError;
use log::{info, error};

#[derive(Debug)]
pub enum GenesisNodeError {
    RpcInitializationFailed(JsonRpcError),
    WebSocketInitializationFailed(std::io::Error),
    GenesisBlockInitializationFailed(String),
}

impl From<JsonRpcError> for GenesisNodeError {
    fn from(err: JsonRpcError) -> Self {
        GenesisNodeError::RpcInitializationFailed(err)
    }
}

impl From<std::io::Error> for GenesisNodeError {
    fn from(err: std::io::Error) -> Self {
        GenesisNodeError::WebSocketInitializationFailed(err)
    }
}

// Logging Utilities
pub fn init_logging() {
    env_logger::init();
    info!("Logging initialized successfully.");
}

pub fn log_error_and_exit(message: &str) {
    error!("{}", message);
    std::process::exit(1);
}

// Common Types
pub type Result<T> = std::result::Result<T, GenesisNodeError>;

#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
    pub balance: u64,
}

