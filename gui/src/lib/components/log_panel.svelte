<script lang="ts">
  import {ApplicationLogReader, LogLevel, type LogLevelType} from "$lib/logger";
  import {onMount, tick, createEventDispatcher} from "svelte";
  import CloseIcon from "$lib/icons/x.svelte";
  import TrashIcon from "$lib/icons/trash-2.svelte";
  import TerminalIcon from "$lib/icons/terminal.svelte";
  import {fly} from "svelte/transition";
  import {i18nStore} from "$lib/stores/i18n_store";

  const dispatch = createEventDispatcher();
  export let maxHeight = 500
  export let minHeight = 200;
  $: title = $i18nStore.get_key("i18:/log-panel/title", "Logs");

  let container_element: HTMLDivElement;
  let top_bar_element: HTMLDivElement;
  let is_auto_scroll: boolean = true;
  let height = minHeight;
  let drag_enabled = false;

  onMount(() => {
    container_element.focus();

    const unsubscribe = ApplicationLogReader.subscribe(() => {
      tick().then(() => {
        if (is_auto_scroll) {
          container_element.scroll({
            top: container_element.scrollHeight,
          })
        }
      });
    });

    return () => {
      unsubscribe();
    };
  });

  function on_resize(event: MouseEvent) {
    if (drag_enabled) {
      const nextHeight = height + container_element.offsetTop - event.clientY - top_bar_element.clientHeight;

      if (nextHeight > maxHeight) {
        height = maxHeight;
      } else if (nextHeight < minHeight) {
        height = minHeight;
      } else {
        height = nextHeight;
      }
    }
  }

  function on_key_controls(event: KeyboardEvent) {
    if (document.activeElement === container_element || container_element.contains(document.activeElement as Node)) {
      switch (event.code) {
        case "End" :
          is_auto_scroll = true;
          container_element.scroll({top: container_element.scrollHeight});
          break;
        case "Home" :
        case "PageDown" :
        case "PageUp" :
        case "ArrowUp" :
        case "ArrowDown" :
          is_auto_scroll = false;
          break;
        default:
          break;
      }
    }
  }

  function on_clear_logs() {
    ApplicationLogReader.reset();
  }

  function on_close() {
    dispatch("close");
  }

  function get_class_name(level: LogLevelType) {
    switch (level) {
      case LogLevel.DEBUG:
        return "text-secondary-300-600-token";
      case LogLevel.INFO:
        break;
      case LogLevel.ERROR:
        return "text-error-300-600-token";
      case LogLevel.WARN:
        return "text-warning-300-600-token";
      case LogLevel.TRACE:
        return "text-tertiary-300-600-token";
    }
  }

  function get_level_name(level: LogLevelType) {
    switch (level) {
      case LogLevel.DEBUG:
        return "DEBUG";
      case LogLevel.INFO:
        return "INFO"
      case LogLevel.ERROR:
        return "ERROR";
      case LogLevel.WARN:
        return "WARN";
      case LogLevel.TRACE:
        return "TRACE";
    }
  }
</script>

<svelte:window
    on:mouseup={() => {drag_enabled = false}}
    on:mousemove={on_resize}
    on:keyup={on_key_controls}
/>
<div transition:fly={{duration:100, y:500 }}>
  <div
      on:mousedown|stopPropagation|preventDefault={() => {drag_enabled = true}}
      on:mouseup|stopPropagation|preventDefault={() => {drag_enabled = false}}
      class="bg-surface-active-token w-full h-0.5 cursor-row-resize"
  />
  <div
      bind:this={top_bar_element}
      class="py-1 px-3 grid grid-cols-2 w-full bg-surface-200-700-token"
  >
    <div class="flex flex-row gap-2 items-center">
      <div>
        <TerminalIcon size={20}/>
      </div>
      <h5 class="h5 font-bold">
        {title}
      </h5>
    </div>
    <div class="w-full flex flex-row justify-end gap-2">
      <button
          on:click={on_clear_logs}
          type="button"
          class="btn-icon-sm btn-icon variant-soft-error"
      >
        <TrashIcon size={18}/>
      </button>
      <button
          on:click={on_close}
          type="button"
          class="btn-icon-sm btn-icon variant-soft"
      >
        <CloseIcon size={18}/>
      </button>
    </div>
  </div>
  <div
      transition:fly={{duration:150}}
      class="w-full h-full overflow-y-auto break-words p-2 bg-black"
      bind:this={container_element}
      style:height={`${height}px`}
      tabindex="-1"
  >
    <ol class="font-mono text-sm">
      {#each $ApplicationLogReader as log}
        {@const date = new Date(log.timestamp)}
        {@const className = get_class_name(log.level)}
        <li class={className}>
          <span class="mr-2"> {date.toISOString()}</span>
          <span class="inline-block w-12 mr-0.5"> {get_level_name(log.level)}</span>
          <span>{log.message}</span>
        </li>
      {/each}
    </ol>
  </div>
</div>
