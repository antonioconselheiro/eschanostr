use nostr::prelude::*;
use clap::Parser;
use regex::Regex;
use chrono::Local;

#[derive(Parser)]
struct Cli {

    #[arg(short, long)]
    nregex: String,

    #[arg(short, long)]
    npassword: String,
}

fn main() {
    let args = Cli::parse();

    let full_regex_pattern = format!("^npub1({})$", args.nregex);
    println!("Regex: /{}/", full_regex_pattern);
    let re = Regex::new(&full_regex_pattern)
        .map_err(|e| {
            eprintln!("Erro ao compilar a expressão regular: {}", e);
            e
        })?;

    let mut my_keys: Keys;
    let mut bech32_pubkey: String;
    let ncryptsec: String;

    loop {
        // keys
        let secret_key = Keys::generate();
        
        // npub
        bech32_pubkey = my_keys.public_key().to_bech32()?;

        // check if matches
        if re.is_match(&bech32_pubkey) {

            // ncryptsec
            let ncryptsec = EncryptedSecretKey::new(&secret_key.secret_key(), args.npassword, 16, KeySecurity::Medium).unwrap();

            println!("--------------------");
            println!("Timestamp: {}", Local::now().to_rfc3339());
            println!("Npub: {}", bech32_pubkey);
            println!("Ncryptsec: {}", ncryptsec.to_bech32())?;
        }
    }
}
