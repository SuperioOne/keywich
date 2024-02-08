<script lang="ts">
  import {RPC} from "$lib";
  import KeyIcon from "$lib/icons/key.svelte";

  export let icon: string | undefined = undefined;
  export let size: number = 32;
</script>

{#if icon}
  {#await RPC.convert_content_src(icon)}
    <KeyIcon size={size}/>
  {:then icon_url}
    <img width="auto" src={icon_url} alt="Missing Icon"/>
  {:catch _}
    <KeyIcon size={size}/>
  {/await}
{:else}
  <KeyIcon size={size}/>
{/if}