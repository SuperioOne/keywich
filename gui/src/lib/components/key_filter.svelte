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
  const searchOptions = [
    {
      value: "username:",
      description: i18nStore.get_key("i18:/filter/by-username-desc", "Filter by username")
    },
    {
      value: "domain:",
      description: i18nStore.get_key("i18:/filter/by-domain-desc", "Filter by domain")
    },
    {
      value: "tag:",
      description: i18nStore.get_key("i18:/filter/by-tag-desc", "Filter by tag")
    }];

  let inputElement: HTMLElement;
  let inputContainerElement: HTMLElement;
  let menuElement: HTMLElement;
  let isFocused: boolean = false;

  $:{
    if (inputElement && tokens.length > 0) {
      inputElement.innerHTML = tokens_to_html(tokens);

      if (document.activeElement === inputElement) {
        set_caret_to_end(inputElement);
      }
    }
  }

  async function focus_input() {
    isFocused = true;
    await tick();

    if (inputElement) {
      inputElement.focus();

      if (menuElement && inputContainerElement) {
        const containerWidth = getComputedStyle(inputContainerElement).width;
        const {x, y} = await computePosition(inputContainerElement, menuElement, {
          placement: "bottom-start",
          middleware: [offset({mainAxis: 5})]
        })

        Object.assign(menuElement.style, {
          left: `${x}px`,
          top: `${y}px`,
          width: containerWidth
        });
      }
    }
  }

  function on_input(event: InputEvent) {
    if (event.data === " ")
      return;

    tokens = tokenize_filter_query(inputElement.textContent ?? "");
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
        isFocused = false;
        break;
      }
    }
  }

  function on_clear() {
    tokens = [];
    dispatcher("search", tokens);
    isFocused = false;
  }

  function on_option_select(value: string, override: boolean = false) {
    if (inputElement) {
      const parsedTokens = tokenize_filter_query(value);

      if (override) {
        tokens = parsedTokens;
      } else {
        tokens.push(...parsedTokens);
        tokens = tokens;
      }

      inputElement.focus();
    }
  }

  function on_window_click(event: Event) {
    if (!isFocused) {
      return;
    }

    const targetIsInInputContainer = inputContainerElement && inputContainerElement.contains(event.target as Node);
    const targetIsInMenu = menuElement && menuElement.contains(event.target as Node);

    if (!targetIsInInputContainer && !targetIsInMenu) {
      isFocused = false;
    }
  }

  function tokens_to_html(filterTokens: TokenType[]) {
    return filterTokens
      .map(e => {
        if (e.type === "term") {
          return `${e.value}`
        } else {
          return `<span class="p-1.5 text-sm font-bold rounded-md bg-surface-active-token ml-1">${e.type}<span class="font-normal">:${e.value}</span></span>`;
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
  {#if isFocused || tokens.length > 0}
    <div
      class="flex flex-row justify-between"
      bind:this={inputContainerElement}
    >
      <span class="input-group-shim flex flex-row items-center p-3">
        <FilterIcon size={18}/>
      </span>
      <div
        role="search"
        bind:this={inputElement}
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

{#if isFocused}
  <div
    transition:fly={{duration: 100,y: -10, x: 0}}
    bind:this={menuElement}
    class="absolute top-0 left-0 card px-4 py-5 bg-surface-200-700-token flex flex-col gap-2 z-50"
  >
    <div>
      <span class="font-bold">{i18nStore.get_key("i18:/filter/options", "Filter options:")}</span>
      <ul class="p-1">
        {#each searchOptions as option (option)}
          <li>
            <button
              class="btn bg-initial text-sm p-2 rounded-sm w-full hover:bg-surface-backdrop-token justify-start truncate"
              on:click={() => on_option_select(option.value)}
            >
                <span class="font-bold">
                 {option.value}
                </span>
              <span class="font-light italic">
                  {option.description}
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
          <span class="font-bold">{i18nStore.get_key("i18:/filter/history", "History")}</span>
          <button
            class="btn btn-sm text-error-400-500-token text-sm"
            on:click={() => filterHistoryStore.clear()}
          >
            {i18nStore.get_key("i18:/generic/clear", "Clear")}
          </button>
        </div>
        <ul class="p-1">
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