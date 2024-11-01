use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use log::{info, error, warn};
use futures_util::{StreamExt, SinkExt};

/// Enum to define different types of WebSocket messages
enum WebSocketMessage {
    Ping,
    Pong,
    ProposalUpdate { id: u32, description: String },
    BlockNotification { block_hash: String },
}

/// Function to parse incoming text messages into `WebSocketMessage` variants
fn parse_message_type(text: &str) -> WebSocketMessage {
    match text {
        "Ping" => WebSocketMessage::Ping,
        _ if text.starts_with("ProposalUpdate") => {
            // Example parsing for proposal update (assumes format "ProposalUpdate id description")
            let parts: Vec<&str> = text.split_whitespace().collect();
            if parts.len() == 3 {
                let id = parts[1].parse().unwrap_or(0);
                let description = parts[2].to_string();
                WebSocketMessage::ProposalUpdate { id, description }
            } else {
                warn!("Invalid ProposalUpdate format: {}", text);
                WebSocketMessage::Ping // Default to Ping if parsing fails
            }
        },
        _ if text.starts_with("BlockNotification") => {
            let parts: Vec<&str> = text.split_whitespace().collect();
            if parts.len() == 2 {
                let block_hash = parts[1].to_string();
                WebSocketMessage::BlockNotification { block_hash }
            } else {
                warn!("Invalid BlockNotification format: {}", text);
                WebSocketMessage::Ping
            }
        },
        _ => {
            warn!("Received unrecognized WebSocket message type: {}", text);
            WebSocketMessage::Ping
        }
    }
}

/// Start the WebSocket server
pub async fn start_websocket_server() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting WebSocket server...");

    let listener = TcpListener::bind("127.0.0.1:9944").await?;
    info!("WebSocket server running on ws://127.0.0.1:9944");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    let (mut write, mut read) = ws_stream.split();
                    info!("New WebSocket connection established.");

                    while let Some(Ok(message)) = read.next().await {
                        if let Message::Text(text) = message {
                            info!("Received WebSocket message: {}", text);

                            // Handle different WebSocket messages
                            match parse_message_type(&text) {
                                WebSocketMessage::Ping => {
                                    info!("Ping received, responding with Pong.");
                                    write.send(Message::Text("Pong".to_string())).await.unwrap();
                                },
                                WebSocketMessage::ProposalUpdate { id, description } => {
                                    info!("Proposal update received: ID = {}, Description = {}", id, description);
                                    // Here you would handle the proposal update, e.g., store or broadcast
                                },
                                WebSocketMessage::BlockNotification { block_hash } => {
                                    info!("Block notification received: Hash = {}", block_hash);
                                    // Handle block notification, e.g., update the client with new block details
                                },
                                _ => {
                                    warn!("Received an unrecognized message type.");
                                }
                            }

                            // Echo back any received message
                            write.send(Message::Text(format!("Echo: {}", text))).await.unwrap();
                        }
                    }
                },
                Err(e) => {
                    error!("WebSocket handshake failed: {:?}", e);
                }
            }
        });
    }
    Ok(())
}

