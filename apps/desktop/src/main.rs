#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use clap::{Parser, Subcommand, ValueEnum};
use keywich_lib::charset::Charset;
use keywich_lib::errors::Error;
use keywich_lib::{generate_password, Configuration, OutputType, PasswordResult};
use std::env::args;
use std::fmt::Debug;

/// A fictional versioning CLI
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
    /// Password domain
    #[arg(short, long)]
    domain: String,
    /// Password character set
    #[arg(short, long)]
    charset: String,
    /// Password salt value
    #[arg(short, long)]
    salt: String,
    /// Password salt value
    #[arg(short, long)]
    target_length: usize,
    /// Password
    #[arg(short, long)]
    password: String,
    /// Password
    #[arg(short, long, default_value_t = PasswordOutputType::Text, default_missing_value = "text", value_enum)]
    output_type: PasswordOutputType,
  },
  /// Start GUI application
  GUI,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum PasswordOutputType {
  PasswordText,
  Text,
  Base64,
  Json,
}

impl Into<OutputType> for PasswordOutputType {
  fn into(self) -> OutputType {
    match self {
      PasswordOutputType::PasswordText => OutputType::PasswordText,
      PasswordOutputType::Text => OutputType::Text,
      PasswordOutputType::Base64 => OutputType::Base64,
      PasswordOutputType::Json => OutputType::Json,
    }
  }
}

fn main() {
  if args().len() == 1 {
    start_gui()
  } else {
    start_cli()
  }
}

//
// #[tauri::command(rename_all = "snake_case")]
// fn get_passwords() -> Result<Vec<PassMetadataItem>, String> {
//   keywich_lib::profile::get_pass_metadata_collection().map_err(|e| format!("{:?}", e))
// }
//
// #[tauri::command(rename_all = "snake_case")]
// fn get_pinned() -> Result<Vec<PassMetadataItem>, String> {
//   keywich_lib::profile::get_pinned_pass_collection().map_err(|e| format!("{:?}", e))
// }
//
// #[tauri::command(rename_all = "snake_case")]
// fn get_charsets() -> Result<Vec<CharsetItem>, String> {
//   keywich_lib::profile::get_charset_collection().map_err(|e| format!("{:?}", e))
// }

fn start_gui() {
  tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![
    //   get_charsets,
    //   get_pinned,
    //   get_passwords
    // ])
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
      salt,
      target_length,
    } => match command_generate(&domain, &password, &salt, &charset, target_length) {
      Ok(result) => {
        let result_buffer = result.to_formatted_string(output_type.into());
        println!("{}", result_buffer);
      }
      Err(err) => {
        eprintln!("{:?}", err);
      }
    },
    KeywichCommand::GUI => start_gui(),
  };
}

fn command_generate(
  domain: &str,
  password: &str,
  salt: &str,
  charset: &str,
  target_length: usize,
) -> Result<PasswordResult, Error> {
  let configuration = Configuration::new(domain, password, salt, charset, target_length)?;

  let password = generate_password(&configuration)?;
  Ok(password)
}
