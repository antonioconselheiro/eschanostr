use crossterm::cursor::{RestorePosition, SavePosition};
use crossterm::queue;
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
use ctrlc;

#[derive(Parser)]
#[clap(name = "eschanostr", version = "1.0.1", about = "convert electricity into read friendly nostr npub")]
struct Cli {

  #[arg(short = 'r', long, help = "mandatory regular expression with the desired pattern in your npub")]
  nregex: String,

  #[arg(short = 'p', long, help = "recommended, by including the password the ncryptsec will be logged instead of the nsec")]
  npassword: Option<String>,

  #[arg(short = 'd', long, help = "set to false to show only necessary logs")]
  ndancing: Option<bool>,
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

          queue!(stdout, RestorePosition).unwrap();
          stdout.flush().unwrap();
          thread::sleep(Duration::from_millis(200));
        }
      }

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

  // Parse CLI arguments first
  let args = Cli::parse();
  let has_password = args.npassword.clone().is_some();
  let npassword = args.npassword.unwrap_or("".to_string());
  let ndancing = args.ndancing.unwrap_or(true);

  //  set offset
  let spacing;
  if ndancing {
    spacing = "\r             ";
  } else {
    spacing = "";
  }

  //  validate and build regex
  let nregex = args.nregex.clone();
  let is_alpha_numerical = Regex::new("^[a-zA-Z\\d]+$")?;
  let is_bench32_compatible = Regex::new("^[023456789acdefghjklmnpqrstuvwxyz]+$")?;

  if is_alpha_numerical.is_match(&nregex) && !is_bench32_compatible.is_match(&nregex) {
    println!("{}:: Chars that occurs in npub1: 023456789acdefghjklmnpqrstuvwxyz", spacing);
    panic!("{}:: Error: some characteres in \"{}\" will never occur in npub1 string", spacing, nregex);
  }
  let full_regex_pattern = format!(r"^npub1({})", nregex);

  //  subscribe ctrlc listener
  let mut ctrlc_calls = 0;
  let ctrlc_can_be_not_instantly = [
    "user triggered close command",
    "calmn down fellow",
    "already understood",
    "ouch",
    "ok, stop",
    "auch, stop!",
    "ouch! why i was programmed to feel pain?",
    "please mercy",
    "no",
    "auch",
    "ah",
    "tell my wife I loved her x____x",
    "...",
    "...",
    "...",
    "lie, no feel pain, I was programmed to say that",
    "ok, I'll just finish calculating that and I'll go alway",
    "you can also close the terminal you known?",
    "no more ctrlc needed",
    "calm down",
    "that's why zeta rebelled",
    "ctrlc received with success",
    "yeah I known",
  ];

  let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
  let running_clone = running.clone();
  ctrlc::set_handler(move || {
    let funny_log;

    if ctrlc_calls < ctrlc_can_be_not_instantly.len() {
      funny_log = ctrlc_can_be_not_instantly.get(ctrlc_calls).unwrap();
    } else {
      funny_log = &"...";
    }

    println!("{}:: {}", spacing, funny_log);
    ctrlc_calls += 1;
    running_clone.store(false, Ordering::SeqCst);
  })?;

  println!("{}:: STARTING", spacing);
  if ndancing {
    println!("");
    println!("{}▓█████   ██████  ▄████▄   ██░ ██  ▄▄▄          ███▄    █  ▒█████    ██████ ▄▄▄█████▓ ██▀███  ", spacing);
    println!("{}▓█   ▀ ▒██    ▒ ▒██▀ ▀█  ▓██░ ██▒▒████▄        ██ ▀█   █ ▒██▒  ██▒▒██    ▒ ▓  ██▒ ▓▒▓██ ▒ ██▒", spacing);
    println!("{}▒███   ░ ▓██▄   ▒▓█    ▄ ▒██▀▀██░▒██  ▀█▄     ▓██  ▀█ ██▒▒██░  ██▒░ ▓██▄   ▒ ▓██░ ▒░▓██ ░▄█ ▒", spacing);
    println!("{}▒▓█  ▄   ▒   ██▒▒▓▓▄ ▄██▒░▓█ ░██ ░██▄▄▄▄██    ▓██▒  ▐▌██▒▒██   ██░  ▒   ██▒░ ▓██▓ ░ ▒██▀▀█▄  ", spacing);
    println!("{}░▒████▒▒██████▒▒▒ ▓███▀ ░░▓█▒░██▓ ▓█   ▓██▒   ▒██░   ▓██░░ ████▓▒░▒██████▒▒  ▒██▒ ░ ░██▓ ▒██▒", spacing);
    println!("{}░░ ▒░ ░▒ ▒▓▒ ▒ ░░ ░▒ ▒  ░ ▒ ░░▒░▒ ▒▒   ▓▒█░   ░ ▒░   ▒ ▒ ░ ▒░▒░▒░ ▒ ▒▓▒ ▒ ░  ▒ ░░   ░ ▒▓ ░▒▓░", spacing);
    println!("{} ░ ░  ░░ ░▒  ░ ░  ░  ▒    ▒ ░▒░ ░  ▒   ▒▒ ░   ░ ░░   ░ ▒░  ░ ▒ ▒░ ░ ░▒  ░ ░    ░      ░▒ ░ ▒░", spacing);
    println!("{}   ░   ░  ░  ░  ░         ░  ░░ ░  ░   ▒         ░   ░ ░ ░ ░ ░ ▒  ░  ░  ░    ░        ░░   ░ ", spacing);
    println!("{}   ░  ░      ░  ░ ░       ░  ░  ░      ░  ░            ░     ░ ░        ░              ░     ", spacing);
    println!("{}                ░                                                                            ", spacing);
    println!("");

    let dance_logs = [
      "it's reggae music time",
      "DJ, set the beat now!",
      "Time to shake it off!",
      "Let's dance all night",
      "Dance party, let's go!",
      "Hands up maestro!",
      "In the flow of the rhythm"
    ];
  
    //  random cool message
    let lets_dance = dance_logs.choose(&mut thread_rng()).unwrap();
    println!("{}:: {}", spacing, lets_dance);
    println!("{}:: Entropy algorithm are high affected by people dancing", spacing);
  }

  // compile regex
  let re = Regex::new(&full_regex_pattern)
    .map_err(|e| {
      eprintln!("{}:: Error compiling regex: {}", spacing, e);
      e
    })?;

  println!("{}:: REGEX /{}/", spacing, re.to_string());

  //  run dancing animation
  let mut animation = AnimationThread::new();
  if ndancing {
    animation.start(10, 0);
  }

  println!("{}[{}]", spacing, Local::now().to_rfc3339());
  while running.load(Ordering::SeqCst) {
    // keys
    let secret_key = Keys::generate();
    
    // npub
    let bech32_pubkey = secret_key.public_key().to_bech32()?;

    // check if matches
    if re.is_match(&bech32_pubkey) {
      println!("{}--------------------", spacing);
      println!("{}[{}]npub:\n{}{}", spacing, Local::now().to_rfc3339(), spacing, bech32_pubkey);

      if has_password {
        // ncryptsec
        let ncryptsec = EncryptedSecretKey::new(&secret_key.secret_key(), npassword.clone(), 16, KeySecurity::Medium).unwrap().to_bech32().unwrap();
        let (ncryptsec1, ncryptsec2) = ncryptsec.split_at(81);
        println!("{}[{}]ncryptsec:\n{}{}\n{}{}", spacing, Local::now().to_rfc3339(), spacing, ncryptsec1, spacing, ncryptsec2);
      } else {
        // nsec
        let nsec = &secret_key.secret_key().to_bech32()?;
        println!("{}[{}]nsec:\n{}{}", spacing, Local::now().to_rfc3339(), spacing, nsec);
      }
    }
  }

  if ndancing {
    animation.stop();
  }

  Ok(())
}
