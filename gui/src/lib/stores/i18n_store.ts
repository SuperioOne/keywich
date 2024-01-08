import EN from "../../locales/en.json"
import format_string from "@superior-one/format_string";
import {Log} from "$lib";
import {writable, get} from "svelte/store";

const CaseFlag = {
  None: 0,
  UpperCase: 1,
  LowerCase: -1,
} as const;
type CaseFlagType = 0 | 1 | -1

export function create_internalization_store() {
  const activeKeys: Record<string, string> = EN; // TODO: replace with proper loader
  const localeStore = writable<string>("en");
  const {set, subscribe} = localeStore;
  let cache: Record<string, string> = {};

  return {
    setLocale: (locale: string) => {
      cache = {};
      set(locale);
    },
    subscribe,
    getKey: (keyURI: string | URL, fallback?: string) => {
      if (typeof keyURI === "string" && cache[keyURI]) {
        return cache[keyURI];
      }

      const target: URL = typeof keyURI === "string"
        ? new URL(keyURI)
        : keyURI;

      const key = target.pathname.toLowerCase();

      if (target.protocol !== "i18:") {
        Log.error(`Invalid key URI: ${target}`);
        return fallback ?? key;
      }

      let value = activeKeys[key];

      if (!value) {
        Log.error(`Invalid key URI does not exists: ${target}`);
        return fallback ?? key;
      }

      if (target.searchParams.size > 0) {
        const currentLocale = get(localeStore) ?? "en";
        value = apply_operators(value, target, currentLocale);
      }

      if (!target.searchParams.has("$noCache")) {
        cache[keyURI.toString()] = value;
      }

      return value;
    }
  }
}

function apply_operators(text: string, localizationURI: URL, locale: string) {
  const currentLocale = locale ?? undefined;
  const replaceArgs: Record<string, unknown> = {}
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

  const formattedText = format_string(text, replaceArgs);

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