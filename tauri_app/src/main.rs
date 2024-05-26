#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod custom_protocols;
mod errors;
mod log_event_appender;
mod result_log;

use self::{
  custom_protocols::{icon_protocol_handler, img_protocol_handler, ICON_PROTOCOL, IMG_PROTOCOL},
  log_event_appender::LogEventAppender,
};
use clap::Parser;
use keyring::Entry;
use keywich_lib::profile::ProfileDB;
use log::{debug, info, warn, LevelFilter};
use log4rs::{
  append::{console::ConsoleAppender, file::FileAppender},
  config::{Appender, Root},
  encode::pattern::PatternEncoder,
  Config,
};
use std::{io::Write, path::Path, sync::Arc};
use tauri::{async_runtime::RwLock, AppHandle, Manager};

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
  let app = tauri::Builder::default()
    .setup(move |app_handle| {
      let args = KeywichArgs::parse();
      let log_file_path = &app_handle
        .path_resolver()
        .app_log_dir()
        .unwrap()
        .join("app_log.log");
      let console_logger = ConsoleAppender::builder().build();
      let file_logger = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}{n}")))
        .build(log_file_path)
        .unwrap();
      let event_logger = LogEventAppender::builder().build(app_handle.handle(), "app_log");
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

      let local_data_dir = &app_handle.path_resolver().app_local_data_dir().unwrap();
      let keyring_entry = Entry::new("keywich", "mdb")?;
      let config_file = Path::join(local_data_dir, "config.json");

      if let Ok(false) = config_file.try_exists() {
        info!("Creating default config file at {:?}", &config_file);
        let mut fs = std::fs::File::create(config_file).unwrap();
        fs.write_all(DEFAULT_CONFIG).unwrap();
        fs.flush().unwrap();
      }

      app_handle.manage(AppDbState {
        profile_db: Arc::from(RwLock::from(None)),
      });

      app_handle.manage(KeyState {
        entry: Arc::from(keyring_entry),
      });

      app_handle.manage(LogLevel {
        level: Arc::from(log_level),
      });

      Ok(())
    })
    .invoke_handler(generate_keywich_handler!())
    .register_uri_scheme_protocol(ICON_PROTOCOL, icon_protocol_handler)
    .register_uri_scheme_protocol(IMG_PROTOCOL, img_protocol_handler)
    .build(tauri::generate_context!())
    .expect("Error while building Keywich application");

  app.run(|app_handle, e| match e {
    tauri::RunEvent::ExitRequested { .. } => {
      let key_state = app_handle.state::<KeyState>();

      match key_state.entry.delete_password() {
        Err(keyring::error::Error::NoEntry) => {
          debug!("Keyring entry already removed.");
        }
        Err(err) => {
          warn!("Unable to clear keyring entry, {}", err);
        }
        _ => {}
      }
    }
    _ => {}
  })
}
