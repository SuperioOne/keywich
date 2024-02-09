<script lang="ts">
  import KeyIcon from "./key_icon.svelte";
  import type {KeyItem} from "@keywich/api";
  import {App} from "../app";

  export let item: KeyItem;
</script>

<div
    class="block card card-hover overflow-hidden">
  <a href="#" class="inline-block variant-glass-secondary p-5 w-full h-full"
     on:click|preventDefault={() => App.Actions.quick_copy(item)}>
    <div
        class="flex flex-col gap-2 justify-center items-center overflow-hidden"
    >
      <div
          class="text-primary-500 p-0 btn btn-icon-xl w-16 h-16 overflow-hidden"
      >
        <KeyIcon icon={item.custom_icon} size={34}/>
      </div>
      <div class="font-bold truncate w-[90%] text-center"> {item.username} </div>
      <div class="font-light text-xs truncate w-[90%] text-center">
        {item.domain}
      </div>
    </div>
  </a>
</div>