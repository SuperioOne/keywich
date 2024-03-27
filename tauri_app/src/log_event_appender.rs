use std::string::FromUtf8Error;
use std::sync::Arc;

use log4rs::append::Append;
use log4rs::encode::json::JsonEncoder;
use log4rs::encode::Encode;
use tauri::{AppHandle, Manager};

#[derive(Debug)]
pub struct LogEventAppender {
  inner_encoder: Box<dyn Encode>,
  emitter: Arc<AppHandle>,
  event_name: Box<str>,
}

pub struct LogEventAppenderBuilder {
  inner_encoder: Option<Box<dyn Encode>>,
}

impl LogEventAppender {
  pub fn builder() -> LogEventAppenderBuilder {
    LogEventAppenderBuilder {
      inner_encoder: None,
    }
  }
}

impl LogEventAppenderBuilder {
  #[allow(dead_code)]
  pub fn encoder(mut self, encoder: Box<dyn Encode>) -> Self {
    self.inner_encoder = Some(encoder);
    self
  }

  pub fn build(self, handler: AppHandle, event_name: &str) -> LogEventAppender {
    let encoder = match self.inner_encoder {
      Some(e) => e,
      None => Box::from(JsonEncoder::new()),
    };

    LogEventAppender {
      event_name: Box::from(event_name),
      emitter: Arc::from(handler),
      inner_encoder: encoder,
    }
  }
}

struct EventWriter {
  inner: Vec<u8>,
}

impl EventWriter {
  pub fn new() -> Self {
    Self { inner: Vec::new() }
  }

  #[allow(dead_code)]
  pub fn read_bytes(self) -> Vec<u8> {
    self.inner
  }

  pub fn read_string(self) -> Result<String, FromUtf8Error> {
    String::from_utf8(self.inner)
  }
}

impl std::io::Write for EventWriter {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.inner.write(buf)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.inner.flush()
  }
}

impl log4rs::encode::Write for EventWriter {}

impl Append for LogEventAppender {
  fn append(&self, record: &log::Record) -> anyhow::Result<()> {
    let mut writer = EventWriter::new();
    self.inner_encoder.as_ref().encode(&mut writer, record)?;
    let message = writer.read_string()?;

    self
      .emitter
      .as_ref()
      .emit_all(self.event_name.as_ref(), message)?;

    Ok(())
  }

  fn flush(&self) {}
}
