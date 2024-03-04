<script lang="ts">
  import "../app.css";
  import "../app.postcss";
  import {initializeStores, Modal, storePopup, Toast} from "@skeletonlabs/skeleton";
  import {arrow, autoUpdate, computePosition, flip, offset, shift} from "@floating-ui/dom";
  import {Log} from "$lib";
  import {goto} from "$app/navigation";
  import type {LayoutData} from "./$types";

  export let data: LayoutData;

  initializeStores();
  storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});
</script>

<svelte:window
    on:contextmenu|preventDefault={() => {}}
    on:keydown={(event) => {
      if(event.code === "F5" && data.path) {
        Log.debug("reloading page");
        goto(data.path).catch(Log.error);
      }
    }}
/>
<slot/>
<Modal regionBackdrop="backdrop-blur-sm" zIndex="z-[998]" buttonNeutral="variant-soft"
       buttonPositive="variant-filled-primary"/>
<Toast zIndex="z-[999]"/>