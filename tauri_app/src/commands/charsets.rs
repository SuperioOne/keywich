use crate::errors::AppErrors;
use crate::AppRpcState;
use keywich_lib::profile::charsets::CharsetItem;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_charsets(state: State<'_, AppRpcState>) -> Result<Vec<CharsetItem>, AppErrors> {
  let result = state.profile_db.get_charsets().await?;
  Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_charset(
  state: State<'_, AppRpcState>,
  charset: CharsetItem,
) -> Result<String, AppErrors> {
  let result = state.profile_db.insert_charset(charset).await?;
  Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_charset(state: State<'_, AppRpcState>, name: String) -> Result<(), AppErrors> {
  state.profile_db.delete_charset(&name).await?;
  Ok(())
}
