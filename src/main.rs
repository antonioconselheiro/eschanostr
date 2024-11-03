use crossterm::cursor::{RestorePosition, SavePosition};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
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
  cursor::MoveTo,
  style::Print,
};
use std::io::{stdout, Write};

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

struct AnimationThread {
  running: Arc<AtomicBool>,
  handle: Option<thread::JoinHandle<()>>,
}

impl AnimationThread {
  fn new() -> Self {
    AnimationThread {
          running: Arc::new(AtomicBool::new(false)),
          handle: None,
      }
  }

  fn start(&mut self, row: u16, col: u16) {
    self.running.store(true, Ordering::Relaxed);
    let running = self.running.clone();
    
    self.handle = Some(thread::spawn(move || {
      let mut stdout = stdout();
      enable_raw_mode().unwrap();

      while running.load(Ordering::Relaxed) {
        for frame in &FRAMES {
          if !running.load(Ordering::Relaxed) {
              break;
          }

          queue!(stdout, SavePosition).unwrap();
          
          // render animation frame
          for (i, line) in frame.lines().enumerate() {
            queue!(
              stdout,
              MoveTo(col, row + i as u16),
              Print(line),
            ).unwrap();
          }

          // Restore cursor position and flush
          queue!(stdout, RestorePosition).unwrap();
          stdout.flush().unwrap();

          thread::sleep(Duration::from_millis(200));
        }
      }

      disable_raw_mode().unwrap();
    }));
  }

  fn stop(&mut self) {
    self.running.store(false, Ordering::Relaxed);
    if let Some(handle) = self.handle.take() {
      handle.join().unwrap();
    }
  }
}

impl Drop for AnimationThread {
  fn drop(&mut self) {
    self.stop();
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("               _   _     n _____ n   ____        ____    _   _       _         _         n  ___ n   ____       _____      ____      ");
    println!("              | \\ |\"|    \\| ___\"|/  / __\"| n  n /\"___|  |'| |'|  n  /\"\\  n    |\"|         \\/\"_ \\/  / __\"| n   |_ \" _|  n |  _\"\\ n   ");
    println!("             <|  \\| |>    |  _|\"   <\\___ \\/   \\| | n   /| |_| |\\  \\/ _ \\/   n | | n       | | | | <\\___ \\/      | |     \\| |_) |/   ");
    println!("             n| |\\  |n    | |___    n___) |    | |/__  n|  _  |n  / ___ \\    \\| |/__  .-,_| |_| |  n___) |     /| |\\     |  _ <     ");
    println!("              |_| \\_|     |_____|   |____/>>    \\____|  |_| |_|  /_/   \\_\\    |_____|  \\_)-\\___/   |____/>>   n |_|n     |_| \\_\\    ");
    println!("              ||   \\\\,-.  <<   >>    )(  (__)  _// \\\\   //   \\\\   \\\\    >>    //  \\\\        \\\\      )(  (__)  _// \\\\_    //   \\\\_   ");
    println!("              (_\")  (_/  (__) (__)  (__)      (__)(__) (_\") (\"_) (__)  (__)  (_\")(\"_)      (__)    (__)      (__) (__)  (__)  (__) \n\n");

  // Parse CLI arguments first
  let args = Cli::parse();
  let password = args.npassword;
  let full_regex_pattern = format!(r"^npub1({})", args.nregex);
  
  println!("             :: STARTING\r");
  let dance_logs = [
    "it's reggae music time",
    "DJ, set the beat now!",
    "Time to shake it off!",
    "Let's dance all night",
    "Dance party, let's go!",
    "Hands up maestro!",
    "In the flow of the rhythm"
  ];

  let lets_dance = dance_logs.choose(&mut thread_rng()).unwrap();
  println!("             :: {}\r", lets_dance);
  println!("             :: Entropy algorithm in neschalostr are high affected in a positive way by people dancing\r");

  // Compile regex
  let re = Regex::new(&full_regex_pattern)
    .map_err(|e| {
        eprintln!("             :: Error compiling regex: {}\r", e);
        e
    })?;

  println!("             :: REGEX /{}/\r", re.to_string());

  let mut animation = AnimationThread::new();
  animation.start(16, 0);
  thread::sleep(Duration::from_millis(500));

  // really start here
  loop {
    // keys
    let secret_key = Keys::generate();
    
    // npub
    let bech32_pubkey = secret_key.public_key().to_bech32()?;

    // check if matches
    if re.is_match(&bech32_pubkey) {
      println!("            --------------------");
      println!("            [{}] npub: {}\r", Local::now().to_rfc3339(), bech32_pubkey);

      // ncryptsec
      let ncryptsec = EncryptedSecretKey::new(&secret_key.secret_key(), password.clone(), 16, KeySecurity::Medium).unwrap();
      println!("            [{}] ncryptsec: {}\r", Local::now().to_rfc3339(), ncryptsec.to_bech32()?);
    }
  }
}