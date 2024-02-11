import {writable} from "svelte/store";
import type {TokenType} from "../utils/key_filter_tokenizer";
import {create_debouncer, is_null_or_empty} from "@keywich/api/utils";
import {Log} from "$lib/logger";

export type FilterHistoryItem = {
  value: string;
  timestamp: number;
}

const HISTORY_LIMIT = 15;
const STORAGE_KEY = "history_filter"

function loader(): FilterHistoryItem[] {
  try {
    const historyItems = localStorage.getItem(STORAGE_KEY)
    if (is_null_or_empty(historyItems)) {
      return [];
    } else {
      return JSON.parse(historyItems);
    }
  } catch (err) {
    Log.warn(" Filter history format is incorrect. App storage database might be corrupted or altered.")
    return [];
  }
}

const {update, set, subscribe} = writable<FilterHistoryItem[]>(loader());
const write_scheduler = create_debouncer(
  (items: FilterHistoryItem[]) => localStorage.setItem(STORAGE_KEY, JSON.stringify(items)),
  {
    timeout: 1000,
    onError: Log.error,
    onSuccess: () => Log.debug("Filter history persisted to the local storage.")
  });

function push(searchTerm: string): void {
  update(current => {
    const existing_item = current.find(item => item.value === searchTerm);

    if (existing_item) {
      existing_item.timestamp = Date.now();
    } else {
      current.push({value: searchTerm, timestamp: Date.now()});
    }

    current.sort((a, b) => b.timestamp - a.timestamp);

    if (current.length > HISTORY_LIMIT) {
      for (let i = 0; i < current.length - HISTORY_LIMIT; i++) {
        current.pop()
      }
    }

    write_scheduler.update(current);
    return current;
  });
}

function push_from_tokens(searchTokens: TokenType[]) {
  if (searchTokens.length < 1) return;

  const searchTerm = searchTokens.map(e => {
    switch (e.type) {
      case "username":
      case "domain":
      case "tag":
        return `${e.type}:${e.value}`
      case "term":
        return e.value
    }
  }).join(" ");

  push(searchTerm);
}

function clear() {
  set([]);
  write_scheduler.update([]);
}

export const filterHistoryStore = {
  clear,
  push,
  push_from_tokens,
  subscribe,
};
