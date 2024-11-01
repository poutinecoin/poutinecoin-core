use log::{info, error};
use std::fmt;

/// Define the Genesis Block structure
#[derive(Debug)]
pub struct GenesisBlock {
    pub accounts: Vec<(String, u64)>,      // Account names with initial balances
    pub initial_supply: u64,               // Starting supply
    pub block_number: u64,                 // Initial block number
    pub max_supply: u64,                   // Maximum token supply
    pub inflation_rate: f64,               // Annual inflation rate
    pub proposals: Vec<Proposal>,          // List of initial proposals
}

/// Define the Proposal structure
#[derive(Debug)]
pub struct Proposal {
    pub id: u32,
    pub description: String,
    pub vote_count: u64,
    pub approved: bool,
}

/// Define a custom error type for Genesis Block initialization
#[derive(Debug)]
pub struct GenesisError;

impl fmt::Display for GenesisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Genesis Block Initialization Error")
    }
}

impl std::error::Error for GenesisError {}

/// Initialize the genesis block with default values and proposals
pub fn initialize_genesis_block() -> Result<(GenesisBlock, String), GenesisError> {
    info!("Initializing genesis block...");

    // Define initial accounts and balances
    let initial_accounts = vec![
        ("founder_account".to_string(), 100_000_000),
        ("developer_account".to_string(), 50_000_000),
        ("community_fund".to_string(), 200_000_000),
        ("validator_account1".to_string(), 10_000_000),
        ("validator_account2".to_string(), 10_000_000),
    ];
    info!("Accounts initialized with balances: {:?}", initial_accounts);

    // Define initial proposals
    let proposals = vec![Proposal {
        id: 1,
        description: "Initial community fund allocation".to_string(),
        vote_count: 0,
        approved: false,
    }];
    info!("Proposals initialized: {:?}", proposals);

    let founder_account = "founder_account".to_string();

    // Create the genesis block with the predefined values
    let genesis_block = GenesisBlock {
        accounts: initial_accounts,
        initial_supply: 500_000_000,
        block_number: 0,
        max_supply: 1_000_000_000,
        inflation_rate: 0.02,
        proposals,
    };
    info!("Genesis block created with initial values: {:?}", genesis_block);

    // Validate the genesis block to ensure consistency
    match validate_genesis_block(&genesis_block) {
        Ok(_) => info!("Genesis block validation successful."),
        Err(e) => {
            error!("Genesis block validation failed: {}", e);
            return Err(GenesisError);
        }
    }

    Ok((genesis_block, founder_account))
}

/// Validates the genesis block to ensure consistency and integrity
fn validate_genesis_block(genesis_block: &GenesisBlock) -> Result<(), String> {
    info!("Validating genesis block...");

    // Calculate the total supply based on account balances
    let total_supply: u64 = genesis_block.accounts.iter().map(|(_, balance)| balance).sum();
    if total_supply > genesis_block.initial_supply {
        return Err("Total account balances exceed initial supply".into());
    }
    info!("Total account balances verified against initial supply.");

    // Check inflation rate validity
    if genesis_block.inflation_rate < 0.0 || genesis_block.inflation_rate > 1.0 {
        return Err("Invalid inflation rate".into());
    }
    info!("Inflation rate is within acceptable range.");

    info!("Genesis block validation completed successfully.");
    Ok(())
}

