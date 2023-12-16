<script lang="ts">
  import "../app.css";
  import "../app.postcss";
  import RPC from "@keywitch/memory_rpc";
  import type {LayoutData} from "../../.svelte-kit/types/src/routes/$types";
  import {
    AppShell,
    storePopup,
    initializeStores,
    Modal,
    Toast,
    setInitialClassState, getModeAutoPrefers,
  } from "@skeletonlabs/skeleton";
  import {computePosition, autoUpdate, flip, shift, offset, arrow} from "@floating-ui/dom";
  import {create_event_manager, i18nStore, theme_store, LogPanel, set_app_context} from "$lib";
  import HomeIcon from "$lib/icons/home.svelte";
  import KeyIcon from "$lib/icons/key.svelte";
  import SettingsIcon from "$lib/icons/settings.svelte";
  import type {MenuItem} from "./menu.config";

  export let data: LayoutData;

  initializeStores();
  const eventManager = create_event_manager();
  const menuItems: MenuItem[] = [
    {
      label: i18nStore.getKey("i18:/nav/home", "Home"),
      target: "/",
      icon: HomeIcon,
    },
    {
      label: i18nStore.getKey("i18:/nav/keys", "Keys"),
      target: "/keys",
      icon: KeyIcon,
    },
    {
      label: i18nStore.getKey("i18:/nav/settings", "Settings"),
      target: "/settings",
      icon: SettingsIcon,
    },
  ];
  storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});
  theme_store.set({
    name: "crimson",
    isLight: getModeAutoPrefers()
  });

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


<svelte:head>{@html `<script>(${setInitialClassState.toString()})();</script>`}</svelte:head>
<svelte:window on:keyup|preventDefault={on_log_panel_flip}/>
<Modal buttonNeutral="variant-soft" buttonPositive="variant-soft-primary"/>
<Toast/>
<AppShell class="h-full">
  <svelte:fragment slot="header">
    <div class="grid grid-flow-col grid-cols-12 bg-surface-200-700-token px-3 shadow">
      <div class="col-span-3"/>
      <div class="col-span-6 flex flex-row justify-center">
        {#each menuItems as item, index (index)}
          <a
            class:bg-initial={true}
            class:bg-primary-active-token={data.routeId === item.target}
            class="btn bg-surface-token hover:bg-primary-hover-token rounded-none sm:w-36 w-fit"
            data-sveltekit-preload-data="tap"
            href={item.target}
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

