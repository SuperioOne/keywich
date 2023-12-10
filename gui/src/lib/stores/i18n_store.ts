import {writable, get} from "svelte/store";
import {Log} from "$lib";
import {format_string} from "$lib/utils/string_format";

const test: Record<string, string> = {
  "/path/something": "VASDvalue 1",
  "/path": "value 2",
}

export function create_internalization_store() {
  let activeKeys = test;
  const localeStore = writable<string>("en");
  const {set, subscribe, update} = localeStore;
  let cache: Record<string, string> = {};

  return {
    setLocale: (locale: string) => {
      cache = {};
      set(locale);
    },
    subscribe,
    getKey: (keyURI: string | URL) => {
      if (typeof keyURI === "string" && cache[keyURI]) {
        return cache[keyURI];
      }

      let target: URL = typeof keyURI === "string"
        ? new URL(keyURI)
        : keyURI;

      const key = target.pathname.toLowerCase();
      const defaultValue = key;

      if (target.protocol !== "i18:") {
        Log.error(`Invalid key URI: ${target}`);
        return defaultValue
      }

      let value = activeKeys[key];

      if (!value) {
        Log.error(`Invalid key URI does not exists: ${target}`);
        return defaultValue;
      }

      if (target.searchParams.size > 0) {
        const currentLocale = get(localeStore) ?? "en";

        if (target.searchParams.has("toUpper")) {
          value = value.toLocaleUpperCase(currentLocale);
        } else if (target.searchParams.has("toLower")) {
          value = value.toLocaleLowerCase(currentLocale);
        }
      }

      cache[target.toString()] = value;
      return value;
    }
  }
}

const CaseFlag = {
  None: 0,
  UpperCase: 1,
  LowerCase: -1,
} as const;
type CaseFlagType = 0 | 1 | -1

function apply_operators(text: string, localizationURI: URL, locale: string) {
  const currentLocale = locale ?? undefined;
  let replaceArgs: Record<string, any> = {}
  let caseFlag: CaseFlagType = CaseFlag.None;

  for (const [key, param] of localizationURI.searchParams) {

    switch (key) {
      case "$toUpper" :
        caseFlag = CaseFlag.UpperCase;
        break;
      case "$toLower" :
        caseFlag = CaseFlag.LowerCase;
        break;
      default:
        replaceArgs[key] = param;
        break;
    }
  }

  let formattedText = format_string(text, replaceArgs);

  switch (caseFlag) {
    case CaseFlag.UpperCase:
      return formattedText.toLocaleUpperCase(currentLocale);
    case CaseFlag.LowerCase:
      return formattedText.toLocaleLowerCase(currentLocale);
    default:
      return formattedText;
  }
}

export const i18nStore = create_internalization_store();