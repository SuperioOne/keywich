<script lang="ts">
  import PlusCircleIcon from "$lib/icons/plus-circle.svelte";
  import TrashIcon from "$lib/icons/trash-2.svelte";
  import type {CharsetItem} from "@keywich/rpc";
  import {App, i18nStore} from "$lib";
  import {fly} from "svelte/transition";
  import {invalidateAll} from "$app/navigation";

  export let charsets: CharsetItem[];

  async function delete_charset(charsetItem: CharsetItem) {
    const success = await App.Actions.delete_charset(charsetItem)
    if (success) {
      await invalidateAll();
    }
  }

  async function create_charset() {
    const charset = await App.Actions.create_charset();
    if (charset) {
      await invalidateAll();
    }
  }

</script>

<div class="flex flex-col gap-2">
  <div>
    <button
        on:click={create_charset}
        type="button"
        class="btn variant-filled-primary w-full sm:w-auto"
    >
      <PlusCircleIcon/>
      <span class="font-bold"> {i18nStore.getKey("i18:/settings/charsets/create", "Create")} </span>
    </button>
  </div>
  <ul>
    {#if charsets.length < 1 }
      <li class="text-center w-full text-xl font-light">
        {i18nStore.getKey("i18:/settings/charsets/empty-list", "Empty charset list")}
      </li>
    {:else}
      {#each charsets as charset (charset.id)}
        <li class="py-4 flex flex-row justify-between items-center" transition:fly={{duration:200, y:20}}>
          <dl>
            <dt>
              <p class="text-secondary-400-500-token">
                <strong class="text-lg">
                  {charset.name}
                </strong>
              </p>
              <p class="text-tertiary-400-500-token">
                <small>
                  [{charset.charset}]
                </small>
              </p>
            </dt>
            <dd class="font-light">
              <small>
                {charset.description}
              </small>
            </dd>
          </dl>
          <button
              class="btn btn-sm variant-filled-error btn-icon-base h-fit"
              on:click={() => delete_charset(charset)}
          >
            <TrashIcon size={23}/>
          </button>
        </li>
        <hr>
      {/each}
    {/if}
  </ul>

</div>