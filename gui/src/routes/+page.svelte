<script lang="ts">
  import {PinItem, i18nStore} from "$lib";
  import type {PageData} from "./$types";

  export let data: PageData;
</script>


<div class="flex flex-col gap-7 justify-center">
  <div>
    <h4 class="font-bold mb-5 text-3xl">
      {i18nStore.get_key("i18:/home/pinned-keys", "Pinned Keys")}
    </h4>
    {#if data.pinnedItems.length < 1}
      <p class="text-center w-full font-light text-xl">
        {i18nStore.get_key("i18:/home/no-pinned-key")}
      </p>
    {:else}
      <div
          class="grid place-items-stretch grid-cols-2 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-6 gap-2"
      >
        {#each data.pinnedItems as row (row.id)}
          <PinItem item={row}/>
        {/each}
      </div>
    {/if}
  </div>
</div>
