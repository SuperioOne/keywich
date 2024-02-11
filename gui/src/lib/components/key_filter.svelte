<script lang="ts">
  import FilterIcon from "../icons/filter.svelte";
  import XIcon from "../icons/x.svelte";
  import type {TokenType} from "../utils";
  import {computePosition, offset} from "@floating-ui/dom";
  import {createEventDispatcher, tick} from "svelte";
  import {fly} from "svelte/transition";
  import {filterHistoryStore, i18nStore} from "../stores";
  import {tokenize_filter_query} from "../utils";

  export let tokens: TokenType[] = [];

  const dispatcher = createEventDispatcher<{ search: TokenType[]; }>();
  $: search_options = [
    {
      value: "username:",
      desc: $i18nStore.get_key("i18:/filter/by-username-desc", "Filter by username")
    },
    {
      value: "domain:",
      desc: $i18nStore.get_key("i18:/filter/by-domain-desc", "Filter by domain")
    },
    {
      value: "tag:",
      desc: $i18nStore.get_key("i18:/filter/by-tag-desc", "Filter by tag")
    }];

  let input_element: HTMLElement;
  let input_container: HTMLElement;
  let menu_element: HTMLElement;
  let is_focused: boolean = false;

  $:{
    if (input_element && tokens.length > 0) {
      input_element.innerHTML = tokens_to_html(tokens);

      if (document.activeElement === input_element) {
        set_caret_to_end(input_element);
      }
    }
  }

  async function focus_input() {
    is_focused = true;
    await tick();

    if (input_element) {
      input_element.focus();

      if (menu_element && input_container) {
        const container_width = getComputedStyle(input_container).width;
        const {x, y} = await computePosition(input_container, menu_element, {
          placement: "bottom-start",
          middleware: [offset({mainAxis: 5})]
        })

        Object.assign(menu_element.style, {
          left: `${x}px`,
          top: `${y}px`,
          width: container_width
        });
      }
    }
  }

  function on_input(event: InputEvent) {
    if (event.data === " ")
      return;

    tokens = tokenize_filter_query(input_element.textContent ?? "");
  }

  function on_keyboard_event(event: KeyboardEvent) {
    switch (event.key) {
      case "Enter": {
        event.preventDefault();
        dispatcher("search", tokens);
        filterHistoryStore.push_from_tokens(tokens);
        break;
      }
      case "Escape": {
        event.preventDefault();
        is_focused = false;
        break;
      }
    }
  }

  function on_clear() {
    tokens = [];
    dispatcher("search", tokens);
    is_focused = false;
  }

  function on_option_select(value: string, override: boolean = false) {
    if (input_element) {
      const parsed_tokens = tokenize_filter_query(value);

      if (override) {
        tokens = parsed_tokens;
      } else {
        tokens.push(...parsed_tokens);
        tokens = tokens;
      }

      input_element.focus();
    }
  }

  function on_window_click(event: Event) {
    if (!is_focused) {
      return;
    }

    const is_in_container = input_container && input_container.contains(event.target as Node);
    const is_in_menu = menu_element && menu_element.contains(event.target as Node);

    if (!is_in_container && !is_in_menu) {
      is_focused = false;
    }
  }

  function tokens_to_html(filter_tokens: TokenType[]) {
    return filter_tokens
      .map(e => {
        if (e.type === "term") {
          return `${e.value}`
        } else {
          return `<span class="p-1.5 text-sm font-bold rounded-md bg-surface-active-token ml-1">
                    ${e.type}
                  <span class="font-normal">:${e.value}</span></span>`;
        }
      })
      .join(" ");
  }

  function set_caret_to_end(target: Node) {
    const selection = window.getSelection();

    if (selection && target.lastChild && selection.rangeCount > 0) {
      const range = selection?.getRangeAt(0);

      if (range) {
        range.setStartAfter(target.lastChild);
        selection.removeAllRanges();
        selection?.addRange(range);
      }
    }
  }
</script>

<svelte:window on:click={on_window_click}/>
<div class="flex flex-row w-full input overflow-hidden">
  {#if is_focused || tokens.length > 0}
    <div
        class="flex flex-row justify-between"
        bind:this={input_container}
    >
      <span class="input-group-shim flex flex-row items-center p-3">
        <FilterIcon size={18}/>
      </span>
      <div
          role="search"
          bind:this={input_element}
          class="self-center inline whitespace-nowrap py-1 px-2 w-max-[300px] w-[300px] leading-normal overflow-hidden focus-visible:outline-none"
          contenteditable="true"
          on:input={on_input}
          on:keydown={on_keyboard_event}
      />
      <button class="variant-soft-surface px-3" on:click={on_clear}>
        <XIcon size={18}/>
      </button>
    </div>
  {:else }
    <button
        class="btn variant-soft-surface btn-md"
        on:click|stopPropagation|preventDefault={focus_input}
    >
      <slot>
        <FilterIcon size={18}/>
        <span>Filter</span>
      </slot>
    </button>
  {/if}
</div>

{#if is_focused}
  <div
      transition:fly={{duration: 100,y: -10, x: 0}}
      bind:this={menu_element}
      class="absolute top-0 left-0 card px-4 py-5 bg-surface-200-700-token flex flex-col gap-2 z-50"
  >
    <div>
      <span class="font-bold">{$i18nStore.get_key("i18:/filter/options", "Filter options:")}</span>
      <ul class="p-1">
        {#each search_options as option (option)}
          <li>
            <button
                class="btn bg-initial text-sm p-2 rounded-sm w-full hover:bg-surface-backdrop-token justify-start truncate"
                on:click={() => on_option_select(option.value)}
            >
                <span class="font-bold">
                 {option.value}
                </span>
              <span class="font-light italic">
                  {option.desc}
                </span>
            </button>
          </li>
        {/each}
      </ul>
    </div>
    {#if $filterHistoryStore.length > 0}
      <hr class="!border-t-2"/>
      <div>
        <div class="flex flex-row justify-between items-center">
          <span class="font-bold">{$i18nStore.get_key("i18:/filter/history", "History")}</span>
          <button
              class="btn btn-sm text-error-400-500-token text-sm"
              on:click={() => filterHistoryStore.clear()}
          >
            {$i18nStore.get_key("i18:/generic/clear", "Clear")}
          </button>
        </div>
        <ul class="p-1 max-h-72 overflow-auto">
          {#each $filterHistoryStore as historyItem (historyItem.timestamp)}
            <li>
              <button
                  class="btn bg-initial text-sm p-2 rounded-sm w-full hover:bg-surface-backdrop-token justify-start truncate"
                  on:click={() => on_option_select(historyItem.value, true)}
              >
                {historyItem.value}
              </button>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
{/if}