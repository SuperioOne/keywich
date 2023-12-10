<script lang="ts">
  import PlusCircleIcon from "$lib/icons/plus-circle.svelte";
  import type {KeyMetadataItem} from "@keywitch/rpc";
  import type {PageData} from "./$types";
  import {KeyRow, get_app_context, KeyFilterInput, Log, type TokenType} from "$lib";
  import {invalidateAll, goto} from "$app/navigation";
  import {fly} from "svelte/transition";

  export let data: PageData;

  let selected: number | undefined = undefined;
  const appContext = get_app_context();

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

  async function on_tag(event: CustomEvent<string>) {
    const searchTokens: TokenType[] = [{type: "tag", value: event.detail}];
    await appContext.AppEvents.search_keys(searchTokens);
  }

  async function search_keys(event: CustomEvent<TokenType[]>) {
    await appContext.AppEvents.search_keys(event.detail ?? []);
  }
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
      <div class="w-full sm:w-fit">
        <KeyFilterInput
          on:search={search_keys}
          tokens={data.tokens ?? []}
        />
      </div>
    </div>
  </div>

  <dl class="flex flex-col gap-1">
    {#each data.keys as row,index (row.id)}
      <div class="flex flex-row gap-1" transition:fly={{duration:200, y:20}}>
        <div class="w-4 hidden sm:flex flex-col justify-center items-start">
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
          on:tagSelect={on_tag}
        />
      </div>
    {/each}
  </dl>
</div>
