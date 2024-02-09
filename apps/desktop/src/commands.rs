pub mod charsets;
pub mod keys;
pub mod password;
pub mod utilities;

#[macro_export]
macro_rules! generate_keywich_handler {
  () => {
    tauri::generate_handler![
      $crate::commands::password::generate_password,
      $crate::commands::password::generate_password_from,
      $crate::commands::charsets::get_charsets,
      $crate::commands::charsets::insert_charset,
      $crate::commands::charsets::delete_charset,
      $crate::commands::keys::get_keys,
      $crate::commands::keys::get_pinned_keys,
      $crate::commands::keys::search_keys,
      $crate::commands::keys::delete_key,
      $crate::commands::keys::insert_key,
      $crate::commands::keys::update_key,
      $crate::commands::keys::pin_key,
      $crate::commands::keys::unpin_key,
      $crate::commands::keys::get_key_by_id,
      $crate::commands::utilities::create_guid,
      $crate::commands::utilities::get_content_path,
      $crate::commands::utilities::get_config_path,
    ]
  };
}
