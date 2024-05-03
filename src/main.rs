use std::error::Error;

mod clients;
mod utils;

const PROMPT_MESSAGE: &str = "Enter the number of keypairs to generate: ";

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("{}", PROMPT_MESSAGE);

    let num_kepairs = utils::input::read_usize_from_user()?;
    let keypairs = clients::keypair::generate_keypairs(num_kepairs);

    clients::keypair::display_keypairs(&keypairs);

    Ok(())
}
