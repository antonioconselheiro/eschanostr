use nostr::prelude::*;
use clap::Parser;
use regex::Regex;
use chrono::Local;
use nostr::secp256k1::rand::prelude::SliceRandom;
use nostr::secp256k1::rand::thread_rng;

#[derive(Parser)]
struct Cli {

    #[arg(short = 'r', long)]
    nregex: String,

    #[arg(short = 'p', long)]
    npassword: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("   _   _     n _____ n   ____        ____    _   _       _         _         n  ___ n   ____       _____      ____      ");
    println!("  | \\ |\"|    \\| ___\"|/  / __\"| n  n /\"___|  |'| |'|  n  /\"\\  n    |\"|         \\/\"_ \\/  / __\"| n   |_ \" _|  n |  _\"\\ n   ");
    println!(" <|  \\| |>    |  _|\"   <\\___ \\/   \\| | n   /| |_| |\\  \\/ _ \\/   n | | n       | | | | <\\___ \\/      | |     \\| |_) |/   ");
    println!(" n| |\\  |n    | |___    n___) |    | |/__  n|  _  |n  / ___ \\    \\| |/__  .-,_| |_| |  n___) |     /| |\\     |  _ <     ");
    println!("  |_| \\_|     |_____|   |____/>>    \\____|  |_| |_|  /_/   \\_\\    |_____|  \\_)-\\___/   |____/>>   n |_|n     |_| \\_\\    ");
    println!("  ||   \\\\,-.  <<   >>    )(  (__)  _// \\\\   //   \\\\   \\\\    >>    //  \\\\        \\\\      )(  (__)  _// \\\\_    //   \\\\_   ");
    println!("  (_\")  (_/  (__) (__)  (__)      (__)(__) (_\") (\"_) (__)  (__)  (_\")(\"_)      (__)    (__)      (__) (__)  (__)  (__) \n\n");

    println!(" :: NESCHALOSTR STARTING");
    let dance_logs = [
        "it's reggae music time",
        "DJ, set the beat now!",
        "Time to shake it off!",
        "Let's dance all night",
        "Dance party, let’s go!",
        "Hands up maestro!",
        "É hora do arrasta-pé"
    ];

    let lets_dance = dance_logs.choose(&mut thread_rng()).unwrap();
    println!(" :: {}", lets_dance);

    println!(" :: entropy algorithm in neschalostr are high affected in a positive way by people dancing");
    let args = Cli::parse();
    let password = args.npassword;
    let full_regex_pattern = format!(r"^npub1({})", args.nregex);
    let re = Regex::new(&full_regex_pattern)
        .map_err(|e| {
            eprintln!("Erro compiling regex: {}", e);
            e
        })?;

    println!("Regex: /{}/", re.to_string());

    loop {
        // keys
        let secret_key = Keys::generate();
        
        // npub
        let bech32_pubkey = secret_key.public_key().to_bech32()?;

        // check if matches
        if re.is_match(&bech32_pubkey) {
            println!("--------------------");
            println!("[{}]", Local::now().to_rfc3339());
            println!("npub: {}", bech32_pubkey);

            // ncryptsec
            let ncryptsec = EncryptedSecretKey::new(&secret_key.secret_key(), password.clone(), 16, KeySecurity::Medium).unwrap();
            println!("ncryptsec: {}", ncryptsec.to_bech32()?);
        }
    }
}
