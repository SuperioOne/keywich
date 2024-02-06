#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod errors;

use crate::commands::password::generate;
use clap::{Parser, Subcommand, ValueEnum};
use keywich_lib::profile::ProfileDB;
use keywich_lib::PasswordConfig;
use serde::Deserialize;
use std::env::args;
use std::fmt::Debug;
use std::io;
use std::io::{stdin, Write};
use std::path::Path;
use tauri::Manager;

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

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum PasswordOutputType {
  PHC,
  Text,
  Base64,
  Json,
  Qr,
}

pub(crate) struct AppRpcState {
  pub profile_db: ProfileDB,
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
      // TODO: get rid of unwraps

      let app_data_dir = &app.path_resolver().app_data_dir().unwrap();
      let db_path = Path::join(&app_data_dir, "app.db");
      let path_str = db_path.to_str().unwrap();
      let connection_string = format!("sqlite:{}", path_str);
      let profile_db =
        tauri::async_runtime::block_on(ProfileDB::connect(&connection_string)).unwrap();

      app.manage(AppRpcState { profile_db });

      Ok(())
    })
    .invoke_handler(generate_keywich_handler!())
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
