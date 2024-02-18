<script lang="ts">
  import {RPC} from "../rpc";
  import KeyIcon from "$lib/icons/key.svelte";

  export let icon: string | undefined = undefined;
  export let size: number = 32;
</script>

{#if icon}
  <img width={size} src={RPC.convert_icon_src(icon)} alt="Missing Icon"/>
{:else}
  <KeyIcon size={size}/>
{/if}