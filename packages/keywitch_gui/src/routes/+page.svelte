<script lang="ts">
  import {KeyIcon} from "$lib/icons";
  import type {PageData} from "./$types";

  
  export let data: PageData;
</script>

<div class="flex flex-col gap-7 justify-center">
  <div class="flex flex-col gap-4 justify-center">
    <h4 class="font-bold">Pinned Keys</h4>
    <div
      class="logo-cloud grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-6 2xl:grid-cols-6 gap-0.5"
    >
      {#each data.pinnedItems as item (item.id)}
        <a href="#" class="logo-item variant-glass-secondary">
          <div
            class="flex flex-col gap-2 justify-center items-center overflow-hidden"
          >
            <div class="text-primary-500">
              <KeyIcon width="auto" height="2rem"/>
            </div>
            <span> {item.user_name} </span>
            <span class="font-light text-sm truncate w-[90%]">
            {item.domain}
          </span>
          </div>
        </a>
      {/each}
    </div>
  </div>
</div>
