<script lang="ts">
  import ClipboardIcon from "$lib/icons/clipboard.svelte";
  import EditIcon from "$lib/icons/edit.svelte";
  import InfoIcon from "$lib/icons/info.svelte";
  import KeyIcon from "$lib/icons/key.svelte";
  import StarIcon from "$lib/icons/star.svelte";
  import TrashIcon from "$lib/icons/trash-2.svelte";
  import type {KeyItem} from "@keywich/api";
  import type {PopupSettings} from "@skeletonlabs/skeleton";
  import {createEventDispatcher} from "svelte";
  import {popup} from "@skeletonlabs/skeleton";

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

  $: noteTooltip = {
    event: "click",
    target: `note_tooltip_${item.id}`,
    placement: "bottom-start"
  } as PopupSettings;

  $: hasNote = item.notes && item.notes.trim().length > 0;
</script>

<div
  class:bg-surface-active-token={active}
  class="card grid grid-cols-5 sm:grid-cols-8 md:grid-cols-12 gap-3 w-full p-4 items-stretch"
  tabindex="-1"
>
  <div class="flex col-span-1 items-center aspect-square">
    <button
      type="button"
      class="text-primary-500 bg-surface-200-700-token btn btn-icon-xl p-1 aspect-square overflow-hidden"
      on:click|preventDefault={() => dispatch("copy", item)}
    >
      {#if item.custom_icon}
        <img width="100%" src={item.custom_icon} alt="Missing Icon"/>
      {:else}
        <KeyIcon/>
      {/if}
    </button>
  </div>

  <div class="col-span-3 sm:col-span-4 md:col-span-8 flex flex-row sm:gap-8 gap-3 items-center">
    <div class="w-full overflow-hidden flex flex-col gap-4">
      <div>
        <dt class="font-bold text-lg mb-0.5">
          <div class="flex flex-row items-center gap-1">
            <span>
              {item.username}
            </span>
            <button
              class="btn !bg-transparent !m-0 !p-0"
              class:hidden={!hasNote}
              use:popup={noteTooltip}
            >
              <InfoIcon size={18}/>
            </button>
          </div>
        </dt>
        <dd class="font-thin truncate">
          {item.domain}
        </dd>
      </div>
      <div class="flex flex-wrap gap-1 min-h-[24px]">
        {#each item.tags as tag (tag)}
          <button
            class="btn chip font-bold text-xs variant-filled-secondary w-fit px-2 py-1"
            on:click|preventDefault={() => dispatch("tagSelect", tag)}
          >
            {tag}
          </button>
        {/each}
      </div>
    </div>
  </div>
  <div
    class="col-span-1 sm:col-span-3 flex flex-col sm:flex-row gap-2 sm:gap-4 justify-end items-center"
  >
    <button
      type="button"
      on:contextmenu|preventDefault={() => dispatch("copyAlt", item)}
      on:auxclick|preventDefault={() => dispatch("copyAux", item)}
      on:click|preventDefault={() => dispatch("copy", item)}
      class="btn btn-sm variant-filled-primary btn-icon-base h-fit"
    >
      <ClipboardIcon/>
    </button>
    <button
      type="button"
      class="btn btn-sm btn-icon-base h-fit"
      class:variant-filled-warning={item.pinned}
      class:variant-soft={!item.pinned}
      on:click|preventDefault={() => dispatch("pin", item)}
    >
      <StarIcon/>
    </button>
    <button
      type="button"
      class="btn btn-sm variant-filled-secondary btn-icon-base h-fit"
      on:click|preventDefault={() => dispatch("update", item)}
    >
      <EditIcon/>
    </button>
    <button
      on:click|preventDefault={() => dispatch("delete", item)}
      type="button"
      class="btn btn-sm variant-filled-error btn-icon-base h-fit"
    >
      <TrashIcon/>
    </button>
  </div>
</div>

<div class="card p-3 variant-filled-surface rounded-md max-w-sm" data-popup={noteTooltip.target}>
  <p class="font-light">{item.notes}</p>
</div>