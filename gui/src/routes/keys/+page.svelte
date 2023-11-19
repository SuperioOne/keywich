<script lang="ts">
  import ChevronsRightIcon from "$lib/icons/chevrons-right.svelte";
  import FilterIcon from "$lib/icons/filter.svelte";
  import PlusCircleIcon from "$lib/icons/plus-circle.svelte";
  import ZapIcon from "$lib/icons/zap.svelte";
  import type {KeyMetadataItem} from "@keywitch/rpc";
  import type {PageData} from "./$types";
  import type {PopupSettings} from "@skeletonlabs/skeleton";
  import {KeyRow, get_app_context} from "$lib";
  import {invalidateAll} from "$app/navigation";
  import {popup} from "@skeletonlabs/skeleton";

  export let data: PageData;
  const appContext = get_app_context();

  let selected: number | undefined = undefined;

  async function new_key() {
    const key = await appContext.AppEvents.new_key();
    if (key) {
      await invalidateAll();
    }
  }

  async function update_key(event: CustomEvent<KeyMetadataItem>) {
    const updated = await appContext.AppEvents.update_key(event.detail);
    if (updated) {
      await invalidateAll();
    }
  }

  async function flip_pin(event: CustomEvent<KeyMetadataItem>) {
    const success = await appContext.AppEvents.flip_pin(event.detail);
    if (success) {
      await invalidateAll();
    }
  }

  async function quick_copy(event: CustomEvent<KeyMetadataItem>) {
    return await appContext.AppEvents.quick_copy(event.detail);
  }

  async function advanced_copy(event: CustomEvent<KeyMetadataItem>) {
    return await appContext.AppEvents.advanced_copy(event.detail);
  }

  async function delete_key(event: CustomEvent<KeyMetadataItem>) {
    const success = await appContext.AppEvents.delete_key(event.detail);
    if (success) {
      await invalidateAll();
    }
  }

  const filterPopup: PopupSettings = {
    event: 'click',
    target: 'filter_popup',
    placement: 'bottom',
  };

  const sortPopup: PopupSettings = {
    event: 'click',
    target: 'sort_popup',
    placement: 'bottom',
  };
</script>

<div class="flex gap-6 flex-col">
  <div class="grid grid-cols-2 gap-6">
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2">
      <button
        on:click={new_key}
        type="button"
        class="btn variant-soft-primary w-full sm:w-auto"
      >
        <PlusCircleIcon/>
        <span class="font-bold"> Create </span>
      </button>
    </div>
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2 justify-end">
      <button
        type="button"
        class="btn variant-soft w-full sm:w-auto"
        use:popup={sortPopup}
      >
        <ZapIcon size={16}/>
        <span class="font-bold"> Sort </span>
      </button>
      <button
        type="button"
        class="btn variant-soft w-full sm:w-auto"
        use:popup={filterPopup}
      >
        <FilterIcon size={16}/>
        <span class="font-bold"> Filter </span>
      </button>
    </div>
  </div>

  <dl class="flex flex-col gap-1">
    {#each data.keys as row,index (row.id)}
      <div
        class="flex flex-row gap-1"
      >
        <div class="w-4 flex flex-col justify-center items-start">
          {#if index === selected}
            <div class="text-sm font-bold text-on-secondary-token">
              {index + 1}
            </div>
          {:else}
            <span class="text-sm font-bold text-secondary-100-800-token select-none">
              {index + 1}
            </span>
          {/if}
        </div>
        <KeyRow
          item={row}
          active={index === selected}
          on:copy={quick_copy}
          on:copyAux={quick_copy}
          on:copyAlt={advanced_copy}
          on:delete={delete_key}
          on:update={update_key}
          on:pin={flip_pin}
        />
      </div>
    {/each}
  </dl>
</div>

<div class="card p-5 m-w-md shadow-xl rounded-md" data-popup="sort_popup">
  <div><p>Sort Content</p></div>
  <div class="arrow bg-surface-100-800-token"/>
</div>

<div class="card p-5 max-w-md shadow-xl rounded-md" data-popup="filter_popup">
  <div><p>Filter Content</p></div>
  <div class="arrow bg-surface-100-800-token"/>
</div>
