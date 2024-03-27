#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod errors;
mod icon_scheme;
mod log_event_appender;
mod result_log;

use crate::icon_scheme::{icon_protocol_handler, ICON_PROTOCOL};
use clap::Parser;
use keyring::Entry;
use keywich_lib::profile::ProfileDB;
use log::{info, LevelFilter};
use log4rs::{
  append::{console::ConsoleAppender, file::FileAppender},
  config::{Appender, Root},
  encode::pattern::PatternEncoder,
  Config,
};
use std::{io::Write, path::Path, sync::Arc};
use tauri::{async_runtime::RwLock, AppHandle, Manager};

use self::log_event_appender::{LogEventAppender, LogEventAppenderBuilder};

#[derive(Debug, Parser)]
#[command(name = "keywich")]
#[command(about = "Keywich CLI", long_about = None)]
pub(crate) struct KeywichArgs {
  /// App log level
  #[arg(long)]
  pub verbose: Option<LevelFilter>,
}

const DEFAULT_CONFIG: &[u8] = br#"{
    "is_light_theme": false,
    "color_theme": "crimson",
    "locale": "en"
}"#;

pub(crate) struct AppDbState {
  pub profile_db: Arc<RwLock<Option<ProfileDB>>>,
}

pub(crate) struct KeyState {
  pub entry: Arc<Entry>,
}

pub(crate) struct LogLevel {
  pub level: Arc<LevelFilter>,
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
  tauri::Builder::default()
    .setup(move |app| {
      let args = KeywichArgs::parse();
      let log_file_path = &app
        .path_resolver()
        .app_log_dir()
        .unwrap()
        .join("app_log.log");
      let console_logger = ConsoleAppender::builder().build();
      let file_logger = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .build(log_file_path)
        .unwrap();
      let event_logger = LogEventAppender::builder().build(app.handle(), "app_log");
      let log_level = args.verbose.unwrap_or(LevelFilter::Warn);
      let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(console_logger)))
        .appender(Appender::builder().build("logfile", Box::new(file_logger)))
        .appender(Appender::builder().build("eventlog", Box::new(event_logger)))
        .build(
          Root::builder()
            .appender("stdout")
            .appender("logfile")
            .appender("eventlog")
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
