<script lang="ts">
  import FilterIcon from "../icons/filter.svelte";
  import XIcon from "../icons/x.svelte";
  import type {TokenType} from "../utils";
  import {computePosition, offset} from "@floating-ui/dom";
  import {createEventDispatcher, tick} from "svelte";
  import {fly} from "svelte/transition";
  import {filterHistoryStore, i18nStore} from "../stores";
  import {tokenize_filter_query} from "../utils";
  import {is_null_or_empty} from "@keywich/api/utils";
  import {Log} from "$lib/logger";

  export let query: string | null;

  const dispatcher = createEventDispatcher<{ search: string | null; }>();
  const search_options = [
    {
      value: "username:",
      desc_key: "i18:/filter/by-username-desc",
      desc_default: "Filter by username"
    },
    {
      value: "domain:",
      desc_key: "i18:/filter/by-domain-desc",
      desc_default: "Filter by domain"
    },
    {
      value: "tag:",
      desc_key: "i18:/filter/by-tag-desc",
      desc_default: "Filter by tag"
    }];

  let input_element: HTMLDivElement;
  let input_container: HTMLElement;
  let menu_element: HTMLElement;
  let is_focused: boolean = false;

  $:if (input_element && !is_null_or_empty(query)) {
    const tokens = tokenize_filter_query(query);
    render_text(input_element, tokens);
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

  function on_input() {
    const query_text = input_element.textContent ?? "";
    const caret_pos = get_caret_position(input_element);
    const tokens = tokenize_filter_query(query_text);

    render_text(input_element, tokens);
    set_caret_position(input_element, caret_pos);
  }

  function on_keyboard_event(event: KeyboardEvent) {
    switch (event.key) {
      case "Enter": {
        event.preventDefault();
        const query_text = input_element.textContent ?? "";

        dispatcher("search", query_text);
        if (!is_null_or_empty(query_text)) {
          filterHistoryStore.push(query_text);
        }
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
    input_element.innerText = "";
    dispatcher("search", null);
    is_focused = false;
  }

  async function on_option_select(value: string, override: boolean = false) {
    if (input_element) {
      let query_text: string = input_element.textContent ?? "";

      if (override || is_null_or_empty(query_text)) {
        query_text = value;
      } else {
        query_text = `${query_text} ${value}`;
      }

      const tokens = tokenize_filter_query(query_text);
      render_text(input_element, tokens);
      set_caret_position(input_element, query_text.length);
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

  function render_text(target: HTMLDivElement, tokens: TokenType[]) {
    target.replaceChildren(...tokens.map(t => create_token_node(t)));
  }

  function create_token_node(token: TokenType): HTMLSpanElement {
    const node = document.createElement("span");
    node.setAttribute("token-type", token.type);

    switch (token.type) {
      case "username":
      case "domain":
      case "tag": {
        const content_span = create_content_token(token.value);
        const label_span = document.createElement("span");
        label_span.setAttribute("token-label", "");
        label_span.setAttribute("token-text", "");
        label_span.innerText = `${token.type}:`;
        label_span.className = "font-bold";
        node.className = "p-1.5 text-sm rounded-md bg-surface-active-token ml-1";
        node.append(label_span, content_span);
        break;
      }
      default: {
        const content_span = create_content_token(token.value);
        node.append(content_span);
        break;
      }
    }

    return node;
  }

  function create_content_token(value: string): Node {
    const content_span = document.createElement("span");
    content_span.setAttribute("token-value", "");
    content_span.setAttribute("token-text", "");
    content_span.innerText = value;

    return content_span;
  }

  function get_caret_position(target: HTMLElement) {
    const selection = window.getSelection();

    if (!selection || !selection.focusNode) {
      return 0;
    }

    const focus_node = selection.focusNode;

    if (target.hasChildNodes()) {
      if (target.firstChild?.nodeType === Node.TEXT_NODE) {
        return selection.focusOffset;
      }
    }

    const child_nodes = target.querySelectorAll(`[token-text=""]`);

    let pos = 0;
    for (const child of child_nodes) {
      if (child === focus_node || child.contains(focus_node)) {
        pos += selection.focusOffset;
        break;
      } else {
        pos += child.textContent?.length ?? 0;
      }
    }

    return pos;
  }

  function set_caret_position(target: HTMLElement, target_pos: number) {
    const child_nodes = target.querySelectorAll(`[token-text=""]`);
    let current_pos = 0;

    for (const child of child_nodes) {
      const child_len = (child.textContent?.length ?? 0);
      const next_pos = current_pos + child_len;

      if (next_pos >= target_pos) {
        const selection = window.getSelection();
        const offset = target_pos - current_pos;

        for (const node of child.childNodes) {
          if (node.nodeType === Node.TEXT_NODE) {
            selection?.setPosition(node, offset);
            return;
          }
        }

        Log.warn("Child doesn't have any text node.");
        selection?.setPosition(child);
        return;
      } else {
        current_pos = next_pos;
      }
    }
  }
</script>

<svelte:window on:click={on_window_click}/>
<div class="flex flex-row w-full input overflow-hidden">
  {#if is_focused || !is_null_or_empty(query)}
    <div
        class="flex flex-row justify-between"
        bind:this={input_container}
    >
      <span class="input-group-shim flex flex-row items-center p-3">
        <FilterIcon size={18}/>
      </span>
      <div
          id="wtf"
          role="search"
          bind:this={input_element}
          class="self-center inline whitespace-nowrap py-1 px-2 w-max-[300px] w-[300px] leading-normal overflow-hidden focus-visible:outline-none"
          contenteditable="true"
          spellcheck="false"
          on:input={on_input}
          on:keydown={on_keyboard_event}
      >
      </div>
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
                {$i18nStore.get_key(option.desc_key, option.desc_default)}
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