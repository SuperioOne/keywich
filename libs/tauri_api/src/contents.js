import {writeBinaryFile} from "@tauri-apps/api/fs";
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";

// TODO: Move content save system to Rust when switched to Tauri v2.

/**
 * Writes file to applications local content data directory.
 * @param {Uint8Array} file_data data
 * @returns {Promise<string>} created file name.
 */
export async function save_content(file_data) {
  /** @type {string} **/
  const file_name = await invoke("create_guid");
  /** @type {string} **/
  const icon_path = await invoke("get_content_path", {file_name: file_name});
  await writeBinaryFile(icon_path, file_data);

  return file_name;
}

/**
 * Writes file to applications local content data directory.
 * @param {string} file_name data
 * @returns {Promise<string>} created file name.
 */
export async function get_content_url(file_name) {
  /** @type {string} **/
  const data_dir = await invoke("get_content_path", {file_name: file_name});
  return convertFileSrc(data_dir);
}

