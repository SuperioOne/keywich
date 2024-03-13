#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod errors;
mod icon_scheme;
mod result_log;

use crate::commands::password::generate;
use crate::icon_scheme::{icon_protocol_handler, ICON_PROTOCOL};
use clap::{Parser, Subcommand, ValueEnum};
use keyring::Entry;
use keywich_lib::profile::ProfileDB;
use keywich_lib::PasswordConfig;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use serde::Deserialize;
use std::env::args;
use std::fmt::Debug;
use std::io;
use std::io::{stdin, Write};
use std::path::Path;
use std::sync::Arc;
use tauri::async_runtime::RwLock;
use tauri::{AppHandle, Manager};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "keywich")]
#[command(about = "Keywich CLI", long_about = None)]
pub(crate) struct KeywichArgs {
  #[command(subcommand)]
  command: KeywichCommand,
}

#[derive(Debug, Subcommand)]
enum KeywichCommand {
  /// Generate password
  Generate {
    /// Domain for password
    #[arg(short, long)]
    domain: String,
    /// Password character set
    #[arg(short, long)]
    charset: String,
    /// Username for password
    #[arg(short, long)]
    username: String,
    /// Password target length
    #[arg(short, long)]
    target_length: usize,
    /// Password
    #[arg(short, long)]
    password: Option<String>,
    /// Password output type
    #[arg(short, long, default_value_t = PasswordOutputType::Text, default_missing_value = "text", value_enum)]
    output_type: PasswordOutputType,
    /// Seed number
    #[arg(long, default_value_t = 0i64)]
    revision: i64,
  },
  /// Start GUI application
  Gui {
    /// App log level
    #[arg(long)]
    log_level: Option<LevelFilter>,
  },
}

const DEFAULT_CONFIG: &[u8] = br#"{
    "is_light_theme": false,
    "color_theme": "crimson",
    "locale": "en"
}"#;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum PasswordOutputType {
  PHC,
  Text,
  Base64,
  Json,
  Qr,
}

pub(crate) struct AppDbState {
  pub profile_db: Arc<RwLock<Option<ProfileDB>>>,
}

pub(crate) struct KeyState {
  entry: Arc<Entry>,
}

pub(crate) struct LogLevel {
  level: Arc<LevelFilter>,
}

trait DbNotifier {
  fn emit_unlock_required(&self) -> Result<(), tauri::Error>;
}

impl DbNotifier for AppHandle {
  fn emit_unlock_required(&self) -> Result<(), tauri::Error> {
    self.emit_all("unlock_required", ())?;
    Ok(())
  }
}

fn main() {
  if args().len() == 1 {
    start_gui(LevelFilter::Warn)
  } else {
    start_cli()
  }
}

fn start_gui(log_level: LevelFilter) {
  tauri::Builder::default()
    .setup(move |app| {
      let log_dir = &app.path_resolver().app_log_dir().unwrap();
      let log_file_path = log_dir.join("app_log.log");
      let console_logger = ConsoleAppender::builder().build();
      let file_logger = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .build(log_file_path)
        .unwrap();

      let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(console_logger)))
        .appender(Appender::builder().build("logfile", Box::new(file_logger)))
        .build(
          Root::builder()
            .appender("stdout")
            .appender("logfile")
            .build(log_level),
        )
        .unwrap();

      log4rs::init_config(config).unwrap();

      let local_data_dir = &app.path_resolver().app_local_data_dir().unwrap();
      let keyring_entry = Entry::new("keywich", "mdb")?;
      let config_file = Path::join(local_data_dir, "config.json");

      if let Ok(false) = config_file.try_exists() {
        info!("Creating default config file at {:?}", &config_file);
        let mut fs = std::fs::File::create(config_file).unwrap();
        fs.write_all(DEFAULT_CONFIG).unwrap();
        fs.flush().unwrap();
      }

      app.manage(AppDbState {
        profile_db: Arc::from(RwLock::from(None)),
      });
      app.manage(KeyState {
        entry: Arc::from(keyring_entry),
      });
      app.manage(LogLevel {
        level: Arc::from(log_level),
      });

      Ok(())
    })
    .invoke_handler(generate_keywich_handler!())
    .register_uri_scheme_protocol(ICON_PROTOCOL, icon_protocol_handler)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn start_cli() {
  let args = KeywichArgs::parse();
  match args.command {
    KeywichCommand::Generate {
      charset,
      domain,
      output_type,
      password,
      username,
      target_length,
      revision,
    } => {
      // TODO: find a proper way to get user input like stty -echo, read -s
      // Another problem is, does Windows support this?
      let password = password.unwrap_or_else(|| {
        let mut buffer = String::new();
        print!("Password:");
        _ = io::stdout().flush();
        _ = stdin().read_line(&mut buffer);

        buffer
      });

      let config: PasswordConfig = PasswordConfig {
        charset: &charset,
        target_len: target_length,
        domain: &domain,
        username: &username,
        password: &password,
        revision,
      };
      match generate(config, output_type, None) {
        Ok(result) => {
          println!("{}", result);
        }
        Err(err) => {
          eprintln!("{:?}", err);
        }
      }
    }
    KeywichCommand::Gui { log_level } => start_gui(log_level.unwrap_or(LevelFilter::Warn)),
  };
}
