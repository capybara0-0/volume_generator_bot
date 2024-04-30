use solana_sdk::signature::{Keypair, Signer};
use std::io;

const PROMPT_MESSAGE: &str = "Enter the number of keypairs to generate: ";
const READ_ERROR_MESSAGE: &str = "Failed to read line";
const PARSE_ERROR_MESSAGE: &str = "Please type a number!";

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// The entry point for the program logic.
fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", PROMPT_MESSAGE);

    let num_keypairs = read_usize_from_user()?;
    let keypairs = generate_keypairs(num_keypairs);

    display_keypairs(&keypairs);

    Ok(())
}

/// Reads a line from stdin and attempts to parse it into a usize.
fn read_usize_from_user() -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| READ_ERROR_MESSAGE)?;
    input
        .trim()
        .parse::<usize>()
        .map_err(|_| PARSE_ERROR_MESSAGE.into())
}

/// Generates the specified number of Solana keypairs.
fn generate_keypairs(count: usize) -> Vec<Keypair> {
    (0..count).map(|_| Keypair::new()).collect()
}

/// Displays the public and secret keys for each keypair.
fn display_keypairs(keypairs: &[Keypair]) {
    for (index, keypair) in keypairs.iter().enumerate() {
        println!("Keypair {}: Public Key = {}", index + 1, keypair.pubkey());
        println!(
            "Keypair {}: Secret Key = {:?}",
            index + 1,
            keypair.to_bytes()
        );
    }
}
