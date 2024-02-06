pub mod password;
pub mod profile;

#[macro_export]
macro_rules! generate_keywich_handler {
  () => {
    tauri::generate_handler![
      $crate::commands::password::generate_password,
      $crate::commands::password::generate_password_from,
      $crate::commands::profile::get_charsets,
      $crate::commands::profile::insert_charset,
      $crate::commands::profile::delete_charset,
      $crate::commands::profile::get_keys,
      $crate::commands::profile::get_pinned_keys,
      $crate::commands::profile::search_keys,
      $crate::commands::profile::delete_key,
      $crate::commands::profile::insert_key,
      $crate::commands::profile::update_key,
      $crate::commands::profile::pin_key,
      $crate::commands::profile::unpin_key,
      $crate::commands::profile::get_key_by_id,
    ]
  };
}
