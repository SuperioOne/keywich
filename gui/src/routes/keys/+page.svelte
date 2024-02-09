<script lang="ts">
  import FilterIcon from "$lib/icons/filter.svelte";
  import PlusCircleIcon from "$lib/icons/plus-circle.svelte";
  import type {KeyItem} from "@keywich/api";
  import type {PageData} from "./$types";
  import type {TokenType} from "$lib";
  import {KeyRow, KeyFilterInput, i18nStore, App} from "$lib";
  import {fly} from "svelte/transition";
  import {invalidateAll} from "$app/navigation";

  export let data: PageData;

  let selected: number | undefined = undefined;

  async function create_key() {
    const key = await App.Actions.create_key();
    if (key) {
      await invalidateAll();
    }
  }

  async function update_key(event: CustomEvent<KeyItem>) {
    const updated = await App.Actions.update_key(event.detail);
    if (updated) {
      await invalidateAll();
    }
  }

  async function flip_pin(event: CustomEvent<KeyItem>) {
    const success = await App.Actions.flip_pin(event.detail);
    if (success) {
      await invalidateAll();
    }
  }

  async function quick_copy(event: CustomEvent<KeyItem>) {
    return await App.Actions.quick_copy(event.detail);
  }

  async function advanced_copy(event: CustomEvent<KeyItem>) {
    return await App.Actions.advanced_copy(event.detail);
  }

  async function delete_key(event: CustomEvent<KeyItem>) {
    const success = await App.Actions.delete_key(event.detail);
    if (success) {
      await invalidateAll();
    }
  }

  async function on_tag(event: CustomEvent<string>) {
    const searchTokens: TokenType[] = [{type: "tag", value: event.detail}];
    await App.Actions.search_keys(searchTokens);
  }

  async function search_keys(event: CustomEvent<TokenType[]>) {
    await App.Actions.search_keys(event.detail ?? []);
  }
</script>

<div class="flex gap-6 flex-col">
  <div class="grid grid-cols-2 gap-6">
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2">
      <button
          on:click={create_key}
          type="button"
          class="btn variant-filled-primary w-full sm:w-auto"
      >
        <PlusCircleIcon/>
        <span class="font-bold"> {i18nStore.get_key("i18:/keys/button/create", "Create")} </span>
      </button>
    </div>
    <div class="col-span-full sm:col-span-1 flex flex-row flex-wrap gap-2 justify-end">
      <div class="w-full sm:w-fit">
        <KeyFilterInput
            on:search={search_keys}
            tokens={data.tokens ?? []}
        >
          <FilterIcon size={18}/>
          <span>
            {i18nStore.get_key("i18:/keys/button/filter", "Filter")}
          </span>
        </KeyFilterInput>
      </div>
    </div>
  </div>

  {#if data.keys.length < 1}
    <p class="text-center w-full font-light text-xl py-6">
      {i18nStore.get_key("i18:/keys/empty-list", "Empty list")}
    </p>
  {:else }
    <div class="flex flex-col gap-1">
      {#each data.keys as row,index (row.id)}
        <div class="w-full" transition:fly={{duration:200, y:20}}>
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
    </div>
  {/if}
</div>
