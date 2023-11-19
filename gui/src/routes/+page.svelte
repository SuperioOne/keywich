<script lang="ts">
  import {PinItem} from "$lib";
  import type {PageData} from "./$types";

  export let data: PageData;
</script>

<div class="flex flex-col gap-7 justify-center">
  <div>
    <h4 class="font-bold mb-5 text-xl">Pinned Keys</h4>
    <div
      class="grid place-items-stretch grid-cols-2 sm:grid-cols-3 md:grid-cols-5 lg:grid-cols-6 gap-2"
    >
      {#each data.pinnedItems as row (row.id)}
        <PinItem item={row}/>
      {/each}
    </div>
  </div>
</div>
