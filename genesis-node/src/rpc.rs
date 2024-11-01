use jsonrpc_core::{IoHandler, Params, Value, Error};
use jsonrpc_http_server::ServerBuilder;
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Semaphore;
use std::time::Duration;

const VALID_API_KEY: &str = "your_valid_api_key_here"; // Replace with your actual API key
const MAX_CONCURRENT_REQUESTS: usize = 10; // Maximum allowed concurrent requests

/// Custom error type for RPC methods
enum RpcError {
    Unauthorized,
    NotFound,
    InvalidParams,
    InternalError,
}

/// Convert `RpcError` to `jsonrpc_core::Error`
impl From<RpcError> for Error {
    fn from(error: RpcError) -> Self {
        match error {
            RpcError::Unauthorized => Error::invalid_params("Unauthorized access"),
            RpcError::NotFound => Error::invalid_params("Resource not found"),
            RpcError::InvalidParams => Error::invalid_params("Invalid parameters"),
            RpcError::InternalError => Error::internal_error(),
        }
    }
}

/// Initialize the JSON-RPC server with governance methods and concurrency rate limiting
pub async fn initialize_rpc_server() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing RPC server...");

    let mut io = IoHandler::default();
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS)); // Semaphore for concurrent rate limiting

    // Define the `submit_proposal` method with rate limiting
    let semaphore_clone = semaphore.clone();
    io.add_method("submit_proposal", move |params: Params| {
        let semaphore = semaphore_clone.clone();
        async move {
            let _permit = semaphore.acquire().await.map_err(|_| RpcError::InternalError)?;
            info!("submit_proposal method called with params: {:?}", params);

            // API key validation
            let api_key: String = params.clone().parse().map_err(|_| RpcError::InvalidParams)?;
            if api_key != VALID_API_KEY {
                warn!("Unauthorized access attempt with API key: {}", api_key);
                return Err(RpcError::Unauthorized.into());
            }

            // Parse the proposal description
            let description: String = params.parse().map_err(|_| RpcError::InvalidParams)?;
            info!("Parsed description: {}", description);

            // Example proposal ID generation
            let proposal_id = 1;
            info!("Generated proposal ID: {}", proposal_id);

            Ok(Value::String(format!("Proposal {} submitted: '{}'", proposal_id, description)))
        }
    });

    // Define the `vote_on_proposal` method with rate limiting
    let semaphore_clone = semaphore.clone();
    io.add_method("vote_on_proposal", move |params: Params| {
        let semaphore = semaphore_clone.clone();
        async move {
            let _permit = semaphore.acquire().await.map_err(|_| RpcError::InternalError)?;
            info!("vote_on_proposal method called with params: {:?}", params);

            // API key validation
            let api_key: String = params.clone().parse().map_err(|_| RpcError::InvalidParams)?;
            if api_key != VALID_API_KEY {
                warn!("Unauthorized access attempt with API key: {}", api_key);
                return Err(RpcError::Unauthorized.into());
            }

            // Parse proposal ID and account ID
            let (proposal_id, account_id): (u32, String) = params.parse().map_err(|_| RpcError::InvalidParams)?;
            info!("Parsed proposal ID: {}, account ID: {}", proposal_id, account_id);

            Ok(Value::String(format!("Vote added for proposal {}", proposal_id)))
        }
    });

    // Define the `review_proposal` method with rate limiting
    let semaphore_clone = semaphore.clone();
    io.add_method("review_proposal", move |params: Params| {
        let semaphore = semaphore_clone.clone();
        async move {
            let _permit = semaphore.acquire().await.map_err(|_| RpcError::InternalError)?;
            info!("review_proposal method called with params: {:?}", params);

            // API key validation
            let api_key: String = params.clone().parse().map_err(|_| RpcError::InvalidParams)?;
            if api_key != VALID_API_KEY {
                warn!("Unauthorized access attempt with API key: {}", api_key);
                return Err(RpcError::Unauthorized.into());
            }

            // Parse proposal ID and approval status
            let (proposal_id, approve): (u32, bool) = params.parse().map_err(|_| RpcError::InvalidParams)?;
            info!("Parsed proposal ID: {}, approval status: {}", proposal_id, approve);

            Ok(Value::String(format!("Proposal {} reviewed. Approved: {}", proposal_id, approve)))
        }
    });

    // Start the JSON-RPC server without additional rate-limiting layer
    let server = ServerBuilder::new(io)
        .start_http(&"127.0.0.1:9933".parse().expect("Invalid address"))
        .expect("Failed to start JSON-RPC HTTP server");

    info!("RPC server running on port 9933 with concurrency limiting enabled...");
    Ok(server.wait())
}

