use solana_sdk::signature::{Keypair, Signer};

/// Generates the specified number of Solana keypairs.
pub fn generate_keypairs(count: usize) -> Vec<Keypair> {
    (0..count).map(|_| Keypair::new()).collect()
}

/// Displays the public and secret keys for each keypair.
pub fn display_keypairs(keypairs: &[Keypair]) {
    for (index, keypair) in keypairs.iter().enumerate() {
        println!("Keypair {}: Public Key = {}", index + 1, keypair.pubkey());
        println!(
            "Keypair {}: Secret Key = {:?}",
            index + 1,
            keypair.to_bytes()
        );
    }
}
