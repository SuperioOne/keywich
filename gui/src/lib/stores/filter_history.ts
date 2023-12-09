import {writable} from "svelte/store";
import type {TokenType} from "$lib/utils/key_filter_tokenizer";

export type FilterHistoryItem = {
  value: string;
  timestamp: number;
}

function create_filter_history() {
  // use env variable if needed
  const limit = 5;

  const {
    update, set, subscribe
  } = writable<FilterHistoryItem[]>([]);

  const push = (searchTerm: string): void => {
    update(current => {

      const existingItem = current.find(item => item.value === searchTerm);

      if (existingItem) {
        existingItem.timestamp = Date.now();
      } else {
        current.push({value: searchTerm, timestamp: Date.now()});
      }

      current.sort((a, b) => b.timestamp - a.timestamp);

      if (current.length > limit) {
        for (let i = 0; i < current.length - limit; i++) {
          current.pop()
        }
      }

      return current;
    });
  }

  return {
    set,
    push: push,
    push_from_tokens: (searchTokens: TokenType[]) => {
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
    },
    clear: () => {
      set([]);
    },
    subscribe
  }
}

const SearchHistoryStore = create_filter_history();

export function get_filter_history() {
  return SearchHistoryStore;
}
