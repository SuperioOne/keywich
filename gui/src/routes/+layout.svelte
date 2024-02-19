<script lang="ts">
  import "../app.css";
  import "../app.postcss";
  import HomeIcon from "$lib/icons/home.svelte";
  import KeyIcon from "$lib/icons/key.svelte";
  import SettingsIcon from "$lib/icons/settings.svelte";
  import type {ComponentType} from "svelte";
  import type {LayoutData, LayoutRouteId} from "./$types";
  import {AppShell, Modal, storePopup, Toast, initializeStores} from "@skeletonlabs/skeleton";
  import {LogPanel, i18nStore, logPanelStore} from "$lib";
  import {arrow, autoUpdate, computePosition, flip, offset, shift} from "@floating-ui/dom";

  type NavItem = {
    label: string;
    target?: LayoutRouteId;
    icon?: ComponentType;
  }

  $: nav_items = [
    {
      label: $i18nStore.get_key("i18:/nav/home", "Home"),
      target: "/",
      icon: HomeIcon,
    },
    {
      label: $i18nStore.get_key("i18:/nav/keys", "Keys"),
      target: "/keys",
      icon: KeyIcon,
    },
    {
      label: $i18nStore.get_key("i18:/nav/settings", "Settings"),
      target: "/settings",
      icon: SettingsIcon,
    },
  ] as NavItem[];

  export let data: LayoutData;

  initializeStores();
  storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});

  function on_log_panel_close() {
    logPanelStore.close();
  }

  function on_log_panel_flip(event: KeyboardEvent) {
    if (event.code === "KeyI" && event.ctrlKey) {
      logPanelStore.flip();
    }
  }
</script>

<svelte:window on:keyup|preventDefault={on_log_panel_flip}/>
<Modal zIndex="z-[998]" buttonNeutral="variant-soft" buttonPositive="variant-filled-primary"/>
<Toast zIndex="z-[999]"/>
<AppShell class="h-full">
  <svelte:fragment slot="header">
    <div class="grid grid-flow-col grid-cols-12 bg-surface-200-700-token px-3 shadow">
      <div class="col-span-3"/>
      <div class="col-span-6 flex flex-row justify-center">
        {#each nav_items as item, index (index)}
          <a
              class:bg-initial={true}
              class:bg-primary-active-token={data.route_id === item.target}
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
    <div class="p-3 sm:p-6 md:p-12 w-full max-w-screen-lg ">
      <slot/>
    </div>
  </div>
  <svelte:fragment slot="footer">
    {#if $logPanelStore}
      <LogPanel on:close={on_log_panel_close}/>
    {/if}
  </svelte:fragment>
</AppShell>
