use log::{info, warn, error};
use tokio::join;
use genesis_node::{
    initialize_genesis_block, initialize_rpc_server, start_websocket_server, 
    log_error_and_exit, init_logging
};

#[tokio::main]
async fn main() {
    // Initialize the logger
    init_logging();
    info!("Starting POUT Genesis Node...");

    // Initialize the genesis block and log the details
    match initialize_genesis_block() {
        Ok((genesis_block, founder_account)) => {
            info!("Genesis Block Initialized: {:?}", genesis_block);
            info!("Founder Account: {:?}", founder_account);
        }
        Err(e) => log_error_and_exit(&format!("Failed to initialize genesis block: {:?}", e)),
    }

    // Initialize the RPC and WebSocket servers with error handling
    let rpc_server = initialize_rpc_server();
    let websocket_server = start_websocket_server();

    // Await both servers and log results
    match join!(rpc_server, websocket_server) {
        (Ok(_), Ok(_)) => info!("Both RPC and WebSocket servers are running successfully."),
        (Err(rpc_err), Ok(_)) => {
            error!("RPC server failed to start: {:?}", rpc_err);
            warn!("WebSocket server is still running.");
            std::process::exit(1);
        }
        (Ok(_), Err(ws_err)) => {
            error!("WebSocket server failed to start: {:?}", ws_err);
            warn!("RPC server is still running.");
            std::process::exit(1);
        }
        (Err(rpc_err), Err(ws_err)) => {
            error!(
                "Both RPC and WebSocket servers failed to start. RPC Error: {:?}, WebSocket Error: {:?}",
                rpc_err, ws_err
            );
            std::process::exit(1);
        }
    }
}

