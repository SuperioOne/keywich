<script lang="ts">
  import "../app.css";
  import "../app.postcss";
  import MenuItems from "./menu.config";
  import {
    computePosition,
    autoUpdate,
    flip,
    shift,
    offset,
    arrow,
  } from "@floating-ui/dom";
  import {AppShell, storePopup, initializeStores, Modal, Toast} from "@skeletonlabs/skeleton";
  import {LogPanel} from "$lib";

  initializeStores();
  let activePage: number = 0;
  let displayLogger: boolean = false;
  storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});

  function on_log_panel_close() {
    displayLogger = false;
  }

  function on_log_panel_flip(event: KeyboardEvent) {
    console.debug(event)
    if (event.code === "KeyL" && event.ctrlKey) {
      displayLogger = !displayLogger;
    }
  }
</script>

<svelte:window on:keyup={on_log_panel_flip}/>
<Modal buttonNeutral="variant-soft"  buttonPositive="variant-soft-primary" />
<Toast/>
<AppShell class="h-full">
  <svelte:fragment slot="header">
    <div class="grid grid-flow-col grid-cols-8 bg-surface-200-700-token">
      <div/>
      <div class="col-span-6 flex flex-row justify-center">
        {#each MenuItems as item, index (index)}
          <a
            href={item.target}
            class:bg-initial={true}
            class="btn bg-surface-active-token bg-primary-hover-token rounded-none sm:w-36 w-fit"
            class:bg-surface-active-token={activePage === index}
            data-sveltekit-preload-data="hover"
            on:click={() => (activePage = index)}
          >
            <div class="flex flex-col gap-1 align-middle justify-center w-full">
              <div class="flex justify-center">
                <svelte:component this={item.icon}/>
              </div>
              <div class="hidden sm:flex justify-center">
                <span class="font-bold">
                  {item.label}
                </span>
              </div>
            </div>
          </a>
        {/each}
      </div>
      <div/>
    </div>
  </svelte:fragment>
  <div class="flex justify-center w-full">
    <div class="py-10 px-3 w-full max-w-screen-lg">
      <slot/>
    </div>
  </div>
  <svelte:fragment slot="footer">
    {#if displayLogger}
      <LogPanel on:close={on_log_panel_close}/>
    {/if}
  </svelte:fragment>
</AppShell>

