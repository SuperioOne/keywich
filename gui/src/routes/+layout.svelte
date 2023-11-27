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
  import {create_event_manager, LogPanel, set_app_context} from "$lib";
  import RPC from "@keywitch/memory_rpc";

  initializeStores();
  const eventManager = create_event_manager();
  storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});

  let activePage: number = 0;
  let displayLogger: boolean = false;

  set_app_context({
    RPC: RPC,
    AppEvents: eventManager,
  });

  function on_log_panel_close() {
    displayLogger = false;
  }

  function on_log_panel_flip(event: KeyboardEvent) {
    if (event.code === "KeyI" && event.ctrlKey) {
      displayLogger = !displayLogger;
    }
  }
</script>

<svelte:window on:keyup|preventDefault={on_log_panel_flip}/>
<Modal buttonNeutral="variant-soft" buttonPositive="variant-soft-primary"/>
<Toast/>
<AppShell class="h-full">
  <svelte:fragment slot="header">
    <div class="grid grid-flow-col grid-cols-12 bg-surface-200-700-token px-3">
      <div class="col-span-3"/>
      <div class="col-span-6 flex flex-row justify-center">
        {#each MenuItems as item, index (index)}
          <a
            class:bg-initial={true}
            class:bg-primary-active-token={activePage === index}
            class="btn bg-surface-token hover:bg-primary-hover-token rounded-none sm:w-36 w-fit"
            data-sveltekit-preload-data="hover"
            href={item.target}
            on:click={() => {activePage = index}}
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
      <div class="col-span-3 flex flex-row items-center justify-end">

      </div>
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

