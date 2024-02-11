import format_string from "@superior-one/format_string";
import {Log} from "../logger";
import {writable} from "svelte/store";
import {create_debouncer} from "@keywich/api/utils";
import {RPC} from "../rpc";

const CaseFlag = {None: 0, UpperCase: 1, LowerCase: -1} as const;
type CaseFlagType = 0 | 1 | -1

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

class i18Resource {
  readonly #resources: Record<string, string>;
  readonly #cache: Record<string, string>;
  readonly #locale: string;

  get current_locale() {
    return this.#locale;
  }

  constructor(locale: string, resources: Record<string, string>) {
    this.#resources = resources;
    this.#cache = {};
    this.#locale = locale
  }

  get_key(keyURI: string | URL, fallback?: string) {
    if (typeof keyURI === "string" && this.#cache[keyURI]) {
      return this.#cache[keyURI];
    }

    const target: URL = typeof keyURI === "string" ? new URL(keyURI) : keyURI;
    const key = target.pathname.toLowerCase();

    if (target.protocol !== "i18:") {
      Log.error(`Invalid key URI: ${target}`);
      return fallback ?? key;
    }

    let value = this.#resources[key];

    if (!value) {
      Log.warn(`Invalid key URI does not exists: ${target}`);
      return fallback ?? key;
    }

    if (target.searchParams.size > 0) {
      value = apply_operators(value, target, this.#locale);
    }

    if (!target.searchParams.has("$noCache")) {
      this.#cache[keyURI.toString()] = value;
    }

    return value;
  }
}

const locale_store = writable<i18Resource>(new i18Resource("en", {}));
const locale_loader = create_debouncer(
  async (locale: string) => {
    const resources = await RPC.load_locale(locale);
    return {resources, locale};
  },
  {
    timeout: 500,
    onError: Log.error,
    onSuccess: (data) => {
      locale_store.set(new i18Resource(data.locale, data.resources));
    }
  });

function set_locale(locale: string) {
  locale_loader.update(locale);
}

function init_locale(locale: string, resources: Record<string, string>) {
  locale_store.set(new i18Resource(locale, resources));
}

export const i18nStore = {
  init_locale,
  set_locale,
  subscribe: locale_store.subscribe,
};
