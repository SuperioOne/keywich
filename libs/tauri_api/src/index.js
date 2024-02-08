/**
 * @typedef {import("@keywich/api").KeyItem} KeyItem
 * @typedef {import("@keywich/api").CharsetItem} CharsetItem
 * @typedef {import("@keywich/api").CharsetOptions} CharsetOptions
 * @typedef {import("@keywich/api").KeyRequest} KeyRequest
 * @typedef {import("@keywich/api").SearchQuery} SearchQuery
 * @typedef {import("@keywich/api").PasswordGenerateRequest} PasswordGenerateRequest
 * @typedef {{
 *   target_size: number;
 *   revision: number;
 *   charset: string;
 *   domain: string;
 *   username: string;
 *   notes?: string;
 *   pinned: boolean;
 *   tags?: string[];
 *   version?: string;
 *   custom_icon?: string;
 * }} KeyOptions
 */

import {invoke} from "@tauri-apps/api/tauri";
import {is_null_or_empty, or_default} from "@keywich/api/utils";
import {save} from "@tauri-apps/api/dialog";
import {writeBinaryFile} from "@tauri-apps/api/fs";
import {writeText} from "@tauri-apps/api/clipboard";
import {get_content_url, save_content} from "./contents.js";

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
      icon_name = await save_content(data.custom_icon);
    }

    /** @type {KeyOptions} */
    const key_data = {
      charset: data.charset,
      username: data.username,
      tags: or_default(data.tags, []),
      domain: data.domain,
      notes: data.notes,
      version: or_default(data.version, "kw_scrypt:v1"),
      pinned: false,
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
    if (data.custom_icon && data.custom_icon.length > 0) {
      icon_name = await save_content(data.custom_icon);
    }

    /** @type {KeyOptions} */
    const key_data = {
      charset: data.charset,
      username: data.username,
      tags: or_default(data.tags, []),
      domain: data.domain,
      notes: data.notes,
      version: or_default(data.version, "kw_scrypt:v1"),
      pinned: false,
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

      // action is cancelled, if target path is still null.
      if (is_null_or_empty(target_path)) {
        return false;
      }
    }

    await writeBinaryFile(target_path, fileData);
    return true;
  },

  get_system_locale: function () {
    throw new Error("Function not implemented.");
  },

  load_locale: function (locale) {
    throw new Error("Function not implemented.");
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

  convert_content_src: function (content_name) {
    return get_content_url(content_name);
  }
}

export default _api;