use nostr::prelude::*;
use clap::Parser;
use regex::Regex;
use chrono::Local;
use nostr::secp256k1::rand::prelude::SliceRandom;
use nostr::secp256k1::rand::thread_rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crossterm::{
    cursor::{MoveTo, Hide, Show},
    terminal::{Clear, ClearType},
    execute,
    style::Print,
};
use std::io::stdout;

#[derive(Parser)]
struct Cli {

    #[arg(short = 'r', long)]
    nregex: String,

    #[arg(short = 'p', long)]
    npassword: String,

    // #[arg(short = 'd', long)]
    // ndancing: Boolean,
}

const FRAME1: &str = "    N     \n   /|\\     \n   / \\     ";
const FRAME2: &str = "   \\ N/    \n    |      \n   |  |     ";
const FRAME3: &str = "      N    \n     /|\\    \n    /  \\    ";
const FRAME4: &str = "    \\N/     \n     |      \n   |  |     ";
const FRAME5: &str = "    \\N/     \n    |       \n   |  |     ";

const FRAMES: [&str; 13] = [
  FRAME1,
  FRAME2,
  FRAME3,
  FRAME2,
  FRAME1,
  FRAME2,
  FRAME3,
  FRAME4,
  FRAME5,
  FRAME4,
  FRAME5,
  FRAME4,
  FRAME5
];

fn animate_character(running: Arc<AtomicBool>, row: u16, col: u16) {
    let mut stdout = stdout();
    
    // Hide cursor during animation
    execute!(stdout, Hide).unwrap();

    while running.load(Ordering::Relaxed) {
        for frame in &FRAMES {
            if !running.load(Ordering::Relaxed) {
                break;
            }

            // Clear previous frame
            execute!(
                stdout,
                MoveTo(col, row),
                Clear(ClearType::FromCursorDown)
            ).unwrap();

            // Draw new frame
            for (i, line) in frame.lines().enumerate() {
                execute!(
                    stdout,
                    MoveTo(col, row + i as u16),
                    Print(line)
                ).unwrap();
            }

            thread::sleep(Duration::from_millis(200));
        }
    }

    execute!(stdout, Show).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("   _   _     n _____ n   ____        ____    _   _       _         _         n  ___ n   ____       _____      ____      ");
    println!("  | \\ |\"|    \\| ___\"|/  / __\"| n  n /\"___|  |'| |'|  n  /\"\\  n    |\"|         \\/\"_ \\/  / __\"| n   |_ \" _|  n |  _\"\\ n   ");
    println!(" <|  \\| |>    |  _|\"   <\\___ \\/   \\| | n   /| |_| |\\  \\/ _ \\/   n | | n       | | | | <\\___ \\/      | |     \\| |_) |/   ");
    println!(" n| |\\  |n    | |___    n___) |    | |/__  n|  _  |n  / ___ \\    \\| |/__  .-,_| |_| |  n___) |     /| |\\     |  _ <     ");
    println!("  |_| \\_|     |_____|   |____/>>    \\____|  |_| |_|  /_/   \\_\\    |_____|  \\_)-\\___/   |____/>>   n |_|n     |_| \\_\\    ");
    println!("  ||   \\\\,-.  <<   >>    )(  (__)  _// \\\\   //   \\\\   \\\\    >>    //  \\\\        \\\\      )(  (__)  _// \\\\_    //   \\\\_   ");
    println!("  (_\")  (_/  (__) (__)  (__)      (__)(__) (_\") (\"_) (__)  (__)  (_\")(\"_)      (__)    (__)      (__) (__)  (__)  (__) \n\n");

  // Parse CLI arguments first
  let args = Cli::parse();
  let password = args.npassword;
  let full_regex_pattern = format!(r"^npub1({})", args.nregex);
  
  println!(" :: STARTING");
  let dance_logs = [
      "it's reggae music time",
      "DJ, set the beat now!",
      "Time to shake it off!",
      "Let's dance all night",
      "Dance party, let's go!",
      "Hands up maestro!",
      "É hora do arrasta-pé",
      "In the flow of the rhythm"
  ];

  let lets_dance = dance_logs.choose(&mut thread_rng()).unwrap();
  println!(" :: {}", lets_dance);
  println!(" :: Entropy algorithm in neschalostr are high affected in a positive way by people dancing");

  // Compile regex
  let re = Regex::new(&full_regex_pattern)
      .map_err(|e| {
          eprintln!("Error compiling regex: {}", e);
          e
      })?;

  println!(" :: REGEX /{}/", re.to_string());

  let running = Arc::new(AtomicBool::new(true));
  let animation_running = running.clone();

  let animation_handle = thread::spawn(move || {
      animate_character(animation_running, 16, 0);
  });

  // Give the animation a moment to start
  thread::sleep(Duration::from_millis(500));

  // really start here
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