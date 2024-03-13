use log::error;
use std::fmt::Display;

pub(crate) trait ResultLog {
  fn log_err(self) -> Self;
}

impl<T, E> ResultLog for Result<T, E>
where
  E: Display,
{
  /// Logs error with [log::error](https://docs.rs/log/latest/log/macro.error.html) when `Result<T,E>`
  /// is `Err(E)` and `E` implements `Display` trait.
  ///
  /// # Example
  /// ```rust
  /// some_func().log_err()?;
  ///
  /// if let Ok(result) = async_func().await.log_err() {
  ///  // Do other stuff...
  /// }
  /// ```
  fn log_err(self) -> Self {
    if let Err(err) = &self {
      error!("{}", err);
    }

    self
  }
}
