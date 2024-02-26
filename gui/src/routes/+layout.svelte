<script lang="ts">
  import "../app.css";
  import "../app.postcss";
  import {initializeStores, Modal, storePopup, Toast} from "@skeletonlabs/skeleton";
  import {arrow, autoUpdate, computePosition, flip, offset, shift} from "@floating-ui/dom";

  initializeStores();
  storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});
</script>

<svelte:window on:contextmenu|preventDefault={() => {}}/>
<slot/>
<Modal regionBackdrop="backdrop-blur-sm" zIndex="z-[998]" buttonNeutral="variant-soft"
       buttonPositive="variant-filled-primary"/>
<Toast zIndex="z-[999]"/>