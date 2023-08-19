#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use clap::{Parser, Subcommand, ValueEnum};
use std::ffi::OsString;
use keywitch_lib::{generate_password, Configuration, PasswordResult, OutputType};
use keywitch_lib::errors::KeywitchError;


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
      tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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