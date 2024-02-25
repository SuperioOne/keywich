<script lang="ts">
  import HomeIcon from "$lib/icons/home.svelte";
  import LockIcon from "$lib/icons/lock.svelte";
  import KeyIcon from "$lib/icons/key.svelte";
  import SettingsIcon from "$lib/icons/settings.svelte";
  import type {LayoutData} from "./$types";
  import {AppShell} from "@skeletonlabs/skeleton";
  import {LogPanel, i18nStore, logPanelStore, RPC, Log, getToastStore} from "$lib";
  import {is_error_response} from "@keywich/api/utils";
  import {goto} from "$app/navigation";

  const toast_store = getToastStore();
  const nav_items = [
    {
      label_key: "i18:/nav/home",
      default_label: "Home",
      target: "/",
      target_id: "/(app)",
      icon: HomeIcon,
    },
    {
      label_key: "i18:/nav/keys",
      default_label: "Keys",
      target: "/keys",
      target_id: "/(app)/keys",
      icon: KeyIcon,
    },
    {
      label_key: "i18:/nav/settings",
      default_label: "Settings",
      target: "/settings",
      target_id: "/(app)/settings",
      icon: SettingsIcon,
    },
  ];

  export let data: LayoutData;

  function on_log_panel_close() {
    logPanelStore.close();
  }

  function on_log_panel_flip(event: KeyboardEvent) {
    if (event.code === "KeyI" && event.ctrlKey) {
      logPanelStore.flip();
    }
  }

  async function on_lock() {
    try {
      await RPC.logout()
    } catch (err) {
      Log.error(err);

      if (is_error_response(err)) {
        toast_store.trigger_error($i18nStore.get_key(`i18:/errors/${err.code}`, err.message));
      }
    } finally {
      sessionStorage.removeItem("unlocked");
      await goto("unlock");
    }
  }
</script>

<svelte:window on:keyup|preventDefault={on_log_panel_flip}/>
<AppShell class="h-full">
  <svelte:fragment slot="header">
    <div class="grid grid-flow-col grid-cols-12 bg-surface-200-700-token px-3 shadow">
      <div class="col-span-3"/>
      <div class="col-span-6 flex flex-row justify-center">
        {#each nav_items as item, index (index)}
          <a
              class:bg-primary-active-token={data.route_id === item.target_id}
              class="btn bg-initial bg-surface-token hover:bg-primary-hover-token rounded-none sm:w-36 flex flex-col gap-1 align-middle justify-center w-fit"
              data-sveltekit-preload-data="tap"
              href={item.target}
          >
            <svelte:component this={item.icon}/>
            <span class="!p-0 !m-0 hidden sm:inline-block font-bold">
              {$i18nStore.get_key(item.label_key, item.default_label)}
            </span>
          </a>
        {/each}
      </div>
      <div class="col-span-3 flex flex-row items-center justify-end">
        <button
            class="btn bg-initial bg-surface-token hover:bg-warning-hover-token rounded-none sm:w-36 flex flex-col gap-1 align-middle justify-center w-fit"
            data-sveltekit-preload-data="tap"
            on:click={on_lock}
        >
          <LockIcon/>
          <span class="!p-0 !m-0 hidden sm:inline-block font-bold">
             {$i18nStore.get_key("i18:/nav/lock", "Lock")}
          </span>
        </button>
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
