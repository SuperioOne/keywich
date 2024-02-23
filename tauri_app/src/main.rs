#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod errors;
mod icon_scheme;

use crate::commands::password::generate;
use crate::icon_scheme::{icon_protocol_handler, ICON_PROTOCOL};
use clap::{Parser, Subcommand, ValueEnum};
use keyring::Entry;
use keywich_lib::profile::ProfileDB;
use keywich_lib::PasswordConfig;
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
  GUI,
}

const DEFAULT_CONFIG: &'static [u8] = br#"{
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

trait DbNotifier {
  fn notify_db_status(&self) -> Result<(), tauri::Error>;
}

impl DbNotifier for AppHandle {
  fn notify_db_status(&self) -> Result<(), tauri::Error> {
    let _ = self.emit_all("unlock_required", ())?;
    Ok(())
  }
}

fn main() {
  if args().len() == 1 {
    start_gui()
  } else {
    start_cli()
  }
}

fn start_gui() {
  tauri::Builder::default()
    .setup(|app| {
      let app_data_dir = &app.path_resolver().app_data_dir().unwrap();
      let keyring_entry = Entry::new("keywich", "mdb")?;
      let config_file = Path::join(&app_data_dir, "config.json");

      if !config_file.exists() {
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
    KeywichCommand::GUI => start_gui(),
  };
}
