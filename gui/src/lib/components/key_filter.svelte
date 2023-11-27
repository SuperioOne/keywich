<script lang="ts">
  import FilterIcon from "$lib/icons/filter.svelte";
  import XIcon from "$lib/icons/x.svelte";
  import {computePosition, offset} from "@floating-ui/dom";
  import {createEventDispatcher, tick} from "svelte";
  import {get_filter_history} from "$lib/stores";
  import {is_null_or_empty} from "$lib/utils";
  import {Log} from "$lib/logger";
  
  export let searchContent: string | null = null;

  const id = crypto.randomUUID();
  const dispatcher = createEventDispatcher<{ search: string | null; }>();
  const filterHistory = get_filter_history();
  const searchOptions = [
    {
      value: "username:",
      description: "Filter by username"
    },
    {
      value: "domain:",
      description: "Filter by domain"
    },
    {
      value: "tag:",
      description: "Filter by tag"
    }];

  let inputElement: HTMLInputElement;
  let inputContainerElement: HTMLLabelElement;
  let menuElement: HTMLElement;
  let isFocused: boolean = false;

  async function focus_input() {
    isFocused = true;
    await tick();

    if (inputElement) {
      inputElement.focus();

      if (searchContent === null || searchContent.length === 0 && menuElement && inputContainerElement) {
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

  function trigger_update(value: string | null) {
    Log.debug(value);
    if (value !== searchContent) {
      dispatcher("search", value);
    }

    searchContent = value;
  }

  function on_search(event: Event) {
    const target = event.target as HTMLInputElement;
    trigger_update(target.value);
  }

  function on_clear() {
    trigger_update(null);
    isFocused = false;
  }

  function on_option_select(value: string | null) {
    searchContent = value;
    inputElement?.focus();
  }
</script>

<div class="flex flex-row w-full">
  {#if isFocused}
    <label
      class="input-group input-group-divider flex flex-row justify-between"
      for={id}
      bind:this={inputContainerElement}
    >
      <span class="input-group-shim flex flex-row items-center p-2">
        <FilterIcon size={18}/>
      </span>
      <input
        bind:this={inputElement}
        bind:value={searchContent}
        class="w-full"
        id={id}
        on:change
        on:input
        placeholder="Try tag: mysql user: admin"
        type="text"
      />
      <button class="variant-soft-surface" on:click={on_clear}>
        <XIcon size={18}/>
      </button>
    </label>

    <div
      bind:this={menuElement}
      class:hidden={!is_null_or_empty(searchContent)}
      class="absolute top-0 left-0 card px-4 py-5 bg-surface-200-700-token flex flex-col gap-2"
    >
      <div>
        <span class="font-bold"> Filter options:</span>
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
      {#if filterHistory && $filterHistory.length > 0}
        <hr class="!border-t-2"/>
        <div>
          <div class="flex flex-row justify-between items-center">
            <span class="font-bold"> History</span>
            <button
              class="btn btn-sm text-error-400-500-token text-sm"
              on:click={() => filterHistory.clear()}
            >
              Clear
            </button>
          </div>
          <ul class="p-1">
            {#each $filterHistory as historyItem (historyItem)}
              <li>
                <button
                  class="btn bg-initial text-sm p-2 rounded-sm w-full hover:bg-surface-backdrop-token justify-start truncate"
                  on:click={() => on_option_select(historyItem)}
                >
                  {historyItem}
                </button>
              </li>
            {/each}
          </ul>
        </div>
      {/if}
    </div>
  {:else }
    <button
      class="btn variant-ghost-surface btn-md"
      on:click={focus_input}
    >
      <FilterIcon size={18}/>
      <span>Filter</span>
    </button>
  {/if}
</div>
