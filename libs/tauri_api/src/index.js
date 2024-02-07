/**
 * @typedef {import("@keywich/api").KeyItem} KeyItem
 * @typedef {import("@keywich/api").CharsetItem} CharsetItem
 * @typedef {import("@keywich/api").CharsetOptions} CharsetOptions
 * @typedef {import("@keywich/api").KeyOptions} KeyOptions
 * @typedef {import("@keywich/api").SearchQuery} SearchQuery
 * @typedef {import("@keywich/api").PasswordGenerateRequest} PasswordGenerateRequest
 */

import {invoke} from "@tauri-apps/api/tauri";
import {writeText} from "@tauri-apps/api/clipboard";
import {save} from "@tauri-apps/api/dialog";
import {writeBinaryFile} from "@tauri-apps/api/fs";

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
    console.debug(data);
    return invoke("insert_key", {data: data});
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

  update_key: function (id, data) {
    return invoke("update_key", {key_id: id, data: data});
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

    if (!target_path || (typeof target_path === "string" && target_path.trim().length === 0)) {
      target_path = await save();

      if (!target_path) {
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

  convert_file_src: function (path) {
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
    return writeText(value)
  }
}

export default _api;