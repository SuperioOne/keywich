import {writable} from "svelte/store";

function create_filter_history() {
  // use env variable if needed
  const limit = 25;

  const {
    update, set, subscribe
  } = writable<string[]>([]);

  return {
    set,
    push: (searchTerm: string): void => {
      update(current => {
        current.push(searchTerm);

        if (current.length > limit) {
          current.splice(0, current.length - limit);
        }

        return current;
      });
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
