/**
 * @typedef {import("@keywich/api").KeyItem} KeyItem
 * @typedef {import("@keywich/api").CharsetItem} CharsetItem
 * @typedef {import("@keywich/api").CharsetOptions} CharsetOptions
 * @typedef {import("@keywich/api").KeyRequest} KeyRequest
 * @typedef {import("@keywich/api").PasswordGenerateRequest} PasswordGenerateRequest
 * @typedef {{
 *   target_size: number;
 *   revision: number;
 *   charset: string;
 *   domain: string;
 *   username: string;
 *   notes?: string;
 *   tags?: string[];
 *   version?: string;
 *   custom_icon?: string;
 * }} KeyOptions
 */

import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
import {is_null_or_empty, or_default} from "@keywich/api/utils";
import {save} from "@tauri-apps/api/dialog";
import {writeBinaryFile, readTextFile, writeTextFile, removeFile} from "@tauri-apps/api/fs";
import {writeText} from "@tauri-apps/api/clipboard";

const DEFAULT_HASH_VERSION = "kw_scrypt:v1";

/**
 * @param {Uint8Array} data
 * @returns {Promise<string>}
 */
async function upload_icon(data) {
  /** @type {string} **/
  const temp_path = await invoke("alloc_temp_path");
  await writeBinaryFile(temp_path, data);

  setTimeout(() => {
    removeFile(temp_path)
      .then(() => {
      })
      .catch(err => {
        // TODO: connect with own logger
        console.error(err);
      });
  }, 0)

  /** @type {string} **/
  return await invoke("process_icon", {file_path: temp_path});
}

/** @type {import("@keywich/api").KeywichRpcApi} */
const _api = {
  delete_key: function (id) {
    return invoke("delete_key", {key_id: id});
  },

  get_key_by_id: function (id) {
    return invoke("get_key_by_id", {key_id: id});
  },

  get_keys: function () {
    return invoke("get_keys");
  },

  get_pinned_keys: function () {
    return invoke("get_pinned_keys");
  },

  insert_key: async function (data) {
    /** @type {string | undefined} */
    let icon_name = undefined;

    if (data.custom_icon && data.custom_icon.length > 0) {
      icon_name = await upload_icon(data.custom_icon);
    }

    /** @type {KeyOptions} */
    const key_data = {
      charset: data.charset,
      username: data.username,
      tags: or_default(data.tags, []),
      domain: data.domain,
      notes: data.notes,
      version: or_default(data.version, DEFAULT_HASH_VERSION),
      target_size: data.target_size,
      revision: or_default(data.revision, 0),
      custom_icon: icon_name
    };

    return invoke("insert_key", {data: key_data});
  },

  pin_key: function (id) {
    return invoke("pin_key", {key_id: id});
  },

  search_keys: function (query) {
    return invoke("search_keys", {query: query});
  },

  unpin_key: function (id) {
    return invoke("unpin_key", {key_id: id});
  },

  update_key: async function (id, data) {
    /** @type {string | undefined} */
    let icon_name = undefined;

    if (data.custom_icon) {
      switch (data.custom_icon.type) {
        case "buffer":
          icon_name = await upload_icon(data.custom_icon.data);
          break;
        case "name":
          icon_name = data.custom_icon.name;
          break;
      }
    }

    /** @type {KeyOptions} */
    const key_data = {
      charset: data.charset,
      username: data.username,
      tags: or_default(data.tags, []),
      domain: data.domain,
      notes: data.notes,
      version: or_default(data.version, DEFAULT_HASH_VERSION),
      target_size: data.target_size,
      revision: or_default(data.revision, 0),
      custom_icon: icon_name
    };

    return invoke("update_key", {key_id: id, data: key_data});
  },

  generate_password_from: function (request) {
    return invoke("generate_password_from", {request: request});
  },

  generate_password: function (request) {
    return invoke("generate_password", {request: request});
  },

  save_file: async function (fileData, path) {
    /** @type{string | null | undefined} **/
    let target_path = path;

    if (is_null_or_empty(target_path)) {
      target_path = await save();

      // If target path is still null, action is cancelled.
      if (is_null_or_empty(target_path)) {
        return false;
      }
    }

    await writeBinaryFile(target_path, fileData);
    return true;
  },

  load_locale: async function (locale) {
    /** @type {string} **/
    const path = await invoke("get_locale_path", {locale: locale});
    const locale_content = await readTextFile(path);

    return JSON.parse(locale_content);
  },

  insert_charset: function (charset) {
    return invoke("insert_charset", {charset: charset});
  },

  delete_charset: function (name) {
    return invoke("delete_charset", {name: name});
  },

  get_charsets: function () {
    return invoke("get_charsets");
  },

  copy_to_clipboard: function (value) {
    return writeText(value);
  },

  convert_icon_src: function (icon_name) {
    return convertFileSrc(icon_name, "kwicon");
  },

  update_config_json: async function (configs) {
    /** @type {string} **/
    const config_path = await invoke("get_config_path");
    await writeTextFile(config_path, JSON.stringify(configs, null, 4));
  },

  get_config_json: async function () {
    /** @type {string} **/
    const config_path = await invoke("get_config_path");
    const content = await readTextFile(config_path);
    return JSON.parse(content);
  },

  upload_icon: upload_icon,

  login: function (password) {
    return invoke("unlock_db", {master_pass: password});
  },

  logout: function () {
    return invoke("lock_db");
  },

  load_configs: function () {
    return invoke("load_configs");
  }
}

export default _api;