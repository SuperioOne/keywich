<script lang="ts">
  import {
    ClipboardIcon,
    TrashIcon,
    KeyIcon,
    PlusCircleIcon,
    StarIcon,
    EditIcon,
  } from "$lib/icons";
  import {InputChip} from "@skeletonlabs/skeleton";
  import type {PageData} from "./$types";

  export let data: PageData;
  let searchValue: any[];
  let focused: number = -1;

  async function shortcut_handler(event: KeyboardEvent) {
    if (event.code === "KeyJ" && focused < data.passwords.length - 1) {
      focused += 1;
    } else if (event.code === "KeyK" && focused > 0) {
      focused -= 1;
    }
  }
</script>

<svelte:window on:keydown={shortcut_handler}></svelte:window>

{focused}
<div class="flex gap-6 flex-col">
  <div>
    <InputChip
      bind:value={searchValue}
      name="filter"
      chips="chip font-bold text-xs variant-soft-secondary"
      placeholder="filter..."
    />
  </div>

  <div class="flex flex-row justify-center sm:justify-end gap-1">
    <button type="button" class="btn variant-filled-primary w-full sm:w-auto"
    >
      <PlusCircleIcon/>
      <span class="font-bold"> New </span>
    </button>
  </div>

  <dl on:mouseleave={()=> {focused= -1}} class="flex flex-col gap-1">
    {#each data.passwords as row,index (row.id)}
      <div
        role="none"
        on:mouseenter={()=> {focused= row.id}}
        class:bg-surface-400-500-token={focused === index}
        class="card grid grid-cols-4 gap-3 w-full p-4">
        <div class="col-span-3 flex flex-row sm:gap-8 gap-3 items-center">
          <div
            class="text-primary-500 bg-surface-200-700-token p-3 btn-sm btn h-fit"
          >
            <KeyIcon/>
          </div>
          <div class="w-full overflow-hidden flex flex-col gap-3">
            <div>
              <dt class="font-bold">{row.user_name}</dt>
              <dd class="text-sm font-thin truncate">
                {row.domain}
              </dd>
            </div>
            <div class="flex flex-wrap gap-1">
                <span
                  class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
                >
                  Preset
                </span>
              <span
                class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
              >
                  tag 1
                </span>
              <span
                class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
              >
                  tag 2
                </span>
              <span
                class="chip font-bold text-xs variant-soft-secondary w-fit px-2 py-1"
              >
                  tag 3
                </span>
            </div>
          </div>
        </div>
        <div
          class="flex flex-col md:flex-row flex-wrap gap-1 sm:gap-2 justify-end items-center"
        >
          <button
            type="button"
            class="btn btn-sm variant-soft-primary btn-icon-base h-fit"
          >
            <ClipboardIcon/>
          </button>
          <button
            type="button"
            class="btn btn-sm  btn-icon-base h-fit"
            class:variant-ghost-warning={row.pinned}
            class:variant-soft={!row.pinned}
          >
            <StarIcon/>
          </button>
          <button
            type="button"
            class="btn btn-sm variant-soft-secondary btn-icon-base h-fit"
          >
            <EditIcon/>
          </button>
          <button
            type="button"
            class="btn btn-sm variant-soft-error btn-icon-base h-fit"
          >
            <TrashIcon/>
          </button>
        </div>
      </div>
    {/each}
  </dl>
</div>
