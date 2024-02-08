<script lang="ts">
  import "../app.css";
  import "../app.postcss";
  import HomeIcon from "$lib/icons/home.svelte";
  import KeyIcon from "$lib/icons/key.svelte";
  import SettingsIcon from "$lib/icons/settings.svelte";
  import type {LayoutData} from "./$types";
  import type {MenuItem} from "./menu.config";
  import {AppShell, Modal, Toast} from "@skeletonlabs/skeleton";
  import {LogPanel, App, i18nStore} from "$lib";

  export let data: LayoutData;
  let displayLogger: boolean = false;

  App.init();

  const menuItems: MenuItem[] = [
    {
      label: i18nStore.get_key("i18:/nav/home", "Home"),
      target: "/",
      icon: HomeIcon,
    },
    {
      label: i18nStore.get_key("i18:/nav/keys", "Keys"),
      target: "/keys",
      icon: KeyIcon,
    },
    {
      label: i18nStore.get_key("i18:/nav/settings", "Settings"),
      target: "/settings",
      icon: SettingsIcon,
    },
  ];

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
<Modal buttonNeutral="variant-soft" buttonPositive="variant-filled-primary"/>
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
    <div class="p-3 sm:p-6 md:p-12 w-full max-w-screen-lg ">
      <slot/>
    </div>
  </div>
  <svelte:fragment slot="footer">
    {#if displayLogger}
      <LogPanel on:close={on_log_panel_close}/>
    {/if}

  </svelte:fragment>
</AppShell>
