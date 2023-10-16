<script lang="ts">
  import {ApplicationLogReader, Log, LogLevel} from "$lib";
  import {onMount, tick} from "svelte";
  import {CloseIcon, TrashIcon} from "$lib/icons"
  import {createEventDispatcher} from "svelte";

  const dispatch = createEventDispatcher();
  export let maxHeight = 500
  export let minHeight = 200;
  export let title = "Logs"

  let containerElement: HTMLDivElement;
  let topBarElement: HTMLDivElement;
  let autoScroll: boolean = true;
  let height = minHeight;
  let dragEnabled = false;

  onMount(() => {
    const unsubscribe = ApplicationLogReader.subscribe(() => {
      tick().then(() => {
        if (autoScroll) {
          containerElement.scroll({
            top: containerElement.scrollHeight,
          })
        }
      });
    });

    return () => {
      unsubscribe();
    };
  });

  function on_resize(event: MouseEvent) {
    if (dragEnabled) {
      const nextHeight = height + containerElement.offsetTop - event.clientY - topBarElement.clientHeight;

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
    switch (event.code) {
      case "End" :
        autoScroll = true;
        containerElement.scroll({top: containerElement.scrollHeight});
        break;
      case "Home" :
      case "PageDown" :
      case "PageUp" :
      case "ArrowUp" :
      case "ArrowDown" :
        autoScroll = false;
        break;
      default:
        break;
    }
  }

  function on_clear_logs() {
    ApplicationLogReader.reset();
  }

  function on_close() {
    dispatch("close");
  }
</script>

<svelte:window
  on:keyup|stopPropagation|preventDefault={on_key_controls}
  on:mouseup={() => {dragEnabled = false}}
  on:mousemove={on_resize}
/>
<div
  on:mousedown|stopPropagation|preventDefault={() => {dragEnabled = true}}
  on:mouseup|stopPropagation|preventDefault={() => {dragEnabled = false}}
  class="bg-surface-active-token w-full h-0.5 cursor-row-resize"
/>
<div
  bind:this={topBarElement}
  class="py-1 px-3 grid grid-cols-2 w-full bg-surface-200-700-token"
>
  <div>
    <h5 class="h5 font-bold">
    {title}
    </h5>
  </div>
  <div class="w-full flex flex-row justify-end gap-2">
    <button
      on:click={on_clear_logs}
      type="button"
      class="btn-icon-sm btn-icon variant-soft"
    >
      <TrashIcon/>
    </button>
    <button
      on:click={on_close}
      type="button"
      class="btn-icon-sm btn-icon variant-soft"
    >
      <CloseIcon/>
    </button>
  </div>
</div>
<div
  class="w-full h-full overflow-y-auto break-words p-2 bg-black"
  bind:this={containerElement}
  style:height={`${height}px`}
>
  <ol class="font-mono text-sm ">
    {#each $ApplicationLogReader as log}
      {@const date = new Date(log.timestamp)}
      <li>
        <span class="mr-0.5">{ `${date.toLocaleDateString()} ${date.toLocaleTimeString()}:`}</span>
        <span
          class:text-error-300-600-token={log.level ===  LogLevel.ERROR}
          class:text-warning-300-600-token={log.level === LogLevel.WARN}
          class:text-tertiary-300-600-token={log.level === LogLevel.DEBUG}
          class:text-primary-300-600-token={log.level >= LogLevel.INFO}
        >
            {log.message} 
        </span>
      </li>
    {/each}
  </ol>
</div>