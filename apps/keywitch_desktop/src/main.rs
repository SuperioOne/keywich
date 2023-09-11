#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env::args;
use clap::{Parser, Subcommand, ValueEnum};
use std::ffi::OsString;
use std::fmt::Debug;
use keywitch_lib::{generate_password, Configuration, PasswordResult, OutputType};
use keywitch_lib::errors::KeywitchError;
use keywitch_lib::profile::models::{CharsetItem, PassMetadataItem};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "keywitch")]
#[command(about = "Keywitch CLI", long_about = None)]
pub(crate) struct KeywitchCli {
  #[command(subcommand)]
  command: KeywitchCommand,
}

#[derive(Debug, Subcommand)]
enum KeywitchCommand
{
  /// Generate password
  Generate {
    /// Password domain
    #[arg(short, long)]
    domain: OsString,
    /// Password character set
    #[arg(short, long)]
    charset: OsString,
    /// Password salt value
    #[arg(short, long)]
    salt: OsString,
    /// Password salt value
    #[arg(short, long)]
    target_length: usize,
    /// Password
    #[arg(short, long)]
    password: OsString,
    /// Password
    #[arg(short, long, default_value_t = PasswordOutputType::Text, default_missing_value = "text", value_enum)]
    output_type: PasswordOutputType,
  },
  /// Start GUI application
  GUI,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum PasswordOutputType
{
  PasswordText,
  Text,
  Base64,
  Json,
}

impl Into<OutputType> for PasswordOutputType
{
  fn into(self) -> OutputType {
    match self {
      PasswordOutputType::PasswordText => OutputType::PasswordText,
      PasswordOutputType::Text => OutputType::Text,
      PasswordOutputType::Base64 => OutputType::Base64,
      PasswordOutputType::Json => OutputType::Json
    }
  }
}

fn main() {
  if args().len() == 1
  {
    start_gui()
  } else {
    start_cli()
  }
}

#[tauri::command(rename_all = "snake_case")]
fn get_passwords() -> Result<Vec<PassMetadataItem>, String> {
  keywitch_lib::profile::get_pass_metadata_collection().map_err(|e| format!("{:?}", e))
}

#[tauri::command(rename_all = "snake_case")]
fn get_pinned() -> Result<Vec<PassMetadataItem>, String> {
  keywitch_lib::profile::get_pinned_pass_collection().map_err(|e| format!("{:?}", e))
}

#[tauri::command(rename_all = "snake_case")]
fn get_charsets() -> Result<Vec<CharsetItem>, String> {
  keywitch_lib::profile::get_charset_collection().map_err(|e| format!("{:?}", e))
}

fn start_gui()
{
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_charsets, get_pinned, get_passwords])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn start_cli()
{
  let args = KeywitchCli::parse();
  match args.command
  {
    KeywitchCommand::Generate {
      charset,
      domain,
      output_type,
      password,
      salt,
      target_length,
    } => {
      match command_generate(
        &domain,
        &password,
        &salt,
        &charset,
        target_length,
      ) {
        Ok(result) => {
          let result_buffer = result.to_formatted_string(output_type.into());
          println!("{}", result_buffer);
        }
        Err(err) => {
          eprintln!("{:?}", err);
        }
      }
    }
    KeywitchCommand::GUI => {
      start_gui()
    }
  };
}

fn command_generate(
  domain: &OsString,
  password: &OsString,
  salt: &OsString,
  charset: &OsString,
  target_length: usize,
) -> Result<PasswordResult, KeywitchError>
{
  let configuration = Configuration::new(
    domain.to_str().unwrap(),
    password.to_str().unwrap(),
    salt.to_str().unwrap(),
    charset.to_str().unwrap(),
    target_length,
  )?;

  let password = generate_password(&configuration)?;
  Ok(password)
}