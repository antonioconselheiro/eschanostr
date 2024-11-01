use nostr::prelude::*;
use clap::Parser;
use regex::Regex;
use chrono::Local;

#[derive(Parser)]
struct Cli {

    #[arg(short = 'r', long)]
    nregex: String,

    #[arg(short = 'p', long)]
    npassword: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let password = args.npassword;
    let full_regex_pattern = format!("^npub1({})$", args.nregex);
    println!("Regex: /{}/", full_regex_pattern);
    let re = Regex::new(&full_regex_pattern)
        .map_err(|e| {
            eprintln!("Erro ao compilar a expressão regular: {}", e);
            e
        })?;

    loop {
        // keys
        let secret_key = Keys::generate();
        
        // npub
        let bech32_pubkey = secret_key.public_key().to_bech32()?;

        // check if matches
        if re.is_match(&bech32_pubkey) {

            // ncryptsec
            let ncryptsec = EncryptedSecretKey::new(&secret_key.secret_key(), password.clone(), 16, KeySecurity::Medium).unwrap();

            println!("--------------------");
            println!("Timestamp: {}", Local::now().to_rfc3339());
            println!("Npub: {}", bech32_pubkey);
            println!("Ncryptsec: {}", ncryptsec.to_bech32()?);
        }
    }
}
