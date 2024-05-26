import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { is_null_or_empty, or_default } from "./utils";
import { save, open } from "@tauri-apps/api/dialog";
import {
  writeBinaryFile,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/api/fs";
import { writeText } from "@tauri-apps/api/clipboard";
import type { KeyOptions, KeywichApi, VerifyResponse } from "./api/types";

export * from "./api/types";

const DEFAULT_HASH_VERSION = "kw_scrypt:v1";

function upload_icon(path: string): Promise<string> {
  return invoke("process_icon", { file_path: path });
}

export const Api: KeywichApi = {
  delete_key: function (id) {
    return invoke("delete_key", { key_id: id });
  },

  get_key_by_id: function (id) {
    return invoke("get_key_by_id", { key_id: id });
  },

  get_keys: function () {
    return invoke("get_keys");
  },

  get_pinned_keys: function () {
    return invoke("get_pinned_keys");
  },

  insert_key: async function (data) {
    let icon_name: string | undefined = undefined;

    if (!is_null_or_empty(data.custom_icon)) {
      icon_name = await upload_icon(data.custom_icon);
    }

    const key_data: KeyOptions = {
      charset: data.charset,
      username: data.username,
      tags: or_default(data.tags, []),
      domain: data.domain,
      notes: data.notes,
      version: or_default(data.version, DEFAULT_HASH_VERSION),
      target_size: data.target_size,
      revision: or_default(data.revision, 0),
      custom_icon: icon_name,
    };

    return invoke("insert_key", { data: key_data });
  },

  pin_key: function (id) {
    return invoke("pin_key", { key_id: id });
  },

  search_keys: function (query) {
    return invoke("search_keys", { query: query });
  },

  unpin_key: function (id) {
    return invoke("unpin_key", { key_id: id });
  },

  update_key: async function (id, data) {
    let icon_name: string | undefined = undefined;

    if (data.custom_icon) {
      switch (data.custom_icon.type) {
        case "path":
          icon_name = await upload_icon(data.custom_icon.path);
          break;
        case "name":
          icon_name = data.custom_icon.name;
          break;
      }
    }

    const key_data: KeyOptions = {
      charset: data.charset,
      username: data.username,
      tags: or_default(data.tags, []),
      domain: data.domain,
      notes: data.notes,
      version: or_default(data.version, DEFAULT_HASH_VERSION),
      target_size: data.target_size,
      revision: or_default(data.revision, 0),
      custom_icon: icon_name,
    };

    return invoke("update_key", { key_id: id, data: key_data });
  },

  generate_password_from: function (request) {
    return invoke("generate_password_from", { request: request });
  },

  generate_password: function (request) {
    return invoke("generate_password", { request: request });
  },

  save_file: async function (fileData, path) {
    let target_path: string | null | undefined = path;

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
    const path: string = await invoke("get_locale_path", { locale: locale });
    const locale_content = await readTextFile(path);

    return JSON.parse(locale_content);
  },

  insert_charset: function (charset) {
    return invoke("insert_charset", { charset: charset });
  },

  delete_charset: function (name) {
    return invoke("delete_charset", { name: name });
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

  convert_img_src: function (img_path) {
    return convertFileSrc(img_path, "kwimg");
  },

  update_config_json: async function (configs) {
    const config_path: string = await invoke("get_config_path");
    await writeTextFile(config_path, JSON.stringify(configs, null, 4));
  },

  get_config_json: async function () {
    const config_path: string = await invoke("get_config_path");
    const content = await readTextFile(config_path);
    return JSON.parse(content);
  },

  upload_icon: upload_icon,

  login: function (password) {
    return invoke("unlock_db", { master_pass: password });
  },

  logout: function () {
    return invoke("lock_db");
  },

  load_configs: function () {
    return invoke("load_configs");
  },

  backup: async function (path?: string) {
    let target = is_null_or_empty(path)
      ? await save({
          filters: [{ name: "Keywich Backup", extensions: ["kb"] }],
          title: "Keywich Backup",
        })
      : path;

    if (is_null_or_empty(target)) {
      throw new Error("Export path is still empty.");
    }

    if (!target.endsWith(".kb")) {
      target = target + ".kb";
    }

    return await invoke("backup_profile_db", { export_path: target });
  },

  restore: async function (path?: string): Promise<void> {
    const target = is_null_or_empty(path)
      ? await open({
          multiple: false,
          directory: false,
          filters: [{ name: "Keywich Backup", extensions: ["kb"] }],
        })
      : path;

    if (is_null_or_empty(target)) {
      throw new Error("Import file path is still empty.");
    }

    return await invoke("restore_profile_db", { import_path: target });
  },

  verify_backup: async function (path?: string): Promise<VerifyResponse> {
    const target = (
      is_null_or_empty(path)
        ? await open({
            multiple: false,
            directory: false,
            filters: [{ name: "Keywich Backup", extensions: ["kb"] }],
          })
        : path
    ) as string | null;

    if (is_null_or_empty(target)) {
      throw new Error("Backup file path is still empty.");
    }

    return await invoke("verify_backup", { import_path: target });
  },

  select_file: async function (
    extensions?: string[] | undefined,
  ): Promise<string | undefined> {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: extensions
        ? [{ name: "File", extensions: extensions }]
        : undefined,
    });

    return (selected as string) ?? undefined;
  },
};
