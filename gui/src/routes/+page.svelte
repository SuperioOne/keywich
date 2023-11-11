<script lang="ts">
  import KeyIcon from "$lib/icons/key.svelte";
  import type {PageData} from "./$types";
  import type {KeyMetadataItem} from "@keywitch/rpc";
  import RPC from "@keywitch/memory_rpc";
  import {getExtendedToastStore, Log} from "$lib";

  export let data: PageData;
  const toastStore = getExtendedToastStore();

  async function quick_copy(keyDetails: KeyMetadataItem) {
    try {
      const result = await RPC.KeyMetadata.generate_password(keyDetails.id, "text");
      if (result.success) {
        await navigator.clipboard.writeText(result.data);
        toastStore.trigger_success("Key copied to clipboard.");
      } else {
        Log.error(result.error);
        toastStore.trigger_error("Key generation failed. See logs for more details.");
      }
    } catch (err) {
      Log.error(err as Error);
      toastStore.trigger_error("Key generation failed. See logs for more details.");
    }
  }
</script>

<div class="flex flex-col gap-7 justify-center">
  <div>
    <h4 class="font-bold mb-5 text-xl">Pinned Keys</h4>
    <div
      class="grid place-items-stretch grid-cols-2 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-6 gap-2"
    >
      {#each data.pinnedItems as row (row.id)}
        <div class="variant-glass-secondary rounded-md cursor-pointer hover:backdrop-brightness-75 overflow-hidden">
          <a href="#" class="inline-block variant-glass-secondary p-5 w-full h-full"
             on:click|preventDefault={() => quick_copy(row)}>
            <div
              class="flex flex-col gap-2 justify-center items-center overflow-hidden"
            >
              <div
                class="flex justify-center items-center text-primary-500 overflow-hidden rounded-md h-16 w-16 select-none p-1">
                {#if row.custom_icon}
                  <img src={row.custom_icon} alt="Missing Image" width="100"/>
                {:else}
                  <KeyIcon width="auto" height="2rem"/>
                {/if}
              </div>
              <div> {row.user_name} </div>
              <div class="font-light text-sm truncate w-[90%] text-center">
                {row.domain}
              </div>
            </div>
          </a>
        </div>
      {/each}
    </div>
  </div>
</div>
