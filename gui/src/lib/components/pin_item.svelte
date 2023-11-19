<script lang="ts">
  import KeyIcon from "$lib/icons/key.svelte";
  import type {KeyMetadataItem} from "@keywitch/rpc";
  import {get_app_context} from "$lib";

  export let item: KeyMetadataItem;
  const appContext = get_app_context();
</script>

<div class="variant-glass-secondary rounded-md cursor-pointer hover:backdrop-brightness-75 overflow-hidden">
  <a href="#" class="inline-block variant-glass-secondary p-5 w-full h-full"
     on:click|preventDefault={() => appContext.AppEvents.quick_copy(item)}>
    <div
      class="flex flex-col gap-2 justify-center items-center overflow-hidden"
    >
      <div
        class="text-primary-500 p-0 btn btn-icon-xl w-16 h-16 overflow-hidden"
      >
        {#if item.custom_icon}
          <img width="100" src={item.custom_icon} alt="Missing Icon"/>
        {:else}
          <KeyIcon size={34}/>
        {/if}
      </div>
      <div class="font-bold truncate w-[90%] text-center"> {item.user_name} </div>
      <div class="font-light text-xs truncate w-[90%] text-center">
        {item.domain}
      </div>
    </div>
  </a>
</div>