use blake2::{Blake2bVar, digest::{Update, VariableOutput}};
use ed25519_dalek::Keypair;
use log::info;
use ed25519_dalek::Signer;
use hex;

/// Generate a block hash from previous block and transactions
pub fn generate_block(previous_block: &str, transactions: Vec<&str>) -> String {
    info!("Generating block based on previous block and transactions.");
    let mut hasher = Blake2bVar::new(32).expect("Failed to create Blake2b hasher");
    hasher.update(previous_block.as_bytes());

    for tx in &transactions {
        info!("Including transaction in block: {}", tx);
        hasher.update(tx.as_bytes());
    }

    // Finalize the hash with a mutable reference
    let mut block_hash = vec![0u8; 32];
    hasher.finalize_variable(&mut block_hash).expect("Failed to finalize hash");

    let block_hash_hex = hex::encode(block_hash);
    info!("Generated block hash: {}", block_hash_hex);
    block_hash_hex
}

/// Sign a transaction using a keypair
pub fn sign_transaction(transaction: &str, keypair: &Keypair) -> Vec<u8> {
    info!("Signing transaction: {}", transaction);
    let signature = keypair.sign(transaction.as_bytes());
    let signature_bytes = signature.to_bytes().to_vec();
    info!("Transaction signed with signature: {:?}", signature_bytes);
    signature_bytes
}

