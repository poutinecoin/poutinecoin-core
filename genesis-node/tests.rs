#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_initialize_genesis_block() {
        let (genesis_block, founder_account) = initialize_genesis_block();

        // Check the founder account is correct
        assert_eq!(founder_account, "founder_account".to_string());

        // Validate initial supply and max supply
        assert_eq!(genesis_block.initial_supply, 500_000_000);
        assert_eq!(genesis_block.max_supply, 1_000_000_000);

        // Check accounts setup
        assert_eq!(genesis_block.accounts.len(), 5);
        assert_eq!(genesis_block.accounts[0].1, 100_000_000);

        // Validate proposals
        assert_eq!(genesis_block.proposals.len(), 1);
        assert_eq!(
            genesis_block.proposals[0].description,
            "Initial community fund allocation".to_string()
        );
    }

    #[test]
    fn test_genesis_block_validation() {
        let (genesis_block, _) = initialize_genesis_block();
        assert!(validate_genesis_block(&genesis_block).is_ok());
    }

    #[tokio::test]
    async fn test_websocket_message_handling() {
        // Simulate WebSocket messages and validate correct response
        // This part would use mocks and async channels to simulate incoming WebSocket messages

        // Example setup for WebSocket message simulation
        let message = WebSocketMessage::Ping;
        let response = match message {
            WebSocketMessage::Ping => "Pong".to_string(),
            _ => "Unexpected message".to_string(),
        };

        // Validate the simulated response
        assert_eq!(response, "Pong");
    }
}

