<script lang="ts">
  import ClipboardIcon from "$lib/icons/clipboard.svelte";
  import EditIcon from "$lib/icons/edit.svelte";
  import InfoIcon from "$lib/icons/info.svelte";
  import StarIcon from "$lib/icons/star.svelte";
  import TrashIcon from "$lib/icons/trash-2.svelte";
  import type { KeyItem } from "../api";
  import type { PopupSettings } from "@skeletonlabs/skeleton";
  import { createEventDispatcher } from "svelte";
  import { popup } from "@skeletonlabs/skeleton";
  import KeyIcon from "./key_icon.svelte";

  export let item: KeyItem;
  export let active: boolean = false;

  const dispatch = createEventDispatcher<{
    remove: KeyItem;
    copyAux: KeyItem;
    copy: KeyItem;
    copyAlt: KeyItem;
    delete: KeyItem;
    update: KeyItem;
    pin: KeyItem;
    tagSelect: string;
  }>();

  $: note_tooltip = {
    event: "click",
    target: `note_tooltip_${item.id}`,
    placement: "bottom-start",
  } as PopupSettings;

  $: has_note = item.notes && item.notes.trim().length > 0;
</script>

<div
  class:bg-surface-active-token={active}
  class="card flex justify-between gap-4 w-full p-4 items-center variant-glass-secondary"
  tabindex="-1"
>
  <div class="flex-none gap-3 items-center">
    <button
      type="button"
      class="text-primary-500 rounded-xl overflow-hidden aspect-square pin-btn"
      on:click|preventDefault={() => dispatch("copy", item)}
    >
      <KeyIcon icon={item.custom_icon} size={48} />
    </button>
  </div>

  <div class="min-w-0 flex-grow flex-shrink">
    <div class="font-bold text-lg mb-0.5 flex flex-row items-center gap-1">
      <p class="truncate max-w-[90%]">
        {item.username}
      </p>
      <button
        class="btn !bg-transparent !m-0 !p-0"
        class:hidden={!has_note}
        use:popup={note_tooltip}
      >
        <InfoIcon size={18} />
      </button>
    </div>

    <p class="font-thin truncate max-w-[90%]">
      {item.domain}
    </p>

    <div class="flex flex-wrap mt-3 gap-1 min-h-[24px]">
      {#each item.tags.sort() as tag (tag)}
        <button
          class="btn chip font-bold text-xs variant-glass-tertiary w-fit px-2 py-1"
          on:click|preventDefault={() => dispatch("tagSelect", tag)}
        >
          {tag}
        </button>
      {/each}
    </div>
  </div>

  <div
    class="flex-none flex flex-col sm:flex-row gap-2 sm:gap-4 justify-end items-center"
  >
    <button
      type="button"
      on:contextmenu|preventDefault={() => dispatch("copyAlt", item)}
      on:auxclick|preventDefault={() => dispatch("copyAux", item)}
      on:click|preventDefault={() => dispatch("copy", item)}
      class="btn btn-sm variant-glass-primary btn-icon-base h-fit"
    >
      <ClipboardIcon />
    </button>
    <button
      type="button"
      class="btn btn-sm btn-icon-base h-fit"
      class:variant-filled-warning={item.pinned}
      class:variant-glass-surface={!item.pinned}
      on:click|preventDefault={() => dispatch("pin", item)}
    >
      <StarIcon />
    </button>
    <button
      type="button"
      class="btn btn-sm variant-glass-secondary btn-icon-base h-fit"
      on:click|preventDefault={() => dispatch("update", item)}
    >
      <EditIcon />
    </button>
    <button
      on:click|preventDefault={() => dispatch("delete", item)}
      type="button"
      class="btn btn-sm variant-glass-error btn-icon-base h-fit"
    >
      <TrashIcon />
    </button>
  </div>
</div>

<div
  class="card p-3 variant-filled-surface rounded-md max-w-sm z-50"
  data-popup={note_tooltip.target}
>
  <p class="font-light">{item.notes}</p>
</div>
